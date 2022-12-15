/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}',
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],
  theme: {
    extend: {},
    fontWeight: {
      normal: 400,
      medium: 600,
    }
  },
  plugins: [
    require('flowbite/plugin'),
    require('@tailwindcss/typography'),
  ],
  darkMode: 'class',
}
