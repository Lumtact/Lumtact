# Button 组件使用指南
> [RULE] 本指南依据以下工程规则推导：
> 1. [布局与尺寸规范 · 触摸目标](../../engineering-guide/layout/sizing.md#1-触摸目标)
> 2. [颜色基因映射 · 涟漪层透明度梯度](../../engineering-guide/tokens/color-gene.md#3-涟漪层透明度梯度)
> 3. [状态管理逻辑 · 状态互斥原则](../../engineering-guide/logic/state-management.md#1-状态互斥原则)
---
## 1. 基础用法
Button 组件默认继承涟漪层基因色，并严格遵循状态互斥原则。
```jsx
import { Button } from '@lumtact/ui';
<Button onClick={handleClick}>
  确认提交
</Button>
```
---
## 2. 尺寸约束
基于 [菲茨定律](../../engineering-guide/layout/sizing.md#1-触摸目标)，Button 的点击热区必须 ≥ 44px。
| 尺寸变体 | 视觉高度 | 点击热区 | 适用场景 |
|---|---|---|---|
| `small` | 32px | 44px (通过 Padding 扩展) | 紧凑列表、卡片内部 |
| `medium` (默认) | 40px | 44px+ | 表单、常规操作 |
| `large` | 48px | 48px | 移动端主操作、营销页面 |
> **实现注意**：`small` 尺寸虽然视觉较小，但必须通过 `min-height: 44px` 或内部 Padding 扩展热区，防止误触。
---
## 3. 状态与样式
Button 使用 `data-state` 属性控制样式，由逻辑层维护优先级栈。
### 状态优先级
`focus` > `active` (按下) > `hover` > `default`
### 透明度映射 (基于水蓝 #2B8CBE)
| 状态 | CSS 变量 (示例) | Alpha 值 | 视觉表现 |
|---|---|---|---|
| **default** | `--ripple-default` | 0.15 | 极淡的水蓝色背景，不抢夺注意力 |
| **hover** | `--ripple-hover` | 0.30 | 清晰的交互反馈 |
| **active** | `--ripple-active` | 0.40 | 沉浸的按下感 |
| **focus** | `--ripple-focus` | 0.15 + 外部光环 | 明确的焦点指示，满足无障碍需求 |
### 代码示例
```css
/* 逻辑层输出：[data-state="hover"] */
.button[data-state="hover"] {
  background-color: color-mix(in srgb, var(--color-gene-water-blue) var(--alpha-hover), transparent);
  /* 此时 Alpha = 0.30 */
  transition: background-color 100ms ease-out; /* 符合感知下限 */
}
```
---
## 4. 危险操作
危险操作（如删除、退出）使用信号层色相（暖红），但必须遵循双重编码原则（图标 + 文字）。
```jsx
<Button variant="danger" icon={<IconTrash />}>
  删除项目
</Button>
```
> [RULE] 详见 [颜色基因映射 · 信号层色相](../../engineering-guide/tokens/color-gene.md#6-信号层色相双重编码)
---
## 5. 加载状态
操作反馈必须跨越因果感知下限 (100ms)。若操作耗时可能超过 1s，需显示 Loading。
```jsx
<Button loading>
  提交中...
</Button>
```
*   **行为**：点击后立即进入 `loading` 状态（锁定交互）。
*   **动效**：Spinner 旋转 600ms 后若未完成，建议显示文字提示“请稍候”，防止焦虑。
---
## 6. 常见错误
❌ **错误：仅靠颜色区分状态**
```css
.button:hover { color: red; } /* 忽略了背景变化和 Alpha */
```
✅ **正确：使用变量叠加**
```css
.button[data-state="hover"] { background-color: var(--ripple-hover); }
```
❌ **错误：忽略无障碍**
```jsx
// 没有处理 focus 状态
```
✅ **正确：显式定义 focus 样式**
```css
.button[data-state="focus"] {
  box-shadow: 0 0 0 2px var(--color-focus-ring);
}
```
