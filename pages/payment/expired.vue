<template>
  <div class="max-w-md mx-auto p-4 text-center">
    <div class="bg-white rounded-lg shadow-md p-8">
      <div class="mb-6">
        <Icon name="heroicons:clock" class="w-16 h-16 text-red-500 mx-auto mb-4" />
        <h1 class="text-2xl font-bold mb-2">Payment Expired</h1>
        <p class="text-gray-600">
          The payment request has expired. Would you like to try again?
        </p>
      </div>
      
      <div class="space-y-4">
        <button
          @click="router.push('/payment/request' + (route.query.id ? `?id=${route.query.id}` : ''))"
          class="w-full bg-[#F7931A] text-white py-3 px-4 rounded-lg font-medium hover:bg-[#E88A18] transition-colors"
        >
          Try Again
        </button>
        
        <button
          @click="router.push('/')"
          class="w-full bg-gray-100 text-gray-700 py-3 px-4 rounded-lg font-medium hover:bg-gray-200 transition-colors"
        >
          Return Home
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const router = useRouter();
const route = useRoute();
</script> 