<script setup>
defineProps({
   showDrawer: {
      type: Boolean,
      default: false,
   },
   imageUrls: {
      type: Array,
      required: true,
   },
});

const emit = defineEmits(["close"]);
</script>

<template>
   <transition name="fade" appear>
      <div
         v-if="showDrawer"
         class="fixed inset-0 z-40 flex items-end bg-black bg-opacity-50 transition-opacity"
         @click.self="emit('close')"
      >
         <transition name="slide-up" appear>
            <div class="h-5/6 w-full overflow-y-auto bg-white dark:bg-gray-800">
               <div
                  class="sticky top-0 z-50 cursor-pointer bg-white hover:bg-gray-50 dark:bg-gray-800 dark:hover:bg-gray-700"
                  @click="emit('close')"
               >
                  <span
                     class="absolute left-1/2 top-3 h-1 w-8 -translate-x-1/2 rounded-lg bg-gray-500 dark:bg-gray-600"
                  />
               </div>
               <div class="flex flex-col items-center justify-between">
                  <div
                     v-if="imageUrls.length === 0"
                     class="text-center text-gray-500"
                  >
                     No images to display
                  </div>
                  <NuxtImg
                     v-for="(imageUrl, index) in imageUrls"
                     :key="index"
                     :src="imageUrl"
                     class="object-cover"
                     loading="lazy"
                  />
               </div>
            </div>
         </transition>
      </div>
   </transition>
</template>

<style>
.slide-up-enter-active,
.slide-up-leave-active {
   transition:
      transform 0.3s ease,
      opacity 0.3s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
   transform: translateY(100%);
   opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
   transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
   opacity: 0;
}
</style>
