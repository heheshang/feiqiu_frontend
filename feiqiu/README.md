# 飞秋 (FeiQiu)

基于 Tauri + React + Vite 的局域网通讯工具，兼容飞秋（FeiQ）和飞鸽传书（IPMsg）协议。

## 技术栈

- **前端框架**: React 18 + TypeScript
- **构建工具**: Vite
- **桌面应用**: Tauri 2.0
- **状态管理**: Zustand
- **样式**: Tailwind CSS 4
- **图标**: Lucide React
- **日期处理**: date-fns

## 开发环境要求

- [Bun](https://bun.sh/) (推荐) 或 Node.js >= 18
- Rust (用于 Tauri 后端)
- 系统依赖（根据操作系统不同）:
  - **macOS**: Xcode Command Line Tools
  - **Linux**: libwebkit2gtk-4.0-dev, build-essential, curl, wget, file, libssl-dev
  - **Windows**: Microsoft C++ Build Tools + WebView2

## 快速开始

### 1. 安装依赖

```bash
# 使用 Bun (推荐)
bun install

# 或使用 npm
npm install
```

### 2. 开发模式

```bash
# 启动开发服务器
bun tauri dev

# 或者分别运行
bun run dev          # 前端开发服务器
bun run tauri dev    # Tauri 应用
```

### 3. 构建应用

```bash
# 构建生产版本
bun tauri build

# 构建输出位于 src-tauri/target/release/bundle/
```

## 可用脚本

```bash
# 开发
bun run dev          # 启动 Vite 开发服务器
bun run tauri dev    # 启动 Tauri 开发模式（完整应用）

# 构建
bun run build        # 构建前端资源 (TypeScript + Vite)
bun run preview      # 预览生产构建
bun run tauri build  # 构建 Tauri 桌面应用

# Tauri CLI
bun run tauri [command]  # 运行 Tauri CLI 命令
```

## 项目结构

```
feiqiu/
├── src/                 # 前端源代码
│   ├── components/      # React 组件
│   ├── stores/          # Zustand 状态管理
│   ├── utils/           # 工具函数
│   └── main.tsx         # 应用入口
├── src-tauri/           # Rust 后端代码
│   ├── src/             # Rust 源代码
│   ├── examples/        # Rust 示例代码
│   ├── capabilities/    # Tauri 能力配置
│   └── tauri.conf.json  # Tauri 配置文件
├── public/              # 静态资源
├── dist/                # 构建输出目录
└── package.json         # 项目配置
```

## 开发说明

### 前端开发

前端使用 Vite 进行热更新开发，默认运行在 `http://localhost:1420`。

### 后端开发 (Rust)

Tauri 后端使用 Rust 编写，位于 `src-tauri/` 目录。

运行 Rust 示例：

```bash
cd src-tauri
cargo run --example feiq_discovery
```

更多示例请参考 [src-tauri/examples/README.md](./src-tauri/examples/README.md)

## IPMsg 协议

本应用实现了 IPMsg 协议，与以下软件兼容：

- ✅ 飞秋（FeiQ）
- ✅ 飞鸽传书（IPMsg）
- ✅ 其他支持 IPMsg 协议的 LAN 通讯软件

协议详细信息请参考 [src-tauri/examples/README.md](./src-tauri/examples/README.md)

## 性能优化

本项目使用 Bun 作为包管理器，相比 npm 具有以下优势：

- ⚡ 更快的安装速度（最高 20 倍）
- 📦 更小的 node_modules
- 🔧 内置的 dev server 和 test runner
- 💾 更高效的依赖管理

## 故障排除

### 端口被占用

如果端口 1420 被占用，可以在 `vite.config.ts` 中修改端口：

```typescript
server: {
  port: 1420, // 修改为其他端口
  strictPort: false,
}
```

### Tauri 构建失败

确保已安装 Rust 和系统依赖：

```bash
# 检查 Rust 版本
rustc --version

# 检查 Cargo 版本
cargo --version
```

### 依赖安装问题

```bash
# 清理缓存并重新安装
rm -rf node_modules package-lock.json bun.lockb
bun install
```

## 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

MIT License

## 相关链接

- [Tauri 文档](https://tauri.app/)
- [React 文档](https://react.dev/)
- [Vite 文档](https://vitejs.dev/)
- [Bun 文档](https://bun.sh/docs)
- [Tailwind CSS 文档](https://tailwindcss.com/)
