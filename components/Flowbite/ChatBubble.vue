<template>
   <div
      :class="[
         'flex gap-2.5',
         isSent
            ? 'flex-row-reverse items-start self-end'
            : 'flex-row items-start self-start',
      ]"
   >
      <img class="h-8 w-8 rounded-full" :src="profileImage" alt="Chat avatar" />
      <div
         :class="[
            'flex w-full max-w-[320px] flex-col gap-1',
            isSent ? 'text-right' : 'text-left',
         ]"
      >
         <div
            :class="[
               'flex items-center space-x-2 rtl:space-x-reverse',
               isSent ? 'justify-end' : 'justify-start',
            ]"
         >
            <span class="text-sm font-semibold text-gray-900 dark:text-white">
               {{ name }}
            </span>
            <span class="text-sm font-normal text-gray-500 dark:text-gray-400">
               {{ time }}
            </span>
         </div>
         <div
            class="leading-1.5 flex flex-col rounded-e-xl rounded-es-xl border-gray-200 bg-gray-100 p-4 text-gray-900 dark:bg-gray-700"
         >
            <p class="text-sm font-normal">{{ message }}</p>
         </div>
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

   time: {
      type: String,
      required: true,
   },
   profileImage: {
      type: String,
      required: false, // Optional for sent messages
   },
   isSent: {
      type: Boolean,
      required: true,
   },
});
</script>
