<template>
   <div class="mx-auto max-w-md p-4 text-center">
      <div class="rounded-lg bg-white p-8 shadow-md">
         <div class="mb-6">
            <Icon
               name="heroicons:clock"
               class="mx-auto mb-4 h-16 w-16 text-red-500"
            />
            <h1 class="mb-2 text-2xl font-bold">Payment Expired</h1>
            <p class="text-gray-600">
               The payment request has expired. Would you like to try again?
            </p>
         </div>

         <div class="space-y-4">
            <button
               class="w-full rounded-lg bg-[#F7931A] px-4 py-3 font-medium text-white transition-colors hover:bg-[#E88A18]"
               @click="
                  router.push(
                     '/payment/request' +
                        (route.query.id ? `?id=${route.query.id}` : ''),
                  )
               "
            >
               Try Again
            </button>

            <button
               class="w-full rounded-lg bg-gray-100 px-4 py-3 font-medium text-gray-700 transition-colors hover:bg-gray-200"
               @click="router.push('/')"
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
