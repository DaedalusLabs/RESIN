interface ErrorWithStatusCode extends Error {
   statusCode?: number;
}

export default defineNuxtPlugin((nuxtApp) => {
   nuxtApp.vueApp.config.errorHandler = (error: unknown, instance, info) => {
      // Handle the error
      console.error("Global error:", error);

      // Create a formatted error object
      const errorObject = {
         statusCode: (error as ErrorWithStatusCode)?.statusCode || 500,
         message:
            error instanceof Error
               ? error.message
               : "An unexpected error occurred",
         stack: error instanceof Error ? error.stack : undefined,
         info,
      };

      // Show the error page
      showError(errorObject);
   };

   // Handle Nuxt errors
   nuxtApp.hook("vue:error", (error) => {
      console.error("Nuxt error:", error);
   });
});
