# demo01 — Top-Down Survival Shooter

一个用 [Raylib](https://www.raylib.com/) + Rust 写的俯视角生存射击游戏 demo。

**全部使用几何图形**（绿色方块、红色方块、黄色圆形），零美术资源依赖。

## 游戏玩法

| 操作 | 效果 |
|------|------|
| WASD / 方向键 | 移动 |
| 鼠标 | 瞄准 |
| 左键 | 射击 |
| P | 暂停 / 继续 |
| Enter | 游戏结束后重新开始 |

敌人会从屏幕边缘不断生成并向你追来。每波需要击杀一定数量敌人才能进入下一波，击杀数达标后回 1 点 HP。随着波次增加，敌人 HP 和生成数量逐步提升。

## 技术栈

- **语言**: Rust
- **引擎**: Raylib 3.7（2D 渲染、输入、音频）
- **随机数**: fastrand 2

## 运行

```bash
# macOS（需要先装 raylib）
brew install raylib
cargo run --release -p demo01

# Linux（需要 X11）
sudo apt install libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libgl1-mesa-dev
cargo run --release -p demo01
```

## 代码结构

单个源文件 `src/main.rs`（~490 行），按功能分区：

| 模块 | 行号 | 功能 |
|------|------|------|
| Constants | 1-20 | 游戏参数（速度、大小、冷却） |
| Game Objects | 23-42 | Player / Bullet / Enemy / Particle 结构体 |
| Main Loop | 50-155 | 主循环：更新 + 渲染 |
| update_player | 157-175 | WASD 输入 + 位置更新 |
| update_shooting | 179-191 | 鼠标左键射击 |
| update_bullets | 195-215 | 子弹移动 + 生命周期 |
| update_enemies AI | 241-247 | 敌人向玩家追逐 |
| Collisions | 249-281 | 子弹→敌人 / 敌人→玩家伤害 |
| Particles | 283-289 | 粒子物理 + 衰减 |
| draw_frame | 295-470 | 所有渲染逻辑 |
