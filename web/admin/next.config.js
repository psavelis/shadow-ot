/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  transpilePackages: ['@shadow-ot/shared'],
  output: 'standalone',
}
module.exports = nextConfig

