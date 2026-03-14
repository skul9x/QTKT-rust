/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#0ea5e9',
          dark: '#0284c7',
        },
        secondary: '#10b981',
        background: '#0f172a',
        surface: '#1e293b',
        text: {
          DEFAULT: '#f8fafc',
          muted: '#94a3b8',
        },
        danger: '#ef4444',
      },
      fontFamily: {
        inter: ['Inter', 'sans-serif'],
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
};
