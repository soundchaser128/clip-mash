/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        "clip-mash-dark": {
          // eslint-disable-next-line @typescript-eslint/no-var-requires
          ...require("daisyui/src/theming/themes")["[data-theme=dark]"],
          primary: "hsl(260,100%,70%)",
          secondary: "#dd77d0",
        },
      },
      {
        "clip-mash-light": {
          // eslint-disable-next-line @typescript-eslint/no-var-requires
          ...require("daisyui/src/theming/themes")["[data-theme=light]"],
          primary: "#471AA0",
          secondary: "#dd77d0",
        },
      },
    ],
  },
}
