# Lumtract
> **构建可视化的结构，如同在水面上折射光。**
Lumtract 是一个基于 Monorepo 架构的结构化数据可视化平台。它致力于在硅基逻辑与碳基美学之间寻找平衡，将复杂的拓扑数据转化为直观、流畅的交互体验。
---
## 🏛️ 架构概览
本项目采用 **Rust + Next.js** 的混合架构，严格遵循“数据优先”原则：
1.  **Backend (Rust)**: 核心数据引擎。负责基于 `dag-generator` 算法生成结构化的 JSON 数据。
2.  **Frontend (Next.js)**: 位于 `web-viewer`。负责读取 JSON 并进行高性能的文档渲染与交互。
3.  **Design System (Storybook)**: 位于 `web-viewer/.storybook`。独立的 UI 组件实验室。
---
## 🚀 快速开始
### 0. 环境要求
-   Rust 1.70+
-   Node.js 18+
### 1. 生成数据 (Rust Backend)
项目的起点是数据。运行 Rust 生成器，将输出结果直接重定向到前端的公共资源目录，使其成为可视化层的数据源。
```bash
# 在项目根目录执行
cargo run --release > web-viewer/public/dag-data.json
```
*(现在 `web-viewer` 可以通过 `/dag-data.json` 访问这些数据)*
### 2. 启动前端
进入前端目录，安装依赖并启动服务。
```bash
cd web-viewer
npm install
npm run dev
```
访问 `http://localhost:3000` 查看生成的图谱结构。
### 3. 组件开发 (Storybook)
进入 UI 设计系统界面，查看和调试 Lumtract 组件库。
```bash
# 仍在 web-viewer 目录下
npm run storybook
```
---
## 🎨 Lumtract 设计哲学
### 水质波光
我们的 UI 不追求机械的锐利，而是模仿水的物理属性：
-   **流动感**：使用微光和玻璃态模拟水面的折射与流动。
-   **有机形态**：使用 `rounded-xl` 等自然圆角，拒绝完美的几何圆，呼应碳基生命的柔性。
-   **硅基骨架**：在底层架构上保持严格的工程约束，确保性能与稳定性。
### 深海发光
在 Dark Mode 下，我们模拟深海生物的“自发光”：
-   **可见性 (硅基约束)**：拒绝隐形。通过辉光和对比度确保信息传递的高效。
-   **交互感 (碳基反馈)**：交互时产生能量聚集的辉光，而非生硬的色块切换。
---
## 🛠️ 技术栈
-   **Core**: Rust (Cargo)
-   **Framework**: Next.js 15+ (App Router)
-   **Styling**: Tailwind CSS
-   **Component Lab**: Storybook
---
## 🙏 致谢
**特别感谢 [Storybook](https://storybook.js.org/)**
Storybook 不仅仅是一个工具，它不仅是 Lumtract 的“组件实验室”，更是我们工程化思维的放大器。
正是 Storybook 的“隔离渲染”能力，让我们得以将 `web-viewer` 的表现层从复杂的业务逻辑中剥离出来，专注于打磨每一个像素的“水波光”质感。它赋予了我们在不运行主应用的情况下，独立迭代设计系统的自由。
*“优雅不是多余的设计，而是恰到好处的解耦。”*
---
## 📄 License
Apache 2.0
