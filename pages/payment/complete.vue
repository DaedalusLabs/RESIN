<template>
  <div class="max-w-md mx-auto p-4 text-center">
    <div class="bg-white rounded-lg shadow-md p-8">
      <div class="mb-6">
        <PhCheckCircle class="w-16 h-16 text-green-500 mx-auto mb-4" />
        <h1 class="text-2xl font-bold mb-2">Payment Complete</h1>
        <p class="text-gray-600">
          Thank you for your payment. Your transaction has been processed successfully.
        </p>
      </div>
      
      <div class="space-y-4">       
        <NuxtLinkLocale to="/">
          <button
            class="w-full bg-gray-100 text-gray-700 py-3 px-4 rounded-lg font-medium hover:bg-gray-200 transition-colors"
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

const router = useRouter();
</script> 