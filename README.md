[English](#english) | [한국어](#korean) | [日本語](#japanese)

[Developer documentation](docs/DEVELOPMENT.md)

<a id="english"></a>
## English

# Server Status

A Rust HTTP API that reports host-wide CPU usage and operating-system uptime.

### Features

- `GET /api/status` returns `total_cpu_usage_percent`, `uptime_seconds`, and `collected_at`.
- CPU usage is sampled once per second. It is `null` until the first sample.
- `GET /api/health` returns `{"status":"ok"}`.
- The bind IP and port can be configured with `SERVER_STATUS_HOST` and `SERVER_STATUS_PORT`. Defaults are `0.0.0.0` and `3000`.

### Download

The private repository's [GitHub Releases](https://github.com/jyc8369/Server_Status/releases) page contains the Linux Debian package `server-status_0.1.0-1_amd64.deb`. Repository access is required to download it.

<a id="korean"></a>
## 한국어

# Server Status

호스트 전체 CPU 사용률과 운영체제 uptime을 제공하는 Rust HTTP API입니다.

### 기능

- `GET /api/status`에서 `total_cpu_usage_percent`, `uptime_seconds`, `collected_at`을 반환합니다.
- CPU 사용률은 1초마다 수집하며, 첫 측정 전에는 `null`입니다.
- `GET /api/health`에서 `{"status":"ok"}`를 반환합니다.
- 바인딩 IP와 포트는 `SERVER_STATUS_HOST`, `SERVER_STATUS_PORT`로 설정할 수 있습니다. 기본값은 각각 `0.0.0.0`, `3000`입니다.

### 다운로드

비공개 저장소의 [GitHub Releases](https://github.com/jyc8369/Server_Status/releases)에 Linux Debian 패키지 `server-status_0.1.0-1_amd64.deb`가 있습니다. 다운로드하려면 저장소 접근 권한이 필요합니다.

<a id="japanese"></a>
## 日本語

# Server Status

ホスト全体のCPU使用率とOSの稼働時間を提供するRust製HTTP APIです。

### 機能

- `GET /api/status`は`total_cpu_usage_percent`、`uptime_seconds`、`collected_at`を返します。
- CPU使用率は1秒ごとに収集され、最初の測定までは`null`です。
- `GET /api/health`は`{"status":"ok"}`を返します。
- バインドするIPアドレスとポートは`SERVER_STATUS_HOST`と`SERVER_STATUS_PORT`で設定できます。既定値はそれぞれ`0.0.0.0`と`3000`です。

### ダウンロード

非公開リポジトリの[GitHub Releases](https://github.com/jyc8369/Server_Status/releases)にLinux Debianパッケージ`server-status_0.1.0-1_amd64.deb`があります。ダウンロードにはリポジトリへのアクセス権が必要です。
