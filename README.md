# Server Status

현재 실행 중인 운영체제의 **전체 CPU 사용률**을 수집해 HTTP API로 제공하는 Rust 백엔드입니다. 코어별 사용률은 노출하지 않습니다.

## 실행

```powershell
cargo run
```

기본 바인딩 주소는 `0.0.0.0:3000`이며 `SERVER_STATUS_HOST`와 `SERVER_STATUS_PORT` 환경 변수로 바꿀 수 있습니다. `SERVER_STATUS_HOST`에는 IP 주소를 지정합니다.

```powershell
$env:SERVER_STATUS_HOST = "127.0.0.1"
$env:SERVER_STATUS_PORT = "8080"
cargo run
```

## API

- `GET /api/status`: 전체 CPU 사용률, OS uptime(초), 수집 시각을 반환합니다.
- `GET /api/health`: 서버 프로세스 상태를 반환합니다.

CPU 사용률은 `sysinfo`의 호스트 전체 값을 이용해 논리 CPU 전체를 기준으로 0~100%로 정규화하며, 1초마다 갱신합니다. 첫 측정 전에는 사용률이 `null`입니다.

## Linux `.deb` 릴리즈

`v` 접두사를 붙여 Cargo 버전과 일치하는 Git 태그를 푸시하면 GitHub Actions가 Linux `.deb` 패키지를 빌드해 해당 GitHub Release에 첨부합니다. 예: `v0.1.0`.
