export default defineAppConfig({
  BACKEND_ENDPOINT: 'http://localhost:3001',
  // BACKEND_ENDPOINT: 'http://localhost:3001',
  TYPESENSE_ENDPOINT: 'http://localhost:8100',
  TYPESENSE_HOST: 'http://localhost',
  TYPESENSE_PORT: 8108,
  TYPESENSE_APIKEY: 'IAc8hL8tUngzHRFrPP6iorEik8TyaSbV',
  MESSAGES_NPUB: 'fd22007ee363ec08c2fe2a2b28173afeb2875d7c58e3c306b22a77dae8d3ee93',
  // IMAGES_BASE_URL: 'http://localhost:3001/',
  IMAGES_BASE_URL: 'http://localhost:3001/',
//  https://api.resin.estate
  showTrendingAreas: process.env.VITE_SHOW_TRENDING_AREAS === 'true',
  showTransactions: process.env.VITE_SHOW_TRANSACTIONS === 'true',
  showAgreements: process.env.VITE_SHOW_AGREEMENTS === 'true',
  showFinancials: process.env.VITE_SHOW_FINANCIALS === 'true'
})