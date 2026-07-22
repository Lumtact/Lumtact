import type { Preview } from '@storybook/react';
import '../app/globals.css'; // 👈 关键！这行代码给组件穿上了衣服

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
};

export default preview;
