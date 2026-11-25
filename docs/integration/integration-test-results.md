# 통합 검증 결과

2026-11-25, Windows x64, 신규 바탕화면 프로젝트. Node 24.22.3과 npm 11.21.3을 프로젝트 내부에 두고 pnpm 10.34.9, Rust 1.99.1 MSVC를 사용했다. 아래 수치는 이 프로젝트에서 실행한 결과다.

| 검사 (루트에서 scripts/dev-env.cmd 사용) | 결과 |
|---|---|
| pnpm install --frozen-lockfile --ignore-scripts | FE lock 설치 완료 |
| npm --prefix agent-runtime ci --ignore-scripts | Runtime lock 설치 완료. 초기 시스템 npm이 Node 20으로 실행되어 Node 24 전용 로컬 npm으로 재설치 |
| pnpm test --maxWorkers=4 | 13파일 292 통과, 실패 0 |
| pnpm exec tsc --noEmit -p tsconfig.test.json 및 pnpm build | 타입 검사 및 Vite 배포 빌드 통과 |
| npm --prefix agent-runtime run check | 타입 검사 통과 |
| npm --prefix agent-runtime test | 267개 중 266 통과, 실패 0, Windows CLI Ctrl+C 시그널 테스트 1개 skip |
| cargo check 및 cargo test (--locked --manifest-path src-tauri/Cargo.toml) | 컴파일 성공. Rust 단위 7, Tauri IPC 테스트 1 통과 |
| Windows Tauri 개발 앱 | 실제 WebView에서 Core 수신, AI 계획 생성과 별도 실행, Markdown·파일 저장 확인 |

## 기능별 판정

| ID | 실제 근거 | 결과 |
|---|---|---|
| REG-01~04 | FE 입력·유효성·원문·UUID 회귀 검사 | 통과 |
| REG-05~07 | Rust 명령 수신 및 FE 응답 검증, Windows 앱에서 앞뒤 공백을 포함한 한글·영문·숫자 원문 확인 | 통과 |
| REG-08~10 | FE IPC 오류/타임아웃·다중 대화·페이지 탐색 검사. 실제 앱에서 모드 전환 후 채팅 확인 | 통과 |
| AI-01~03 | Windows Tauri → 등록된 Rust 명령 → Node 자식 → JSON 응답 | 통과 |
| AI-04~06 | 데모 계획 생성과 Workflow 검증, 계획 Markdown과 표 렌더링 | 통과 |
| AI-07 | 필수 자료 누락 시 버튼 비활성, Runtime MISSING_INPUT 검사 | 통과 |
| AI-08~10 | 계획 후 runs 폴더가 없는 상태 확인 → 실행 버튼 클릭 → completed 및 execution.json, answer.md 확인 | 통과 |
| AI-11 | 빈 설정의 live 계획이 CONFIG_INVALID 반환. 모의 네트워크 오류는 Runtime 테스트로 검증 | 통과 |
| AI-12 | 두 agent_task 동시 호출 중 하나만 draft, 다른 하나는 실행 중 오류 | 통과 |
| AI-13 | 입력을 읽지 않는 자식과 250KB 입력으로 짧은 제한시간 종료 검사 | 통과 |
| AI-14 | 잘못된 JSON·다른 taskId·8MB 초과 출력 거부 | 통과 |
| AI-15 | 실제 Provider/FactChat 인증 계정으로 모델 호출 | 미검증. 실사용 키와 호출 승인 없음 |
| UI-01~05 | 기존 레이아웃 회귀 검사, 실제 계획의 제목·목록·표 및 최종 답변 Markdown 확인 | 통과 |
| UI-06~08 | 실제 WebView의 1440px/640px 측정. 640px 패널에 검사용 2,400자 본문·긴 코드·12열 표를 렌더링하고 표/코드만 내부 스크롤되는지 확인 | 통과 |
| UI-09~10 | 예시 모드 표기·실제 입력 원문과 수신 상태·탭 전환 회귀 검사 | 통과 |

## Windows에서 발견한 문제

첫 데모 실행은 실행 기록을 교체하는 rename에서 EPERM이 발생해 답변을 저장하지 못했다. 같은 경로에서 오류를 재현한 후 일시적 Windows 잠금 오류에만 제한된 재시도를 적용했다. 실제 앱에서 다시 계획 생성·실행을 수행해 완료 답변을 읽었다. 최종 확인한 계획은 5f7f575d-bfe4-4934-b896-8f5e5b3a9658, 실행은 62f3b94c-008d-4ac1-800e-d1a96f28c781이다. .cache/demo-640.png와 demo-1440.png에 화면을 남겼다. E2E는 로컬 디버깅이 가능한 별도 포트의 실제 개발 창에서 수행했다.

## 직접 확인

README 실행 명령으로 앱을 연다. Home에서 입력 수신 확인을 한 뒤 AI 계획·실행에서 데모 계획을 만든다. 계획 본문을 확인한 다음 실행 버튼을 눌러 실행 ID, 완료 상태와 답변을 확인한다. 화면에 표시되는 실행 폴더에서 execution.json의 id, status와 answer.md 내용을 대조할 수 있다. Live 검증은 별도 인증 정보를 설정한 뒤 동일한 두 단계로 진행한다.

개발용 데모 흐름과 기존 입력 PoC는 통합 검증을 마쳤다. 실제 AI 품질·인증, 배포 설치본 및 저장 이력 재조회 UI는 이 판정의 범위 밖이다.

