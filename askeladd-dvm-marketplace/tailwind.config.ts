/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        arcade: ['"Press Start 2P"', 'cursive'],
      },
      colors: {
        'neon-green': '#00ff00',
        'neon-pink': '#ff00ff',
        'neon-blue': '#00ffff',
        'neon-yellow': '#ffff00',
        'neon-orange': '#ff8000',
        'neon-red': '#ff0000',
        'dark-purple': '#2a0e61',
      },
    },
  },
  plugins: [],
};