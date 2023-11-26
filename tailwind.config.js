/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: { 
    files: ["./site/*.html", "./site/src/**/*.rs", "./crates/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        primarylight: '#415091',
        secondarylight: '#949FCE',
      }
    },
  },
  plugins: [],
}