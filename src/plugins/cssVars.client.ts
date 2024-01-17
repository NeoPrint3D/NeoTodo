import { colors } from "../../tailwind.config";

export default defineNuxtPlugin((app) => {
  useHead(
    {
      style: `
        :root {
          ${Object.entries(colors)
            .map(([name, value]) => `--color-${name}: ${value};`)
            .join("\n")}
        }
      `,
    },
    app
  );
});
