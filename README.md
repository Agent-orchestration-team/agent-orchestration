# Agent Workspace

Agent Workspace는 자연어로 요청한 업무를 **실행 전 계획, 단계별 작업, 검토 가능한 결과물**로 연결하는 캡스톤 디자인 프로젝트입니다. 사용자는 한 작업 공간에서 Workflow의 목적과 필요한 자료를 확인하고 실행 여부를 결정합니다. 현재 Windows 데스크톱 개발 앱에서 키 없이 데모 계획을 생성하고, 실행 결과를 Markdown 문서로 저장하는 흐름까지 검증했습니다.

[설치 및 첫 실행](#설치-및-첫-실행) · [사용 방법](#사용-방법) · [실제-ai-설정](#실제-ai-설정) · [테스트와 검증](#테스트와-검증)

## 주요 기능과 개발 상태

| 영역 | 상태 | 현재 사용할 수 있는 기능 |
|---|---|---|
| 입력 수신 확인 | UI 연동 완료 | 글자·숫자 포함 여부와 UTF-8 8,192바이트 제한을 검사합니다. UUID v4와 원문을 Rust에 전달하고 응답 ID·원문을 대조합니다. 요청별 수신·거부·오류·미확인 상태를 채팅 기록에 표시합니다. |
| Workspace | UI 연동 완료 | 탐색 레일과 접이식 사이드바로 페이지를 전환하고 Light/Dark 테마를 선택합니다. 화면 이동 중에도 입력 초안과 대화 기록을 유지합니다. |
| AI 계획·실행 | 데모 연동 검증 완료 | Runtime이 고정 예시로 계획을 생성·검증합니다. 사용자가 필수 자료를 채우고 실행하면 텍스트 작업을 순차 수행해 답변이나 검토 의견을 표시합니다. |
| 실제 AI 모드 | 연결 구현, 실계정 미검증 | 같은 화면에서 Provider로 계획 생성과 실행을 요청합니다. 설정·인증·호출 오류는 실패 상태로 전달합니다. |
| Markdown 출력 | UI 연동 완료 | 계획·답변·검토 의견에 제목, 목록, GFM 표와 코드를 적용합니다. 긴 본문은 줄바꿈하고 표·코드는 내부에서 스크롤합니다. 사용자 채팅 원문은 일반 텍스트로 표시합니다. |
| 실행 기록·결과물 | 부분 연동 | 실행 파일을 로컬에 저장하고 AI 화면에 ID와 경로를 표시합니다. 사이드바의 Workflows·Preview·Runs·Artifacts·Review는 예시 데이터로 탐색하는 화면입니다. |
| Runtime 확장 기능 | 내부 구현 / 데스크톱 UI 미연동 | 별도 CLI에 실행 재개·취소와 지속 실행 sidecar가 있습니다. 데스크톱 앱은 요청마다 Node 프로세스를 시작합니다. |

## 시스템 아키텍처

Home에서 선택한 탭에 따라 두 독립적인 IPC 명령이 동작합니다.

```text
입력 수신 확인
  React 입력 폼 → Tauri IPC: receive_user_input → Rust 입력 검증
  Rust 수신 응답 → React 응답 검증 → 요청별 채팅 기록

AI 계획·실행
  계획 생성 클릭 → Tauri IPC: agent_task(plan) → Rust → Node Runtime
    → 고정 데모 또는 AI Provider → Workflow 검증·계획 저장 → Markdown 미리보기
  사용자 확인·실행 클릭 → Tauri IPC: agent_task(run) → Rust → Node Runtime
    → 저장된 계획과 추가 자료 검사 → 순차 실행·최종 검토 → 실행 기록·결과물 저장
    → React에 답변, 검토 필요 상태 또는 오류 표시
```

Rust 브리지는 Node 자식 프로세스와 표준 입력·출력으로 JSON을 교환합니다. 동시에 하나의 AI 요청을 처리하며 프로세스 시간 제한, 응답 크기와 형식을 검사합니다. [통합 아키텍처](docs/architecture/integrated-architecture.md)

## 기술 스택

| 영역 | 기술 |
|---|---|
| Frontend | React 19.2, TypeScript 5.9, Vite 8 |
| 데스크톱 / Core | Tauri 2, Rust 1.99.1, Serde JSON |
| AI Runtime | Node.js 24, TypeScript, Agent Runtime 0.32.0, Zod 스키마 검증 |
| AI Provider | OpenAI 직접 연결, FactChat Gateway를 통한 OpenAI·Gemini·Claude 연결 |
| 문서 출력 | react-markdown, remark-gfm |
| 테스트 | Vitest, React Testing Library, Node 테스트 러너, Cargo 및 Tauri IPC 테스트 |
| 패키지 관리 | Frontend는 pnpm, Runtime은 npm, Rust는 Cargo. 각 lockfile을 독립적으로 사용합니다. |

## 프로젝트 구조

```text
Agent-Workspace-App/
├── src/
│   ├── main.tsx                React 진입점
│   ├── app/, layouts/, pages/  Home 모드, 탐색 및 페이지
│   ├── features/user-input/    입력 검증, Rust 수신 확인, 채팅
│   ├── features/agent-task/    계획 생성, 추가 자료 입력, 실행과 결과
│   └── components/markdown/    공통 Markdown 렌더러와 스타일
├── src-tauri/                 앱 설정, Rust 진입점 및 두 IPC 명령
├── agent-runtime/             생성기·실행기·Provider, 스키마, CLI 및 테스트
├── tests/                     Frontend 회귀·통합 테스트
├── scripts/dev-env.cmd        Windows 개발 명령 실행
└── docs/                      아키텍처, PoC 및 통합 검증 기록
```

## 필수 개발 환경

Windows x64에서는 Git, **Node.js 24.x**, **pnpm 10.34.9**, npm, rustup과 **Rust/Cargo 1.99.1 (MSVC)**, Visual Studio C++ 데스크톱 빌드 도구, Windows SDK, WebView2 Runtime이 필요합니다. npm 버전은 고정하지 않았으며 통합 검증에 사용한 버전은 11.21.3입니다. API 키는 실제 AI 모드에서만 필요합니다. 다른 운영체제에서의 데스크톱 통합 실행은 미검증입니다.

## 설치 및 첫 실행

1. PowerShell에서 아래 자리표시자를 팀 저장소의 Clone URL로 바꿔 실행합니다.

   ```powershell
   git clone "<협업 저장소 Clone URL>" Agent-Workspace-App
   cd Agent-Workspace-App
   ```

2. 프로젝트 루트에서 도구 버전을 확인합니다. 이후 명령도 같은 폴더에서 실행합니다.

   ```powershell
   .\scripts\dev-env.cmd node --version
   .\scripts\dev-env.cmd pnpm --version
   .\scripts\dev-env.cmd npm --version
   rustc --version
   cargo --version
   ```

   `dev-env.cmd`는 프로젝트 전용 Node가 있으면 우선 사용하고, 새 클론에서는 PATH의 Node가 24.x인지 확인합니다. Rust 버전은 루트의 `rust-toolchain.toml`을 따릅니다.

3. 두 lockfile로 의존성을 설치하고 앱을 시작합니다. 데모는 환경 파일 없이 실행할 수 있습니다.

   ```powershell
   .\scripts\dev-env.cmd pnpm install --frozen-lockfile --ignore-scripts
   .\scripts\dev-env.cmd npm --prefix agent-runtime ci --ignore-scripts
   .\scripts\dev-env.cmd pnpm tauri dev
   ```

   **Agent Workspace Integrated** 창이 열리면 Home에서 시작합니다. Tauri가 Vite 개발 서버(기본 포트 1420)도 함께 실행합니다.

## 사용 방법

1. **Home → 입력 수신 확인**에서 자연어를 입력하고 **전송**을 누릅니다. 사용자 말풍선과 Rust Core 수신 상태가 같은 요청에 연결됩니다.
2. **AI 계획·실행**을 선택합니다. **실행 모드**를 **데모 — API 키 없이 고정 응답**으로 두고 **요청**을 작성한 뒤 **계획 생성**을 누릅니다.
3. Markdown 계획에서 목적·실행 순서·필요한 자료를 확인합니다. 추가 설명을 요청하면 원래 요청을 보완해 계획을 다시 생성합니다. 필수 자료 입력란이 있으면 채웁니다.
4. **이 계획으로 실행**을 눌러 작업을 시작합니다. 완료되면 **답변**과 실행 ID가 표시됩니다. 검토가 필요하면 **실행 결과**에서 제공된 초안과 의견을 확인합니다.

계획은 `agent-runtime/.local/desktop-tasks/<taskId>/`의 `plan.md`와 `record.json`에서 확인할 수 있습니다. 실행마다 그 아래 `runs/<runId>/execution.json`이 저장됩니다. 완료 답변은 `answer.md`, 검토가 필요한 경우 제공된 초안과 의견은 `draft.md`, `review.md`에 저장됩니다.

## 실제 AI 설정

API 설정은 Node Runtime이 읽는 `agent-runtime/.env`에 둡니다. 예시 파일을 처음 한 번 복사한 뒤 계정에서 사용할 키와 모델 ID를 입력합니다.

```powershell
if (!(Test-Path agent-runtime/.env)) { Copy-Item agent-runtime/.env.example agent-runtime/.env }
notepad agent-runtime/.env
```

| 연결 방식 | 설정 |
|---|---|
| FactChat | `AI_BACKEND=factchat`, `FACTCHAT_API_KEY`, `WORKFLOW_PROVIDER`(`openai`, `gemini`, `claude` 중 하나). 기본 `FACTCHAT_BASE_URL`은 `https://factchat-cloud.mindlogic.ai/v1/gateway`입니다. |
| OpenAI 직접 연결 | `AI_BACKEND=direct`, `WORKFLOW_PROVIDER=openai`, `OPENAI_API_KEY`, `OPENAI_MODEL`을 설정합니다. |

FactChat에서는 키를 설정한 뒤 아래 `models` 명령으로 모델 ID를 조회해 선택한 Provider의 `OPENAI_MODEL`, `GEMINI_MODEL` 또는 `CLAUDE_MODEL`에 입력합니다. 데스크톱 경로에서 `WORKFLOW_MODEL`을 지정하면 계획 생성과 실행에 해당 모델을 우선 사용합니다.

```powershell
.\scripts\dev-env.cmd npm --prefix agent-runtime run models
.\scripts\dev-env.cmd npm --prefix agent-runtime run doctor
```

Home의 **AI 계획·실행 → 실행 모드 → 실제 AI**를 선택합니다. 계획 생성과 실행은 각각 API를 호출하므로 계정 사용량이 발생할 수 있습니다. [FactChat 설정 가이드](agent-runtime/docs/factchat.md)

## 테스트와 검증

```powershell
.\scripts\dev-env.cmd pnpm test
.\scripts\dev-env.cmd pnpm exec tsc --noEmit -p tsconfig.test.json
.\scripts\dev-env.cmd pnpm build
.\scripts\dev-env.cmd npm --prefix agent-runtime run check
.\scripts\dev-env.cmd npm --prefix agent-runtime test
.\scripts\dev-env.cmd cargo check --locked --manifest-path src-tauri/Cargo.toml
.\scripts\dev-env.cmd cargo test --locked --manifest-path src-tauri/Cargo.toml
```

**2026-11-25 Windows 통합 검증 기록**에서 Frontend는 292개 통과, Runtime은 266개 통과·1개 건너뜀(Windows CLI Ctrl+C 검사), Rust는 8개 통과했습니다. 실제 Tauri 창에서 데모 계획 생성, 사용자 실행, 결과 파일 저장과 Markdown 표시를 확인했습니다. 실제 AI 계정의 인증·응답 품질은 미검증입니다. [통합 검증 결과 및 수동 확인 절차](docs/integration/integration-test-results.md)

## 관련 문서

- [통합 설계 및 구현 내역](docs/integration/integration-report.md)
- [FE–Core API 초안](docs/poc/user-input/fe-core-api-contract.md) · [Rust 수신 검증](docs/poc/user-input/reception-test-results.md) · [채팅 출력 검증](docs/poc/user-input/chat-ui-test-results.md)
- [Workspace UI 검증](docs/poc/user-input/workspace-ui-test-results.md) · [Markdown 렌더링 검증](docs/poc/user-input/markdown-rendering-test-results.md)
- [AI Runtime 사용 안내](agent-runtime/README.md) · [Workflow 생성기](agent-runtime/docs/workflow-generator.md)

## 현재 제한과 후속 범위

데스크톱 실행기는 텍스트 작업을 순차 처리합니다. 웹 검색·외부 도구 실행·PDF 출력, 복수 Workflow 병렬 실행, UI에서의 실행 중 취소는 후속 범위입니다. 저장한 이력을 다시 탐색하는 UI, 스트리밍 응답, 배포용 Node 번들과 설치 패키지도 후속 대상입니다. 현재 브리지는 빌드 시점의 소스 경로에서 Runtime을 찾으므로 프로젝트를 이동한 후에는 새 위치에서 다시 빌드해야 합니다.
