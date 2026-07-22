# Lumtract
> **构建可视化的结构，如同在水面上折射光。**
Lumtract 是一个基于 Monorepo 架构的结构化数据可视化平台。它致力于在硅基逻辑与碳基美学之间寻找平衡，将复杂的拓扑数据转化为直观、流畅的交互体验。
---
## 🏛️ 架构概览
本项目采用 **Rust + Next.js** 的混合架构，严格遵循“单一职责原则”：
-   **Backend (Rust)**: 位于根目录。负责核心数据的生成、解析与图算法（基于 `dag-generator`）。
-   **Frontend (Next.js)**: 位于 `web-viewer`。基于 App Router，提供高性能的文档渲染与交互界面。
-   **Design System (Storybook)**: 位于 `web-viewer/.storybook`。独立的 UI 组件实验室，实现了“UI 表现层”与“数据逻辑层”的解耦。
---
## 🚀 快速开始
### 环境要求
-   Node.js 18+
-   Rust 1.70+ (如需运行生成器)
-   pnpm/npm/yarn
### 1. 安装依赖
```bash
cd web-viewer
npm install
```
### 2. 运行应用
这会启动 Next.js 开发服务器，访问 `http://localhost:3000`。
```bash
npm run dev
```
### 3. 组件开发 (Storybook)
进入 UI 设计系统界面，查看和调试 Lumtract 组件库。
```bash
npm run storybook
# 访问 http://localhost:6006
```
---
## 🎨 Lumtract 设计哲学
### 水质波光
我们的 UI 不追求机械的锐利，而是模仿水的物理属性：
-   **流动感**：使用微光和玻璃态模拟水面的折射与流动。
-   **有机形态**：使用 `rounded-xl` 等自然圆角，拒绝完美的几何圆，呼应碳基生命的柔性。
-   **硅基骨架**：在底层架构上保持严格的工程约束，确保性能与稳定性。
### 深海模式
推荐在深色背景 (`#0f172a`) 下体验我们的 Glass Mode 组件，感受光影在深水中的穿透力。
---
## 🛠️ 技术栈
-   **Framework**: Next.js 15+ (App Router)
-   **Styling**: Tailwind CSS
-   **Component Lab**: Storybook
-   **Backend**: Rust (Cargo)
---
## 🙏 致谢
**特别感谢 [Storybook](https://storybook.js.org/)**
Storybook 不仅仅是一个工具，它不仅是 Lumtract 的“组件实验室”，更是我们工程化思维的放大器。
正是 Storybook 的“隔离渲染”能力，让我们得以将 `web-viewer` 的表现层从复杂的业务逻辑中剥离出来，专注于打磨每一个像素的“水波光”质感。它赋予了我们在不运行主应用的情况下，独立迭代设计系统的自由。
*“优雅不是多余的设计，而是恰到好处的解耦。”*
---
## 📄 License
Apache 2.0
