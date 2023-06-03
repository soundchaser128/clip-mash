// eslint-disable-next-line @typescript-eslint/no-var-requires
const colors = require("tailwindcss/colors")

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    colors: {
      ...colors,
      brand: "#471AA0",
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      "dark",
      {
        clipmash: {
          // eslint-disable-next-line @typescript-eslint/no-var-requires
          ...require("daisyui/src/theming/themes")["[data-theme=light]"],
          primary: "#471AA0",
          secondary: "#dd77d0"
        },
      },
    ],
  },
}
