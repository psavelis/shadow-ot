/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  transpilePackages: ['@shadow-ot/shared'],
  images: {
    domains: ['shadow-ot.com', 'api.shadow-ot.com'],
  },
}

module.exports = nextConfig

