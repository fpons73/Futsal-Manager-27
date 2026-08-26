/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        fm: {
          bg: "#101418",
          panel: "#181d24",
          panel2: "#1f2630",
          border: "#2a3340",
          accent: "#00c853",
          accent2: "#2979ff",
          warn: "#ffb300",
          danger: "#ff5252",
          text: "#e8eaed",
          dim: "#9aa4b0"
        }
      }
    }
  },
  plugins: []
};
