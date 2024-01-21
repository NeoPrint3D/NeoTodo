import { colors } from "config/tailwind.config";

export const getCssVars = () => `:root {
    ${Object.entries(colors)
      .map(([name, value]) => `--color-${name}: ${value};`)
      .join("\n")}
    }`;
