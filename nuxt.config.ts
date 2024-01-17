// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: false },
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
    transpile: ["@vuepic/vue-datepicker"],
  },
  css: ["~/assets/css/tailwind.css", "@vuepic/vue-datepicker/dist/main.css"],
  modules: ["@nuxtjs/google-fonts", "nuxt-icon"],
});
