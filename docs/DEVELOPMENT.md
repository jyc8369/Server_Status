# Development

## Project

This repository contains a Rust HTTP API. The application entry point is `src/main.rs`; package metadata and dependencies are in `Cargo.toml` and `Cargo.lock`.

The release workflow installs the stable Rust toolchain and builds on Ubuntu 24.04. The verified local run command is:

```powershell
cargo run
```

The server reads `SERVER_STATUS_HOST` and `SERVER_STATUS_PORT`. Their defaults are `0.0.0.0` and `3000`. For example, in PowerShell:

```powershell
$env:SERVER_STATUS_HOST = "127.0.0.1"
$env:SERVER_STATUS_PORT = "8080"
cargo run
```

## Debian release

The GitHub Actions workflow at `.github/workflows/release-deb.yml` runs when a `v*` tag is pushed. The tag version must match the package version in `Cargo.toml`. It builds a Linux Debian package on Ubuntu 24.04 and uploads it to the GitHub Release for that tag.

The workflow's packaging commands are:

```bash
cargo install cargo-deb --locked
cargo build --release --locked
cargo deb --no-build
```

The repository is private, so pushing tags and accessing release artifacts requires repository access.
