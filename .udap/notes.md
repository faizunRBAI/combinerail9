# combinerail9 — Build Notes

## Project
- **Name:** combinerail9
- **Cloud:** AWS / us-east-1 / EC2 (t3.micro)
- **OS:** Ubuntu 22.04 LTS (ssh_user=ubuntu)
- **Stack:** Rust/Actix-Web, nginx reverse proxy, systemd, Terraform + Ansible
- **VCS:** GitHub, branch: main
- **No database** — stateless service

## Decisions
- Reusing the default VPC (probe confirmed 1 VPC, 6 subnets) — no custom VPC needed at Tier 1.
- Elastic IP attached so the public IP is stable across stop/start cycles.
- App listens on port 8080 internally; nginx proxies port 80 → 8080.
- Rust binary built in CI (release mode) and uploaded as a GitHub Actions artifact, then downloaded by the configure stage for Ansible deployment. No Rust toolchain needed on the EC2 instance.
- systemd service runs as a dedicated `combinerail9` system user (no root).
- verify stage re-inits terraform to read the EIP (not threaded via job outputs, avoiding GitHub secret masking of PROJECT_NAME-derived values).

## Pipeline Stages
1. lint → cargo fmt + clippy
2. test → cargo test
3. build → cargo build --release, artifact upload
4. provision → Terraform (EC2 + EIP + SG + key pair)
5. configure → Ansible (install nginx, deploy binary, systemd service)
6. verify → curl with retry on /health

## Status
- [ ] Plan approved
- [ ] Generation complete
- [ ] validate_project clean
- [ ] Repo pushed
- [ ] Deployed

## Known Gotchas
- apt lock race on fresh Ubuntu instances — playbook waits for dpkg lock before installing packages.
- GitHub Actions artifact path: binary is uploaded from `target/release/combinerail9` and downloaded to `artifact/` in the configure stage; Ansible copies from `artifact/combinerail9`.
- SSH_USER is platform-derived from the Ubuntu AMI → value is `ubuntu`.
