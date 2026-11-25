use serde_json::Value;
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

static BUSY: AtomicBool = AtomicBool::new(false);
const OUTPUT_LIMIT: u64 = 8_000_000;
struct BusyGuard;
impl Drop for BusyGuard {
    fn drop(&mut self) {
        BUSY.store(false, Ordering::SeqCst);
    }
}

#[tauri::command]
pub async fn agent_task(payload: Value) -> Result<Value, String> {
    BUSY.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .map_err(|_| "이미 AI 작업이 실행 중입니다.")?;
    let guard = BusyGuard;
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../agent-runtime");
        let node = std::env::var_os("AGENT_NODE_PATH").unwrap_or_else(|| "node".into());
        let mut command = Command::new(node);
        command.current_dir(root).args([
            "--env-file-if-exists=.env",
            "--import",
            "tsx",
            "src/task/desktop-cli.ts",
        ]);
        exchange(command, &payload, Duration::from_secs(345))
    })
    .await
    .map_err(|_| "Runtime 작업 처리 실패")?
}

fn exchange(mut command: Command, payload: &Value, timeout: Duration) -> Result<Value, String> {
    let body = serde_json::to_vec(payload).map_err(|_| "요청 형식 오류")?;
    if body.len() > 1_000_000 {
        return Err("입력 자료 크기 초과".into());
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "Node 24 설치와 AGENT_NODE_PATH를 확인하세요.")?;
    let mut stdin = child.stdin.take().expect("piped stdin");
    let stdout = child.stdout.take().expect("piped stdout");
    let deadline = Instant::now() + timeout;
    let writer = thread::spawn(move || stdin.write_all(&body));
    let reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.take(OUTPUT_LIMIT + 1).read_to_end(&mut bytes)?;
        Ok::<_, std::io::Error>(bytes)
    });
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(25)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                break Err("Runtime 시간 초과 또는 프로세스 오류");
            }
        }
    };
    let written = writer.join();
    let output = reader.join();
    let status = status?;
    written
        .map_err(|_| "Runtime 입력 오류")?
        .map_err(|_| "Runtime 입력 전달 실패")?;
    let bytes = output
        .map_err(|_| "Runtime 출력 오류")?
        .map_err(|_| "Runtime 출력 읽기 실패")?;
    if bytes.len() as u64 > OUTPUT_LIMIT {
        return Err("Runtime 응답 크기 초과".into());
    }
    let reply: Value = serde_json::from_slice(&bytes).map_err(|_| "Runtime JSON 응답 오류")?;
    validate_reply(payload, &reply, status.success())?;
    Ok(reply)
}

fn validate_reply(request: &Value, reply: &Value, success: bool) -> Result<(), &'static str> {
    let invalid = "Runtime 응답 형식 오류";
    let status = reply["status"].as_str().ok_or(invalid)?;
    for key in [
        "taskId",
        "runId",
        "mode",
        "markdown",
        "message",
        "error",
        "draft",
        "review",
        "recordPath",
    ] {
        if !reply[key].is_null() && !reply[key].is_string() {
            return Err(invalid);
        }
    }
    if let Some(inputs) = reply.get("inputs") {
        for input in inputs.as_array().ok_or(invalid)? {
            if !(input["name"].is_string()
                && input["description"].is_string()
                && input["required"].is_boolean())
            {
                return Err(invalid);
            }
        }
    }
    if let Some(issues) = reply.get("issues") {
        if !issues
            .as_array()
            .ok_or(invalid)?
            .iter()
            .all(|issue| issue["detail"].is_string())
        {
            return Err(invalid);
        }
    }
    if status == "failed" && reply["error"].is_string() {
        return Ok(());
    }
    if !success || reply["taskId"].as_str().is_none_or(str::is_empty) {
        return Err(invalid);
    }
    match request["action"].as_str() {
        Some("plan")
            if matches!(status, "draft" | "needs_clarification")
                && reply["mode"] == request["mode"]
                && reply["markdown"].is_string()
                && reply["inputs"].is_array() =>
        {
            Ok(())
        }
        Some("run")
            if matches!(status, "completed" | "needs_review" | "cancelled")
                && reply["taskId"] == request["taskId"]
                && reply["markdown"].is_string() =>
        {
            Ok(())
        }
        _ => Err(invalid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn node(script: &str) -> Command {
        let mut cmd =
            Command::new(std::env::var_os("AGENT_NODE_PATH").unwrap_or_else(|| "node".into()));
        cmd.args(["-e", script]);
        cmd
    }
    #[test]
    fn unread_stdin_is_covered_by_deadline() {
        let result = exchange(
            node("setInterval(()=>{},1000)"),
            &json!("a".repeat(250000)),
            Duration::from_millis(200),
        );
        assert!(result.unwrap_err().contains("시간 초과"));
    }
    #[test]
    fn rejects_invalid_replies_and_large_output() {
        let req = json!({"action":"run","taskId":"a"});
        assert!(
            validate_reply(
                &req,
                &json!({"status":"completed","taskId":"b","markdown":"x"}),
                true
            )
            .is_err()
        );
        for script in [
            "process.stdout.write('bad')",
            "process.stdout.write('a'.repeat(8000010))",
        ] {
            assert!(exchange(node(script), &req, Duration::from_secs(15)).is_err());
        }
    }
}
