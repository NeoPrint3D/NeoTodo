import { getCssVars } from "../../../../packages/shared-lib";

export default defineNuxtPlugin((app) => {
  useHead({
    style: {
      innerHTML: getCssVars(),
    },
  });
});
