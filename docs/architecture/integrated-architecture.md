# 데스크톱 통합 구조

```text
Home / 입력 수신 확인 → UserInputRequestForm → receive_user_input → Rust reception
                    ← 응답 스키마·ID·원문 검증 ← 수신 결과 → 요청별 채팅
Home / AI 계획·실행 → AgentTaskPanel → agent_task → Node 24 desktop-cli
  plan → desktopTask → planTask → Demo 또는 Provider → Workflow 검증 → 계획 저장
       ← MarkdownContent + 필요 자료 + taskId
  run  → 사용자 클릭 → taskId + inputs → 저장된 모드·계획 → 준비 검사
       → 순차 텍스트 작업 → 최종 검토 → 체크포인트/결과물 저장
       ← 완료 답변 또는 초안·검토·오류 + runId
```

Rust는 한 번에 AI 요청 하나를 받는다. 블로킹 작업 풀에서 자식 프로세스를 띄우고 stdin/stdout으로 JSON 한 건을 교환한다. 입력 제한은 1 MB, 출력 제한은 8 MB다. 입력 쓰기와 출력 읽기를 분리하여 자식이 입력을 읽지 않는 경우에도 345초 제한이 적용된다. Runtime에는 300초 실행 제한과 330초 watchdog이 있다. 종료 시 프로세스를 회수하고 성공 응답의 작업 종류·상태·ID 및 렌더링에 필요한 필드 형태를 검사한다.

Node는 .env를 읽어 FactChat 또는 직접 Provider를 선택한다. 기술 오류는 일반 텍스트로 표시하고 문서형 콘텐츠는 MarkdownContent로 표시한다. 데스크톱 실행 경로는 텍스트 전용이며 기존 Runtime의 별도 CLI 도구 기능은 노출하지 않는다.

Runtime은 .local/desktop-tasks 아래에 계획·모드·실행 기록을 분리해 저장한다. 임시 파일을 같은 폴더에 쓴 뒤 rename하며 Windows의 일시적 잠금만 제한적으로 재시도한다. 화면 상태는 메모리에 있고 파일은 앱 종료 후에도 남는다. 향후 조회 인터페이스, 번들링 및 취소 기능을 추가할 수 있다.

