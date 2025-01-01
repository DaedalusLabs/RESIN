// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
   compatibilityDate: "2024-04-03",
   devtools: { enabled: true },
   vite: {
      server: {
         hmr: {
            overlay: true,
         },
      },
   },
   modules: [
      "@nuxt/image",
      "@nuxt/eslint",
      "@nuxtjs/tailwindcss",
      "@nuxtjs/i18n",
      "@vite-pwa/nuxt",
      "@nuxt/fonts",
      "@pinia/nuxt",
      "@vueuse/nuxt",
      'pinia-plugin-persistedstate/nuxt'
   ],

   i18n: {
      locales: [
         {
            code: "en-US",
            name: "English"
         },
         {
            code: "nl-NL",
            name: "Nederlands"
         }
      ],
      strategy: "prefix_except_default",
      defaultLocale: "en-US",
      detectBrowserLanguage: false,
      vueI18n: "./i18n.config.ts",
      lazy: false,
   },

   fonts: {
      families: [{ name: "Inter", provider: "fontsource" }],
   },

   ssr: false,
   spaLoadingTemplate: true,
   pwa: {
      registerType: "autoUpdate",
      manifest: {
         name: "Resin",
         short_name: "Resin",
         theme_color: "#F07E19",
         description: "Resin - Buy a home without a bank",
         icons: [
            {
               src: "android-chrome-256x256.png",
               sizes: "256x256",
               type: "image/png",
            },
            {
               src: "android-chrome-192x192.png",
               sizes: "192x192",
               type: "image/png",
            },
         ],
      },
      workbox: {
         navigateFallback: '/',
         navigateFallbackAllowlist: [/^\/$/],
      },
      devOptions: {
         enabled: false,
      },
   },

   app: {
      head: {
         title: "Resin",
         meta: [
            { charset: "utf-8" },
            {
               name: "viewport",
               content: "width=device-width, initial-scale=1",
            },
            {
               hid: "description",
               name: "description",
               content: "Resin - Buy a home without a bank",
            },
            { name: 'mobile-web-app-capable', content: 'yes' },
            { name: 'apple-mobile-web-app-capable', content: 'yes' },
            { name: 'apple-mobile-web-app-status-bar-style', content: 'default' }
         ],
      },
   },

   // Nuxt Image module configuration
   image: {
      format: ["webp"],
      quality: 80,
      provider: "none"
   },

   runtimeConfig: {
      public: {
         IMAGES_BASE_URL: process.env.IMAGES_BASE_URL,
         BACKEND_ENDPOINT: process.env.BACKEND_ENDPOINT,
         TYPESENSE_ENDPOINT: process.env.TYPESENSE_ENDPOINT,
         TYPESENSE_HOST: process.env.TYPESENSE_HOST,
         TYPESENSE_PORT: process.env.TYPESENSE_PORT,
         TYPESENSE_API_KEY: process.env.TYPESENSE_API_KEY,
         MESSAGES_NPUB: process.env.MESSAGES_NPUB,
         FILES_BASE_URL: process.env.FILES_BASE_URL,
         SHOW_TRENDING_AREAS: process.env.SHOW_TRENDING_AREAS,
         SHOW_TRANSACTIONS: process.env.SHOW_TRANSACTIONS,
         SHOW_AGREEMENTS: process.env.SHOW_AGREEMENTS,
         SHOW_FINANCIALS: process.env.SHOW_FINANCIALS,
      },
   },
});
