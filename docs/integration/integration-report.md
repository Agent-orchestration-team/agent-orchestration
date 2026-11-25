# 통합 구현

프로젝트 A의 추적 파일을 기반으로 User Input PoC, Workspace 페이지와 테마, Markdown, 기존 문서를 유지했다. 원본 파일 매핑은 아래와 같다. B 경로는 demos/Agent-Workspace-Integrated 기준이다.

| 원본 | 신규 프로젝트의 동일 상대 경로 | 조정 및 목적 |
|---|---|---|
| A + B src/app/App.tsx | src/app/App.tsx | Home에서 수신 확인과 계획·실행을 선택. 화면 전환에도 양쪽 상태 보존 |
| B src/features/agent-task/AgentTaskPanel.tsx | src/features/agent-task/AgentTaskPanel.tsx | 공통 Markdown으로 계획·답변·초안·검토 렌더링, 작업/실행 ID 표시 |
| A + B src/styles/global.css | src/styles/global.css | 패널·탭 스타일 추가, 문서 grid 최소 너비 제한 |
| B agent-runtime/ | agent-runtime/ | 스키마·생성·실행·Provider·CLI·테스트·문서 및 npm lock 선별 이식 |
| B src/task/desktop{,-cli}.ts (Runtime 내부) | agent-runtime/src/task/ | UTF-8 스트림 디코딩, 실행 기록 ID로 폴더·응답 연결, 저장 오류 안내 |
| B Runtime src/execution/store.ts, run.ts | agent-runtime/src/execution/ | Windows 파일 교체 시 일시적 잠금 재시도, 내부 실패 원인 보존 |
| B Runtime tests/desktop.test.ts | agent-runtime/tests/desktop.test.ts | 응답 runId와 execution.json ID 일치 확인 |
| B src-tauri/src/agent_task.rs | src-tauri/src/agent_task.rs | 백그라운드 프로세스, 입력 쓰기 포함 시간 제한, 출력 제한·종료 상태·응답 필드 검증, 브리지 테스트 |
| A src-tauri/src/lib.rs | src-tauri/src/lib.rs | 기존 receive_user_input과 agent_task 명령 함께 등록 |
| B tests/agent-task.test.tsx | tests/agent-task.test.tsx | 실행 버튼, 필수 자료, 실패/브라우저 상태 검증 |
| A/B package.json, .nvmrc | package.json, .nvmrc | FE 패키지 버전 유지, Node 24 요구 및 Runtime 설치/검사 명령 |
| A scripts/dev-env.cmd, .gitignore | 동일 | 프로젝트 내부 또는 PATH의 Node 24 선택, 실행 기록 제외 |
| 신규 문서 | README.md, docs/integration/*.md, docs/architecture/integrated-architecture.md | 설치·사용·테스트 및 설계 판단 |

기존 API v0.2와 Rust 수신·검증 로직은 그대로 사용한다. 데모와 실사용 모드는 저장된 계획에 귀속된다. 실제 AI 설정 오류는 오류로 반환한다. 계획 생성 후에만 명시적 실행이 가능하며, 저장된 실행 결과에는 해당 계획·실행 ID가 연결된다. 자식 프로세스 단위의 개발 통합이며 배포용 Node 번들, 상시 sidecar, 취소 및 이력 탐색은 후속 범위다.


추가 조정: src/pages/WorkspacePages.tsx 및 WorkflowPreview.tsx는 예시 화면에서 실제 실행 탭의 위치를 안내한다. src-tauri/tauri.conf.json의 앱 식별자와 창 제목은 독립 앱용으로 지정했다. vite.config.ts는 로컬 실행 기록·QA 캐시·도구 폴더의 HTTP 접근을 제한하고 개발 파일 감시에서 제외한다.
