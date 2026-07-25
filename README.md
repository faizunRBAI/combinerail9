# combinerail9

A Rust web service built with [Actix-Web](https://actix.rs/), deployed to AWS EC2 via the UDAP platform.

## Architecture

- **Runtime:** Rust (stable) / Actix-Web 4
- **Cloud:** AWS EC2 — `t3.micro`, `us-east-1`
- **OS:** Ubuntu 22.04 LTS
- **Reverse Proxy:** Nginx (port 80 → app port 8080)
- **Process Manager:** systemd
- **IaC:** Terraform (state in S3 via UDAP)
- **Configuration:** Ansible
- **CI/CD:** GitHub Actions (rendered from `.udap/pipeline.yaml`)

## Endpoints

| Method | Path        | Description              |
|--------|-------------|--------------------------|
| GET    | `/`         | Landing page (HTML)      |
| GET    | `/health`   | Health check (`{"status":"ok"}`) |
| GET    | `/api/info` | Service metadata (JSON)  |

## Local Development

**Prerequisites:** Rust stable toolchain ([rustup](https://rustup.rs/))

```bash
# Clone and run
cargo run

# App available at http://localhost:8080
```

```bash
# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check
```

## Deploy Pipeline

Pushing to `main` triggers the CI/CD pipeline:

| Stage       | Description                                              |
|-------------|----------------------------------------------------------|
| `lint`      | `cargo fmt --check` + `cargo clippy`                    |
| `test`      | `cargo test`                                             |
| `build`     | `cargo build --release` — binary uploaded as artifact    |
| `provision` | Terraform provisions EC2 + EIP + SG + key pair           |
| `configure` | Ansible deploys the binary, systemd service, nginx proxy |
| `verify`    | HTTP health check with retry/backoff on the public IP    |

## Configuration

Environment variables read by the service at runtime:

| Variable   | Default | Description       |
|------------|---------|-------------------|
| `PORT`     | `8080`  | App listen port   |
| `RUST_LOG` | `info`  | Log level         |

## Operations

**Check service status (on the EC2 instance):**
```bash
systemctl status combinerail9
journalctl -u combinerail9 -f
```

**Restart the service:**
```bash
sudo systemctl restart combinerail9
```

**Access URL:** `http://<instance-ip>` (set after first deploy)

## Infrastructure

Terraform resources under `infra/`:

- `aws_instance` — Ubuntu 22.04, `t3.micro`
- `aws_eip` — Static public IP
- `aws_security_group` — Ingress: TCP 80, TCP 22; Egress: all
- `aws_key_pair` — Platform-managed SSH key pair

**Destroy infrastructure** (from the UDAP platform Destroy action — does not delete the repository):
```
Trigger the destroy workflow from GitHub Actions → workflow_dispatch
```
