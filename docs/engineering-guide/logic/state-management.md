# 状态管理逻辑
> [PRINCIPLE] 本规范依据以下白皮书原则推导：
> 1. [渲染约束 · Alpha合成的非直觉叠加](../../whitepaper/index.md#311-渲染约束)
> 2. [第四条戒律 · 涟漪的生命](../../whitepaper/index.md#第四条-涟漪的生命)
---
## 1. 状态互斥原则
**问题**：多状态同时激活（如 hover + focus + active）时，CSS 的 Alpha 背景会非预期叠加，产生“无因之色”。
**解决方案**：逻辑层维护单一状态优先级栈，输出确定的 `data-state` 属性。CSS 只消费确定状态。
### 优先级栈
```javascript
// 伪代码
function determineState(interactions) {
  if (interactions.focus) return 'focus';
  if (interactions.active) return 'active';
  if (interactions.hover) return 'hover';
  return 'default';
}
```
**优先级**：`focus > active > hover > default`。
---
## 2. 实现约束
| 约束 | 要求 | 破坏后果 |
|---|---|---|
| **禁止 CSS Alpha 叠加** | 不使用 `:hover:hover` 或多重伪类叠加 | 产生非预期颜色，违反因果不虚 |
| **必须使用 data-state** | 通过 `[data-state="focus"]` 选择器控制样式 | 确保逻辑层唯一控制权 |
| **降级路径** | 无 JS 环境下，重协商为 CSS 有限互斥 (如 `:focus-visible` 覆盖 `:hover`) | 确保在邮件客户端等环境的基础可用性 |
> **标记**：若因历史原因使用 CSS 有限互斥，必须标记 `TODO: [DEBT]`，并在条件允许时迁移至逻辑层。
---
## 3. 媒体查询响应
尊重用户的系统偏好。
| 查询 | 响应行为 | 适用规则 |
|---|---|---|
| `prefers-reduced-motion: reduce` | 停止所有非必要动画（流转、引导） | [PHYS] 避免引发眩晕或注意力分散 |
| `prefers-color-scheme: dark/light` | 切换背景色与文本色 | [PURPOSE] 目的裁决模式 |
---
## 4. 色彩函数与渲染
| 函数 | 用途 | 约束 |
|---|---|---|
| `oklch(l c h)` | 现代色彩空间，支持感知均匀调整 | 确保不同亮度下的色相一致性 |
| `rgba(r g b a)` | 传统 Alpha 透明度 | 需注意背景混合风险 |
| `color-mix()` | 动态混合前景与背景 | 避免产生非预期的“无因之色” |
> **渲染边角**：Pentile OLED 排列可能导致 1px 线条出现色相偏移，详见 [布局与尺寸规范](../layout/sizing.md#5-分割线)。
