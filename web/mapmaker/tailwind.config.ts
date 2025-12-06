import type { Config } from 'tailwindcss'

const config: Config = {
  content: ['./pages/**/*.{js,ts,jsx,tsx,mdx}', './components/**/*.{js,ts,jsx,tsx,mdx}', './app/**/*.{js,ts,jsx,tsx,mdx}'],
  theme: {
    extend: {
      colors: {
        shadow: { 50: '#f5f5f6', 100: '#e6e6e8', 200: '#cfcfd4', 300: '#adaeb5', 400: '#84858f', 500: '#696a74', 600: '#5a5b63', 700: '#4c4d54', 800: '#434449', 900: '#3b3c40', 950: '#1a1a2e' },
        accent: { 50: '#fef2f3', 100: '#fee2e4', 200: '#fecacd', 300: '#fca5ab', 400: '#f8717a', 500: '#e94560', 600: '#d62c47', 700: '#b4213a', 800: '#961f36', 900: '#7e1f33', 950: '#450b17' },
      },
      fontFamily: { sans: ['Inter', 'system-ui', 'sans-serif'], display: ['Cinzel', 'serif'], mono: ['JetBrains Mono', 'monospace'] },
    },
  },
  plugins: [],
}
export default config

