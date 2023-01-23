/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}',
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],
  theme: {
    extend: {},
    fontWeight: {
      extralight: 200,
      light: 300,
      normal: 400,
      semibold: 600,
      bold: 700,
    }
  },
  plugins: [
    require("daisyui"),
    // require('flowbite/plugin'),
    require('@tailwindcss/typography'),
  ],
  darkMode: 'class',
}
