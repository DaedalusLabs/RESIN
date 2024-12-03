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
            <div
               class="w-full overflow-y-auto bg-white dark:bg-gray-800 lg:min-h-[75vh] lg:self-end"
            >
               <div class="sticky top-0 z-50">
                  <span
                     class="absolute left-1/2 top-3 h-1 w-10 -translate-x-1/2 cursor-pointer rounded-lg bg-gray-500"
                     @click="emit('close')"
                  />
               </div>
               <div class="grid grid-cols-1 lg:grid-cols-2">
                  <div
                     v-if="imageUrls.length === 0"
                     class="col-span-full text-center text-gray-500"
                  >
                     No images to display
                  </div>
                  <NuxtImg
                     v-for="(imageUrl, index) in imageUrls"
                     :key="index"
                     :src="imageUrl"
                     :class="{
                        'col-span-2':
                           imageUrls.length % 2 !== 0 &&
                           index === imageUrls.length - 1,
                     }"
                     class="h-64 w-full rounded-md object-cover shadow-lg lg:h-96"
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
