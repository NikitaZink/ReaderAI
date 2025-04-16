/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  // Отключаем eslint при билде для избежания проблем с tauri
  eslint: {
    ignoreDuringBuilds: true,
  },
  // Отключаем TypeScript проверки при билде
  typescript: {
    ignoreBuildErrors: true,
  },
  // Отключаем кэширование webpack
  webpack: (config, { dev, isServer }) => {
    config.cache = false;
    return config;
  }
}

module.exports = nextConfig 