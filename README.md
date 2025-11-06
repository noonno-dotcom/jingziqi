# 井字棋游戏

一个基于 Rust 和 Tauri 开发的井字棋游戏，具有美观的图形界面。

## 功能特性

- ✅ 玩家对战模式
- ✅ 人机对战模式（AI使用极小极大算法）
- ✅ 可选择玩家先手或AI先手
- ✅ 现代化的用户界面
- ✅ 实时游戏状态显示

## 技术栈

- **后端**: Rust
- **前端**: HTML, CSS, JavaScript
- **框架**: Tauri 1.x

## 安装要求

1. **Rust**: 需要安装 Rust 工具链
   ```bash
   # 如果还没有安装 Rust，请访问 https://www.rust-lang.org/tools/install
   ```

2. **Tauri CLI**: 安装 Tauri CLI
   ```bash
   cargo install tauri-cli
   ```

3. **系统依赖**:
   - Windows: 需要安装 Microsoft Visual C++ Redistributable
   - 其他系统依赖请参考 [Tauri 官方文档](https://tauri.app/v1/guides/getting-started/prerequisites)

## 运行项目

### 开发模式

```bash
cargo tauri dev
```

### 构建生产版本

```bash
cargo tauri build
```

构建完成后，可执行文件将位于 `src-tauri/target/release/` 目录下。

## 游戏说明

### 玩家对战模式
1. 选择"玩家对战"
2. 两个玩家轮流在棋盘上放置 X 和 O
3. 先连成一条线的玩家获胜

### 人机对战模式
1. 选择"人机对战"
2. 选择先手（玩家先手或AI先手）
3. 玩家使用 X，AI 使用 O
4. AI 使用极小极大算法，具有最优策略
5. 与 AI 对战通常以平局告终（如果玩家也使用最优策略）

## 项目结构

```
jingziqi/
├── src-tauri/          # Tauri 后端
│   ├── src/            # Rust 源代码
│   │   ├── main.rs     # 主程序入口和 Tauri 命令
│   │   └── game.rs     # 游戏逻辑（棋盘、AI算法）
│   ├── Cargo.toml      # Rust 依赖配置
│   ├── tauri.conf.json # Tauri 配置文件
│   └── build.rs        # 构建脚本
├── index.html          # 前端 HTML
├── styles.css          # 样式文件
├── app.js             # 前端 JavaScript
└── README.md          # 项目说明
```

## 开发说明

### Rust 代码结构
- `src/game.rs`: 包含游戏核心逻辑
  - `Game` 结构体：管理游戏状态
  - `Player` 枚举：X 和 O 玩家
  - `GameStatus` 枚举：游戏状态（进行中、X获胜、O获胜、平局）
  - `minimax` 函数：极小极大算法实现

- `src/main.rs`: Tauri 应用入口
  - `new_game`: 创建新游戏
  - `make_move`: 执行移动
  - `get_ai_move`: AI 自动下棋

### 前端代码结构
- `index.html`: 游戏界面结构
- `styles.css`: 样式和动画
- `app.js`: 游戏交互逻辑和前后端通信

## 许可证

MIT License

