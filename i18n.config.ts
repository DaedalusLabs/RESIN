import enUS from './locales/en-US.json'
import nlNL from './locales/nl-NL.json'

export default defineI18nConfig(() => ({
   legacy: false,
   fallbackLocale: "en-US",
   messages: {
      "en-US": enUS,
      "nl-NL": nlNL
   },
}));
