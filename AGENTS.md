# AGENTS.md — Game Demos Workspace

## Project Overview

Rust game development workspace. Sub-projects under `~/git/mine/games/`, each in its own directory listed in the root `Cargo.toml` `[workspace].members`.

## Quick Start

```bash
# Build all
cargo build --release

# Build & run specific project
cargo run --release -p demo01
```

## Project Structure

```
games/
├── Cargo.toml          ← workspace root — add new projects here
├── README.md           ← workspace overview
├── demo01/
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── README.md       ← per-project docs
└── target/             ← shared build cache
```

## Adding a New Game

1. `cargo new <name> --name <name>` (inside games/)
2. Add `"<name>"` to `members` in root `Cargo.toml`
3. Create `src/main.rs` with Raylib boilerplate
4. Create `README.md` for the sub-project
5. Collapse to single commit: `git add -A && git commit -m "feat: add <name> (...)"`

## Conventions

- **Single `main.rs`**: Keep each game in one file until it exceeds ~800 lines
- **All geometry first**: Use `DrawRectangle`/`DrawCircle`/`DrawLine` before any sprite textures — ship the game logic, then skin
- **No binary assets in repo**: Textures/sounds go via code (procedural) or are downloaded at build time
- **Workspace-level deps**: Shared dependencies (`raylib`, `fastrand`) are specified per crate; plan to extract common utilities if 3+ games share them
- **Cross-platform**: Windows (MSVC/mingw), macOS (Metal), Linux (X11) — CI should cover all three
- **Formatting**: Use `cargo fmt` before commit; long lines (over 100 cols) in function calls are OK for readability

## Tech Stack

| Layer | Choice |
|-------|--------|
| Language | Rust (edition 2021) |
| Graphics | Raylib 3.7 (raylib-rs) |
| RNG | fastrand 2 |

## Publishing

- Remote: `git@github.com:yusiwen/game_demos.git`
- No binary releases yet (just learning projects)
