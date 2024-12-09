<template>
   <transition
      enter-active-class="transition-opacity duration-500 ease-out"
      leave-active-class="transition-opacity duration-500 ease-in"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
   >
      <div
         v-if="visible"
         id="toast-success"
         class="fixed right-4 top-4 z-50 mb-4 flex w-full max-w-xs items-center rounded-lg bg-white p-4 text-gray-500 shadow-lg dark:bg-gray-800 dark:text-gray-400"
         role="alert"
      >
         <div
            class="inline-flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-lg bg-green-100 text-green-500 dark:bg-green-800 dark:text-green-200"
         >
            <PhCheck size="20" />
            <span class="sr-only">Check icon</span>
         </div>
         <div class="ml-3 text-sm font-normal">{{ text }}</div>
         <button
            type="button"
            class="-mx-1.5 -my-1.5 ml-auto inline-flex h-8 w-8 items-center justify-center rounded-lg bg-white p-1.5 text-gray-400 hover:bg-gray-100 hover:text-gray-900 focus:ring-2 focus:ring-gray-300 dark:bg-gray-800 dark:text-gray-500 dark:hover:bg-gray-700 dark:hover:text-white"
            aria-label="Close"
            @click="closeToast"
         >
            <span class="sr-only">Close</span>
            <svg
               class="h-3 w-3"
               aria-hidden="true"
               xmlns="http://www.w3.org/2000/svg"
               fill="none"
               viewBox="0 0 14 14"
            >
               <path
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M1 1l6 6m0 0l6 6M7 7l6-6M7 7l-6 6"
               />
            </svg>
         </button>
      </div>
   </transition>
</template>

<script setup>
import { PhCheck } from "@phosphor-icons/vue";

const props = defineProps({
   text: {
      type: String,
      required: true,
   },
   showToast: {
      type: Boolean,
      required: true,
   },
});

const visible = ref(false);

const emit = defineEmits(["close-toast"]);

const closeToast = () => {
   visible.value = false;
   emit("close-toast");
};

watchEffect(() => {
   visible.value = props.showToast;

   if (visible.value) {
      setTimeout(() => {
         closeToast();
      }, 5000); // Toast disappears after 5 seconds
   }
});
</script>
