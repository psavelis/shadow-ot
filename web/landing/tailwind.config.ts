import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        // Shadow OT Brand Colors
        shadow: {
          50: '#f5f5f6',
          100: '#e6e6e8',
          200: '#cfcfd4',
          300: '#adaeb5',
          400: '#84858f',
          500: '#696a74',
          600: '#5a5b63',
          700: '#4c4d54',
          800: '#434449',
          900: '#3b3c40',
          950: '#1a1a2e',
        },
        accent: {
          50: '#fef2f3',
          100: '#fee2e4',
          200: '#fecacd',
          300: '#fca5ab',
          400: '#f8717a',
          500: '#e94560',
          600: '#d62c47',
          700: '#b4213a',
          800: '#961f36',
          900: '#7e1f33',
          950: '#450b17',
        },
        // Realm-specific colors
        aetheria: {
          primary: '#4A90D9',
          secondary: '#FFD700',
        },
        shadowveil: {
          primary: '#1a1a2e',
          secondary: '#e94560',
        },
        warbound: {
          primary: '#8B0000',
          secondary: '#FF4500',
        },
        mythara: {
          primary: '#6B46C1',
          secondary: '#9F7AEA',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        display: ['Cinzel', 'serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
        'hero-pattern': "url('/images/hero-bg.jpg')",
      },
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.5s ease-out',
        'pulse-glow': 'pulseGlow 2s ease-in-out infinite',
        'float': 'float 3s ease-in-out infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        pulseGlow: {
          '0%, 100%': { boxShadow: '0 0 5px rgba(233, 69, 96, 0.5)' },
          '50%': { boxShadow: '0 0 20px rgba(233, 69, 96, 0.8)' },
        },
        float: {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-10px)' },
        },
      },
    },
  },
  plugins: [],
}
export default config
