[English](#english) | [한국어](#korean) | [日本語](#japanese)

[Developer documentation](docs/DEVELOPMENT.md)

<a id="english"></a>
## English

# Server Status

A Rust HTTP API that reports host-wide CPU usage and operating-system uptime.

### Features

- `GET /api/status` returns `total_cpu_usage_percent`, `uptime_seconds`, and `collected_at`.
- CPU usage is sampled once per second. It is `null` until the first sample.
- `GET /api/health` returns HTTP 200 with `{"status":"ok"}` while the HTTP service is running (liveness).
- `GET /api/ready` returns HTTP 200 with `{"status":"ready"}` only when a CPU sample was collected within the past five seconds. Otherwise, it returns HTTP 503 with `{"status":"not_ready"}` (readiness).
- The bind IP and port are configurable with `SERVER_STATUS_HOST` and `SERVER_STATUS_PORT`. Defaults are `127.0.0.1` and `3000`. External access must be enabled deliberately and protected with appropriate network controls.

### Run

```bash
cargo run
curl http://127.0.0.1:3000/api/status
curl -i http://127.0.0.1:3000/api/ready
```

### Download

Download the Linux Debian package from [GitHub Releases](https://github.com/jyc8369/Server_Status/releases). The updated .deb packaging installs and enables a `server-status.service` systemd unit. It binds to loopback by default.

<a id="korean"></a>
## 한국어

# Server Status

호스트 전체 CPU 사용률과 운영체제 uptime을 제공하는 Rust HTTP API입니다.

### 기능

- `GET /api/status`에서 `total_cpu_usage_percent`, `uptime_seconds`, `collected_at`을 반환합니다.
- CPU 사용률은 1초마다 수집하며, 첫 측정 전에는 `null`입니다.
- `GET /api/health`는 HTTP 서비스의 생존 여부를 확인하며, 실행 중이면 HTTP 200과 `{"status":"ok"}`를 반환합니다.
- `GET /api/ready`는 최근 5초 이내에 CPU 샘플이 수집된 경우에만 HTTP 200과 `{"status":"ready"}`를 반환합니다. 그 외에는 HTTP 503과 `{"status":"not_ready"}`를 반환합니다.
- 바인딩 IP와 포트는 `SERVER_STATUS_HOST`, `SERVER_STATUS_PORT`로 설정합니다. 기본값은 `127.0.0.1`, `3000`입니다. 외부 공개 시 별도 접근 제어가 필요합니다.

### 실행

```bash
cargo run
curl http://127.0.0.1:3000/api/status
curl -i http://127.0.0.1:3000/api/ready
```

### 다운로드

[GitHub Releases](https://github.com/jyc8369/Server_Status/releases)에서 Linux Debian 패키지를 받을 수 있습니다. 변경된 .deb 패키지를 설치하면 `server-status.service` systemd 유닛이 설치되고 활성화되며, 기본적으로 localhost에만 바인딩됩니다.

<a id="japanese"></a>
## 日本語

# Server Status

ホスト全体のCPU使用率とOSの稼働時間を提供するRust製HTTP APIです。

### 機能

- `GET /api/status`は`total_cpu_usage_percent`、`uptime_seconds`、`collected_at`を返します。
- CPU使用率は1秒ごとに収集され、最初の測定までは`null`です。
- `GET /api/health`はHTTPサービスの稼働状態を示し、稼働中はHTTP 200と`{"status":"ok"}`を返します。
- `GET /api/ready`はCPUサンプルが過去5秒以内に収集された場合のみHTTP 200と`{"status":"ready"}`を返し、それ以外はHTTP 503と`{"status":"not_ready"}`を返します。
- バインドするIPアドレスとポートは`SERVER_STATUS_HOST`と`SERVER_STATUS_PORT`で設定できます。既定値は`127.0.0.1`と`3000`です。外部公開時には適切なアクセス制御が必要です。

### 実行

```bash
cargo run
curl http://127.0.0.1:3000/api/status
curl -i http://127.0.0.1:3000/api/ready
```

### ダウンロード

Linux用Debianパッケージは[GitHub Releases](https://github.com/jyc8369/Server_Status/releases)から入手できます。更新後の.debパッケージは`server-status.service`をインストールして有効化します。既定ではlocalhostにのみバインドします。
