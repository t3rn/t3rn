/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */

const tailwindcss = require("tailwindcss");
const postcssPresetEnv = require("postcss-preset-env");

module.exports = {
  plugins: [postcssPresetEnv(), tailwindcss],
};
