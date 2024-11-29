export const useFormatNumber = () => {
   const { locale } = useI18n();

   const formatNumber = (number: number) => {
      try {
         return new Intl.NumberFormat(locale.value, {
            style: "decimal",
            minimumFractionDigits: 0,
            maximumFractionDigits: 0,
         }).format(number);
      } catch (error) {
         console.error("Error formatting number:", error);
         return "0";
      }
   };

   const formatCurrency = (number: number, currency = "USD") => {
      try {
         return new Intl.NumberFormat(locale.value, {
            style: "currency",
            currency,
            minimumFractionDigits: 0,
            maximumFractionDigits: 0,
         }).format(number);
      } catch (error) {
         console.error("Error formatting currency:", error);
         return "$0";
      }
   };

   return {
      formatNumber,
      formatCurrency,
   };
};
