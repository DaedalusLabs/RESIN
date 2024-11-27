<template>
   <div class="flex items-start gap-2.5">
      <img class="h-8 w-8 rounded-full" :src="profileImage" alt="Chat avatar" />
      <div class="flex w-full max-w-[320px] flex-col gap-1">
         <div class="flex items-center space-x-2 rtl:space-x-reverse">
            <span class="text-sm font-semibold text-gray-900 dark:text-white">
               {{ name }}
            </span>
            <span class="text-sm font-normal text-gray-500 dark:text-gray-400">
               {{ time }}
            </span>
         </div>
         <div
            class="leading-1.5 flex flex-col rounded-e-xl rounded-es-xl border-gray-200 bg-gray-100 p-4 dark:bg-gray-700"
         >
            <p class="text-sm font-normal text-gray-900 dark:text-white">
               {{ message }}
            </p>
         </div>
         <span class="text-sm font-normal text-gray-500 dark:text-gray-400">
            {{ status }}
         </span>
      </div>
   </div>
</template>

<script setup>
defineProps({
   name: {
      type: String,
      required: true,
   },
   message: {
      type: String,
      required: true,
   },
   status: {
      type: String,
      required: true,
   },
   time: {
      type: String,
      required: true,
   },
   profileImage: {
      type: String,
      required: true,
   },
});
</script>
