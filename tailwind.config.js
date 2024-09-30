/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        backgroundImage: {
          'gradient-to-r-red-blue': 'linear-gradient(to right, red, blue)',
        },
      },
    },
    plugins: [],
  }