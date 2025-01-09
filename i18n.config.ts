import enUS from './locales/en-US.json'
import nlNL from './locales/nl-NL.json'

export default defineI18nConfig(() => ({
   legacy: false,
   fallbackLocale: "en-US",
   globalInjection: true,
   useScope: 'global',
   messages: {
      "en-US": enUS,
      "nl-NL": nlNL,
      "nl": nlNL
   },
}));
