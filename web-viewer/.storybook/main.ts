import type { StorybookConfig } from '@storybook/nextjs';

const config: StorybookConfig = {
  // 👇 下面这行是关键，把 ../src 加进去了
  stories: [
    "../stories/**/*.mdx",
    "../stories/**/*.stories.@(js|jsx|mjs|ts|tsx)",
    "../src/**/*.mdx",
    "../src/**/*.stories.@(js|jsx|mjs|ts|tsx)"
  ],
  addons: [
    "@storybook/addon-a11y",
    "@storybook/addon-docs",
    "@storybook/addon-onboarding"
  ],
  framework: "@storybook/nextjs",
  staticDirs: [
    "../public"
  ]
};
export default config;
