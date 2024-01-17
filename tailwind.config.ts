import type { Config } from "tailwindcss";
import daisyui from "daisyui";
import themes from "daisyui/src/theming/themes";

export const colors = themes["dracula"];

const config: Config = {
  content: [
    "./src/pages/**/*.vue",
    "./src/layouts/**/*.vue",
    "./src/components/**/*.vue",
  ],

  themes: {
    extends: {
      colors,
      fontFamily: {
        main: ["Space Grotesk", "sans-serif"],
      },
    },
  },
  daisyui: {
    themes: [
      {
        mytheme: colors,
      },
    ],
  },
  plugins: [daisyui],
};

export default config;
