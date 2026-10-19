# Development

## Project

This repository contains a Rust HTTP API. The application entry point is `src/main.rs`; package metadata and dependencies are in `Cargo.toml` and `Cargo.lock`.

## Run locally

```powershell
cargo run
```

The server reads `SERVER_STATUS_HOST` and `SERVER_STATUS_PORT`. Their defaults are `127.0.0.1` and `3000`. For example, in PowerShell:

```powershell
$env:SERVER_STATUS_HOST = "127.0.0.1"
$env:SERVER_STATUS_PORT = "8080"
cargo run
```

Use `SERVER_STATUS_HOST=0.0.0.0` only when remote network access is necessary and the port is appropriately secured.

## API and health semantics

- `GET /api/status`: host-wide CPU percentage and OS uptime. CPU usage remains `null` until the first sample.
- `GET /api/health`: always HTTP 200 while the HTTP server is running. This is a **liveness** endpoint.
- `GET /api/ready`: HTTP 200 only while the last CPU sample is no more than five seconds old. Before the first sample, or after collection stops, it returns HTTP 503. This is a **readiness** endpoint.

Readiness checks use a monotonic clock; `collected_at` remains a UTC wall-clock timestamp.

## Checks

Before opening a PR, run:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

The `.github/workflows/ci.yml` workflow runs the same checks on PRs and `main` pushes.

## Debian release and systemd

The GitHub Actions workflow at `.github/workflows/release-deb.yml` runs on a `v*` tag. The tag version must match `Cargo.toml`. It validates source, builds the Debian package on Ubuntu 24.04, checks for the systemd service file, and attaches the package to the GitHub Release.

The workflow's packaging commands are:

```bash
cargo install cargo-deb --version 2.9.2 --locked
cargo build --release --locked
cargo deb --no-build
```

The `debian/server-status.service` file is automatically packaged by `cargo-deb`. Installing the package registers, enables, and starts the service when systemd is available:

```bash
sudo apt install ./server-status_*.deb
systemctl status server-status
curl http://127.0.0.1:3000/api/status
```

To override the bind address, use `sudo systemctl edit server-status` and add:

```ini
[Service]
Environment=SERVER_STATUS_HOST=0.0.0.0
```

Then run `sudo systemctl restart server-status`. Only expose port 3000 behind suitable authentication, TLS, and network access controls.
