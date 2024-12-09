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
   <transition name="fade-slide" appear>
      <div
         v-if="showDrawer"
         class="z-top fixed inset-0 flex h-full w-full items-end"
         @click.self="emit('close')"
      >
         <div
            :class="[
               'max-h-[80vh] w-full bg-white lg:self-end dark:bg-gray-800',
               { 'overflow-y-auto': imageUrls.length > 1 },
            ]"
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
                  class="max-h-[50vh] w-full rounded-md object-cover object-center shadow-lg"
                  loading="lazy"
               />
            </div>
         </div>
      </div>
   </transition>
</template>

<style>
.fade-slide-enter-active,
.fade-slide-leave-active {
   transition:
      opacity 0.3s ease,
      transform 0.3s ease;
}

.fade-slide-enter-from,
.fade-slide-leave-to {
   opacity: 0;
   transform: translateY(100%);
}
</style>
