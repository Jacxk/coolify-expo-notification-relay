# AGENTS.md

## Cursor Cloud specific instructions

### Project overview

Coolify Expo Notification Relay — a single-binary Rust (Axum) HTTP server that receives Coolify webhook payloads and forwards them as Expo push notifications. See `README.md` for environment variables and usage.

### Rust toolchain

`Cargo.toml` specifies `edition = "2024"`, which requires Rust **1.85+**. The update script runs `rustup update stable` to ensure this.

### Build / lint / test commands

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Lint | `cargo clippy` |
| Format check | `cargo fmt --check` |
| Tests | `cargo test` |

### Running the application

The server requires the `EXPO_PUSH_TOKENS` env var (comma-separated `ExponentPushToken[...]` values). For local dev/testing you can use a dummy token:

```sh
EXPO_PUSH_TOKENS="ExponentPushToken[test123]" cargo run
```

The server binds to `0.0.0.0:3000` by default. Health check: `GET /health`. Webhook endpoint: `POST /` (configurable via `WEBHOOK_PATH`).

### Known pre-existing issues

- `cargo clippy -- -D warnings` fails with 2 warnings (`.len() == 0` → `.is_empty()` in `deployment_poller.rs`, and a `single_match` lint in `repeater.rs`). Running `cargo clippy` without `-D warnings` reports these as warnings only.
- `cargo fmt --check` reports formatting diffs in several files.
- The `invalid_expo_token_format_is_rejected` integration test in `tests/expo_token_tests.rs` fails. This is a pre-existing issue in the test or the validation logic.
