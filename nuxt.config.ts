// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
   compatibilityDate: "2024-04-03",
   devtools: { enabled: true },

   modules: [
      "@nuxt/image",
      "@nuxt/eslint",
      "@nuxtjs/tailwindcss",
      "@nuxtjs/i18n",
      "@vite-pwa/nuxt",
      "@nuxt/fonts",
      "@pinia/nuxt",
      "@vueuse/nuxt",
   ],

   i18n: {
      locales: ["en-US", "nl-NL"],
      strategy: "prefix",
      defaultLocale: "en-US",
      detectBrowserLanguage: false,
      vueI18n: "./i18n.config.ts",
      lazy: false,
   },

   fonts: {
      families: [{ name: "Inter", provider: "fontsource" }],
   },

   pwa: {
      manifest: {
         name: "Resin",
         short_name: "Resin",
         theme_color: "#F07E19",
         description: "Resin - Buy a home without a bank",
         icons: [
            {
               src: "android-chrome-256x256.png",
               sizes: "150x150",
               type: "image/png",
            },
         ],
      },
      workbox: {
         navigateFallback: null,
      },
      devOptions: {
         enabled: false,
      },
   },

   app: {
      head: {
         title: "Resin - Buy a home without a bank",
         meta: [
            { charset: "utf-8" },
            {
               name: "viewport",
               content: "width=device-width, initial-scale=1",
            },
            {
               hid: "description",
               name: "description",
               content: "Nuxt Image module example",
            },
         ],
      },
   },

   // Nuxt Image module configuration
   image: {
      format: ["webp"],
      quality: 80,
   },
});