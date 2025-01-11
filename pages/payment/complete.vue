<template>
   <div class="mx-auto max-w-md p-4 text-center">
      <div class="rounded-lg bg-white p-8 shadow-md">
         <div class="mb-6">
            <PhCheckCircle class="mx-auto mb-4 h-16 w-16 text-green-500" />
            <h1 class="mb-2 text-2xl font-bold">Payment Complete</h1>
            <p class="text-gray-600">
               Thank you for your payment. Your transaction has been processed
               successfully.
            </p>
         </div>

         <div class="space-y-4">
            <NuxtLinkLocale to="/">
               <button
                  class="w-full rounded-lg bg-gray-100 px-4 py-3 font-medium text-gray-700 transition-colors hover:bg-gray-200"
               >
                  Return Home
               </button>
            </NuxtLinkLocale>
         </div>
      </div>
   </div>
</template>

<script setup lang="ts">
import { PhCheckCircle } from "@phosphor-icons/vue";
</script>
