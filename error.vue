<template>
   <NuxtLayout name="intro">
      <div
         class="flex h-full flex-col items-center justify-between px-12 py-20"
      >
         <div class="w-full max-w-2xl space-y-8 text-center">
            <div class="flex justify-center">
               <img
                  :src="'/images/logos/resin-text.png'"
                  alt="Error illustration"
                  class="mb-6 h-10"
               />
            </div>
            <h1 class="text-4xl font-bold text-white">
               {{
                  error?.statusCode === 404 ? "Page Not Found" : "Server Error"
               }}
            </h1>
            <h2 class="text-xl text-gray-300">
               {{
                  error?.statusCode === 404
                     ? "The page you are looking for might have been removed or is temporarily unavailable"
                     : "Something went wrong on our end. Please try again later"
               }}
            </h2>
            <div class="flex justify-center gap-4">
               <ResinButton
                  :text="`Go Home`"
                  size="lg"
                  :show-icon="false"
                  @click="handleHome"
               />
               <ResinButton
                  :text="`Try Again`"
                  size="lg"
                  variant="outline"
                  :show-icon="false"
                  @click="handleError"
               />
            </div>
         </div>
      </div>
   </NuxtLayout>
</template>

<script setup lang="ts">
definePageMeta({
   layout: "intro",
});

defineProps<{
   error: {
      statusCode?: number;
      message?: string;
   };
}>();

function handleHome() {
   navigateTo("/");
}

function handleError() {
   clearError();
}
</script>
