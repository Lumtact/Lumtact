# 🌊 Lumtract

> **结构如水，逻辑如光。**  
> 在硅基的精确与碳基的感知之间，构建可视化的拓扑叙事。

Lumtract 是一个基于 Monorepo 架构的结构化数据可视化平台。它致力于将复杂的 DAG（有向无环图）数据转化为直观、流畅的交互体验——如水面折射出的光影，既有严谨的骨架，亦有灵动的质感。

---

## 🏛️ 架构概览

本项目采用 **Rust + Next.js** 混合架构，遵循“数据驱动视图”原则：

| 层级 | 技术栈 | 职责 |
| :--- | :--- | :--- |
| **数据引擎** | Rust (`dag-generator`) | 扫描 `web-viewer/public/docs/` 下的 Markdown 文档，生成结构化拓扑数据（JSON），输出到 `web-viewer/public/dag-manifest.json` |
| **表现层** | Next.js 16 (`web-viewer`) | 数据驱动的文档阅读器，负责渲染知识图谱与文档内容 |
| **设计系统** | Storybook 10 (`web-viewer/.storybook`) | 独立的 UI 组件实验室，用于开发与调试 |

---

## 🚀 快速开始

### 0. 环境准备

- **Rust** 1.70+
- **Node.js** 20+（推荐 LTS 版本）

### 1. 生成知识图谱数据

Lumtract 的核心是文档知识图谱。运行 Rust 引擎扫描 `web-viewer/public/docs/` 下的所有 Markdown 文档，提取链接关系与层级结构：

```bash
cd dag-generator
cargo run --release
```

执行后，终端会显示：
```
✅ Manifest generated: ../web-viewer/public/dag-manifest.json
```

### 2. 启动前端

进入前端目录，安装依赖并启动开发服务器：

```bash
cd web-viewer
npm install
npm run dev
```

访问 [http://localhost:3000](http://localhost:3000) 查看文档知识图谱。

### 3. 启动组件实验室（Storybook）

在 `web-viewer` 目录下，启动独立的设计系统环境：

```bash
npm run storybook
```

访问 [http://localhost:6006](http://localhost:6006) 浏览和调试 Lumtract 组件库。

---

## 📚 文档图谱

Lumtract 本身就是一个**知识图谱浏览器**。`web-viewer/public/docs/` 目录下的所有 Markdown 文档会被自动扫描、建立链接关系，并在前端以分层导航的方式呈现。

当前包含的推导体系文档：

| 文档 | 内容 |
| :--- | :--- |
| `philosophy.md` | 设计推导法 · 公理系统 |
| `constraints.md` | 设计推导法 · 约束库 |
| `derivation.md` | 设计推导法 · 推导引擎 |
| `archive.md` | Lumtract 实例档案 |
| `system-summary.md` | 体系总纲 |

这些文档可通过 `http://localhost:3000` 直接阅读，文档内部的 Markdown 链接会在应用内平滑导航，无需刷新页面。

---

## 🎨 设计哲学：水波与光

Lumtract 的 UI 灵感源于自然水体，追求 **“理性的感性表达”**：

- **流动感**：借助微光与玻璃态质感，模拟水面折射与流动性，使界面柔和且富有呼吸感。
- **有机形态**：采用自然的圆角（`rounded-xl`），避免几何上的完美圆弧，呼应碳基生命的柔性特质。
- **硅基骨架**：底层保持严格的工程约束与高性能数据流，确保系统稳定可控。

在 **深色模式** 下，我们进一步引入“深海发光”概念：

- **自发光交互**：模拟深海生物发光机制，通过辉光与对比度强化核心信息传递。
- **能量反馈**：交互时以辉光聚集代替生硬颜色切换，强化操作感知与响应感。

---

## 🛠️ 技术栈

| 类别 | 技术 |
| :--- | :--- |
| 核心引擎 | Rust, Cargo |
| 前端框架 | Next.js 16 (App Router) |
| 样式方案 | Tailwind CSS 4 |
| 组件开发环境 | Storybook 10 |
| 语言支持 | TypeScript, Rust 2021 Edition |

---

## 🙏 致谢

特别感谢 **[Storybook](https://storybook.js.org/)**。

Storybook 不仅是 Lumtract 的“组件实验室”，更是我们工程化思维的放大器。它提供的隔离渲染能力，让我们能够将表现层从复杂业务逻辑中彻底剥离，在不运行主应用的前提下独立迭代设计系统，实现真正的“设计即代码”。

> *“优雅不是多余的设计，而是恰到好处的解耦。”*

感谢所有开源社区的无私贡献，让这样的构建方式成为可能。

---

## 📄 License

Apache 2.0

© 2026 Lumtract Contributors