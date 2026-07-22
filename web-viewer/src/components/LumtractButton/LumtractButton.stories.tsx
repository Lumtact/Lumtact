import type { Meta, StoryObj } from '@storybook/react';
import { LumtractButton } from './LumtractButton';

const meta: Meta<typeof LumtractButton> = {
  title: 'Lumtract/Button',
  component: LumtractButton,
  parameters: {
    layout: 'centered',
    backgrounds: {
      // 为了看清 Glass 效果，我们在 Storybook 里加个深色背景预设
      values: [
        { name: 'Light', value: '#ffffff' },
        { name: 'Deep Ocean', value: '#0f172a' }, // 推荐切换到这个看效果
      ],
    },
  },
  tags: ['autodocs'],
  argTypes: {
    mode: { control: 'select', options: ['solid', 'glass', 'ghost'] },
    size: { control: 'select', options: ['sm', 'md', 'lg'] }
  }
};

export default meta;
type Story = StoryObj<typeof LumtractButton>;

export const SolidMode: Story = {
  args: {
    mode: 'solid',
    children: '确认执行',
  },
};

// 玻璃态最符合你的“水”哲学，请在 Deep Ocean 背景下观看
export const GlassMode: Story = {
  args: {
    mode: 'glass',
    children: '光波传输',
  },
};

export const GhostMode: Story = {
  args: {
    mode: 'ghost',
    children: '静默模式',
  },
};

export const SizePalette: Story = {
  render: () => (
    <div className="flex gap-4 items-center">
      <LumtractButton size="sm" mode="glass">Small</LumtractButton>
      <LumtractButton size="md" mode="glass">Medium</LumtractButton>
      <LumtractButton size="lg" mode="glass">Large</LumtractButton>
    </div>
  )
};
