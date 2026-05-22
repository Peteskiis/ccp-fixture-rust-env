# ccp-fixture-rust-env

Minimal Rust HTTP fixture used by [Peteskiis/cluster-infra](https://github.com/Peteskiis/cluster-infra) e2e suite `13-compute-env-rust.sh` (#165). Exercises `ccp compute deploy` autodetect + `.env` autoload + env-source precedence.

## Routes

- `GET /env/<KEY>` → plain-text value of `std::env::var(KEY)` (empty if unset)
- `GET /health` → 200 OK

## Why stdlib-only

A tokio/axum fixture would push the e2e tier past its ~10-12 min budget with cold cross-compilation to `x86_64-unknown-linux-musl`. Stdlib gets us to ~30s cold builds and the surface is small enough that "minimal HTTP parser" isn't worse than a framework here.

## Pinning

The e2e suite checks out a specific commit SHA, not `main`. Bumps require updating both the upstream tag (`e2e-v1`) and the SHA in `tests/e2e/13-compute-env-rust.sh`.
