/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */

const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./src/**/*.html", "./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        gray: colors.neutral,
        bg: "#29273f",
        toast: "rgb(39 38 63 / 19%)",
        dropdown: "#27263f",
        "dropdown-border": "#676583",
      },
      boxShadow: {
        t3rn: "rgba(255, 82, 67, 0.2) 0px 8px 24px 0px,rgba(67, 183, 255, 0.2) 0px -8px 24px 0px",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
