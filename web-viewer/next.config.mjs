/** @type {import('next').NextConfig} */
const nextConfig = {
  // 消除 Turbopack 错误
  turbopack: {},
  
  webpack: (config, { isServer }) => {
    if (isServer) {
      config.module.rules.push({
        test: /\.md$/,
        use: 'raw-loader',
      });
    }
    return config;
  },
};

export default nextConfig;
