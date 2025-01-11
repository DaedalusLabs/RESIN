<template>
   <div
      :class="{ 'border-resin-500': checked }"
      class="block h-full max-w-sm cursor-pointer rounded-2xl border-4 border-gray-200 bg-white p-4 pr-4 shadow hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700"
      @click="$emit('checked')"
   >
      <div class="flex justify-between">
         <h5
            class="text-md mb-1 w-3/4 font-bold leading-tight tracking-tight text-gray-900"
         >
            {{ title }}
         </h5>
         <div
            class="flex h-8 w-8 items-center justify-center rounded-full border-2 border-gray-200"
            :class="{ 'bg-resin-500': checked }"
         >
            <svg
               v-if="checked"
               xmlns="http://www.w3.org/2000/svg"
               class="h-4 w-4 text-white"
               fill="none"
               viewBox="0 0 24 24"
               stroke="currentColor"
            >
               <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="3"
                  d="M5 13l4 4L19 7"
               />
            </svg>
         </div>
      </div>
      <p class="text-sm font-normal text-gray-400">
         {{ description }}
      </p>
   </div>
</template>

<script setup>
defineProps({
   id: {
      type: String,
      required: true,
   },
   checked: {
      type: Boolean,
      default: false,
   },
   title: {
      type: String,
      required: true,
   },
   description: {
      type: String,
      required: true,
   },
});

defineEmits(["checked"]);
</script>
