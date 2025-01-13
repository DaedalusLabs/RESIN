// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
   compatibilityDate: "2024-04-03",
   devtools: { enabled: true },
   vite: {
      server: {
         hmr: {
            overlay: false,
         },
      },
      optimizeDeps: {
         include: [
            "@phosphor-icons/vue",
            "flowbite",
            "maplibre-gl",
            "pmtiles",
            "blurhash",
            "marked",
            "dompurify",
         ],
         force: true,
         holdUntilCrawlEnd: true,
         noDiscovery: false,
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
      "pinia-plugin-persistedstate/nuxt",
      "@nuxt/test-utils",
      "@nuxtjs/sitemap",
      "@nuxtjs/robots",
      "nuxt-jsonld",
   ],

   i18n: {
      locales: [
         {
            code: "en-US",
            name: "English",
            language: "en",
            iso: "en-US",
         },
         {
            code: "nl-NL",
            name: "Nederlands",
            language: "nl",
            iso: "nl-NL",
         },
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
      manifestFilename: "site.webmanifest",
      manifest: {
         id: "https://resin.estate/",
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
            {
               src: "web-app-manifest-512x512.png",
               sizes: "512x512",
               type: "image/png",
               purpose: "maskable",
            },
         ],
      },
      workbox: {
         navigateFallback: null,
         navigateFallbackAllowlist: [],
         globDirectory: "public",
         globPatterns: ["**/*.{js,css,html,ico,png,svg}"],
         ignoreURLParametersMatching: [/^utm_/, /^fbclid$/],
         skipWaiting: true,
         clientsClaim: true,
         cleanupOutdatedCaches: true,
         additionalManifestEntries: [
            { url: "/nostr-notifications-sw.js", revision: null },
            { url: "/ndk-bundle.min.js", revision: null },
         ],
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
               content: "Resin - Rent to own, without a loan",
            },
            { name: "mobile-web-app-capable", content: "yes" },
            { name: "apple-mobile-web-app-capable", content: "yes" },
            {
               name: "apple-mobile-web-app-status-bar-style",
               content: "default",
            },
            { name: "apple-mobile-web-app-title", content: "RESIN" },

            { name: "theme-color", content: "#F07E19" },
            { name: "icon", content: "/favicon.ico", rel: "shortcut icon" },
            {
               name: "icon",
               content: "/favicon-96x96.png",
               rel: "icon",
               sizes: "96x96",
               type: "image/png",
            },
            {
               name: "icon",
               content: "/favicon.svg",
               rel: "icon",
               type: "image/svg+xml",
            },
            {
               name: "icon",
               content: "/apple-touch-icon.png",
               rel: "apple-touch-icon",
               sizes: "180x180",
            },
            { name: "og:title", content: "Resin", property: "og:title" },
            {
               name: "og:description",
               content: "Resin - Rent to own, without a loan",
               property: "og:description",
            },
            {
               name: "og:image",
               content: "/android-chrome-256x256.png",
               property: "og:image",
            },
         ],
      },
   },

   // Nuxt Image module configuration
   image: {
      format: ["webp"],
      quality: 80,
      provider: "none",
   },

   runtimeConfig: {
      public: {
         IMAGES_BASE_URL: process.env.IMAGES_BASE_URL,
         BACKEND_ENDPOINT: process.env.BACKEND_ENDPOINT,
         TYPESENSE_ENDPOINT: process.env.TYPESENSE_ENDPOINT,
         TYPESENSE_HOST: process.env.TYPESENSE_HOST,
         TYPESENSE_PORT: process.env.TYPESENSE_PORT,
         TYPESENSE_API_KEY: process.env.TYPESENSE_API_KEY,
         TYPESENSE_PROTOCOL: process.env.TYPESENSE_PROTOCOL,
         MESSAGES_PUBKEY: process.env.MESSAGES_PUBKEY,
         PUBKEY_WHITELIST: (process.env.PUBKEY_WHITELIST || "").split(","),
         FILES_BASE_URL: process.env.FILES_BASE_URL,
         SHOW_TRENDING_AREAS: process.env.SHOW_TRENDING_AREAS,
         SHOW_TRANSACTIONS: process.env.SHOW_TRANSACTIONS,
         SHOW_AGREEMENTS: process.env.SHOW_AGREEMENTS,
         SHOW_FINANCIALS: process.env.SHOW_FINANCIALS,
         BTCPAY_BASE_URL: process.env.BTCPAY_BASE_URL,
      },
   },

   nitro: {
      publicAssets: [
         {
            baseURL: "/",
            dir: "public",
            maxAge: 0, // Disable caching for service worker
         },
      ],
   },
   routeRules: {
      "/payment/**": { robots: false },
      "/settings/**": { robots: false },
      "/choose-property-type": { robots: false },
      "/my-*": { robots: false },
   },
   sitemap: {
      exclude: [
         "/payment/**",
         "/settings/**",
         "/choose-property-type",
         "/favorites",
         "/messages",
         "/transactions",
         "/financials",
         new RegExp("/my-.*"),
      ],
      urls: async () => {
         const urls = await fetch(
            `${process.env.BACKEND_ENDPOINT}/listings/urls`,
         );
         return urls;
      },
      sources: [`${process.env.BACKEND_ENDPOINT}/listings/urls`],
   },
   robots: {
      groups: [
         {
            userAgent: "*",
            allow: "/",
         },
      ],
      sitemap: "/sitemap.xml",
   },
});
