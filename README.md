# Game Demos

Rust 游戏开发练习项目集合。使用 [Raylib](https://www.raylib.com/) 作为图形引擎，全部使用几何图形和免费资源，零美术依赖。

## 项目列表

| 项目 | 描述 | 状态 |
|------|------|------|
| [demo01](./demo01/) | 俯视角生存射击 | ✅ 可玩 |

## 前置依赖

### macOS

```bash
brew install raylib
```

### Linux

```bash
sudo apt install libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libgl1-mesa-dev
```

### Windows

Visual Studio Build Tools 或 mingw 环境自带必要的图形库，无需额外安装。

## 编译与运行

```bash
# 编译全部项目
cargo build --release

# 运行指定项目
cargo run --release -p demo01

# 在当前目录下运行
cd demo01 && cargo run --release
```

## Workspace 结构

```
game_demos/
├── Cargo.toml          ← workspace 根配置
├── Cargo.lock          ← 统一 lockfile
├── demo01/             ← 子项目
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── README.md
└── target/             ← 共享编译缓存
```

新增项目：`cargo new <name>` → 在根 `Cargo.toml` 的 `members` 列表追加 `"<name>"`。
