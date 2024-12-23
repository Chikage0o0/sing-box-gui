const { nextui } = require("@nextui-org/react")

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "./node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        onehalfdark: {
          bg: '#2e3440',
          header: '#3b4252',
          text: '#d8dee9',
          icon: '#81a1c1',
          title: '#88c0d0',
          sidebar: '#4c566a',
          'sidebar-item': '#434c5e',
          toggle: '#434c5e',
          'toggle-text': '#d8dee9',
          'toggle-hover': '#5e81ac',
        },
        onehalflight: {
          bg: '#ffffff',
          header: '#f0f0f0',
          text: '#333333',
          icon: '#555555',
          title: '#666666',
          sidebar: '#e0e0e0',
          'sidebar-item': '#d0d0d0',
          toggle: '#c0c0c0',
          'toggle-text': '#333333',
          'toggle-hover': '#a0a0a0',
        },
      },
    },
  },
  darkMode: "class",
  plugins: [nextui()]
}

