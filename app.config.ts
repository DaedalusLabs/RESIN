export default defineAppConfig({
   showTrendingAreas: process.env.VITE_SHOW_TRENDING_AREAS === "true",
   showTransactions: process.env.VITE_SHOW_TRANSACTIONS === "true",
   showAgreements: process.env.VITE_SHOW_AGREEMENTS === "true",
   showFinancials: process.env.VITE_SHOW_FINANCIALS === "true",
});
