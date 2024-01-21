// https://nuxt.com/docs/api/configuration/nuxt-config
export const nuxtConfig = {
  devtools: { enabled: true },
  srcDir: "src/",
  ssr: false,
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
  googleFonts: {
    families: {
      "Space+Grotesk": true,
    },
  },
  build: {
    transpile: ["@vuepic/vue-datepicker", "shared-lib"],
  },
  css: ["~/assets/css/tailwind.css", "@vuepic/vue-datepicker/dist/main.css"],
  modules: ["@tresjs/nuxt", "@nuxtjs/google-fonts", "nuxt-icon"],
};
