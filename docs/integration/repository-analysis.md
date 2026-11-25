# 원본 분석 (2026-11-25)

| 원본 | 기본 브랜치 / 분석 SHA | 구성 |
|---|---|---|
| https://github.com/jtk745858/Agent-Workspace_User-Input | main / e064a697fd11cb2e824f4833cfcf9e949583e93f | React 19, TS 5.9, pnpm 10.34.9, Tauri 2, Rust 1.99.1, Node 22 |
| https://github.com/woo-seok-nam/agent-workspace | main / 6a4a025754fa39695b9f8d5cb17af1aa2fb68c69 | demos/Agent-Workspace-Integrated와 npm 기반 Node 24 Agent Runtime 0.32.0 |

두 원격의 HEAD와 main을 조회하고 임시 저장소에 소스를 가져왔다. Git archive로 A의 추적 파일을 내보내고 B의 하위 트리에서 Runtime, 브리지, 패널, 패널 테스트만 선별했다. B 최상위의 .env, backend/dev.db, storage 및 별도 웹 앱은 가져오지 않았다.

A의 src/main.tsx → App → AppLayout/MainPage/WorkspacePages 경로는 화면 상태 전환과 채팅 보존을 담당한다. receive_user_input은 요청 검증과 수신 확인을 담당한다. A의 기존 정적 Preview·Runs·Artifacts 및 MarkdownContent를 기준으로 삼았다. A의 Vitest/RTL과 Rust 수신 단위·IPC 테스트를 유지했다.

B의 README-INTEGRATED.md 기준 A 기반 SHA는 e064a697fd11cb2e824f4833cfcf9e949583e93f로 이번 A HEAD와 일치했다. 기존 통합본의 VALIDATION.md는 Linux에서 실행한 기록이다. 이번 Windows 검증 결과는 별도 문서에 기록했다.

B의 desktop-cli는 stdin 요청을 desktopTask로 보내고, 생성기·스키마·준비 검사·순차 실행기·모델 어댑터·리포트 저장 기능을 조합한다. CLI/sidecar 및 SQLite 기반 workflow CLI도 있지만 데스크톱 계획 저장소는 별도 파일 구조이다. backend는 별도 Prisma 프로젝트이며 데스크톱 명령의 의존성이 아니다. 데스크톱 동작은 두 명령(plan/run)이어서 기존 CLI의 SQLite 목록을 데스크톱 실행 이력으로 사용할 수 없다.

FE의 pnpm lock과 Runtime의 npm lock을 분리하고 Node 24에서 함께 검증했다. B 패널에 독립 Markdown 래퍼가 있어서 A의 공통 Renderer로 대체했다. 계획·결과·추가자료가 같은 상태를 사용하므로 Home 내 별도 탭에 패널을 유지한다. 다음 단계는 저장된 데스크톱 기록에 대한 제한된 조회 명령과 기존 목록 화면 연동이다.

