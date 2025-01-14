<script setup>
import { PhX } from "@phosphor-icons/vue";
import BlurhashCanvas from "./BlurhashCanvas.vue";

const props = defineProps({
   showDrawer: {
      type: Boolean,
      default: false,
   },
   images: {
      type: Array,
      required: true,
      default: () => [],
   },
});

const imageLoaded = ref({});

const handleImageLoad = (index) => {
   imageLoaded.value[index] = true;
};

const getDefaultImage = (index) => {
   const imageSet = props.images[index]?.files;
   if (!imageSet) return null;

   // Find the largest image for default display
   const largeImage = imageSet.find((file) => file.width === 1280);
   return largeImage?.url || imageSet[0]?.url;
};

const getSrcSet = (index) => {
   const imageSet = props.images[index]?.files;
   if (!imageSet) return "";

   return imageSet.map((file) => `${file.url} ${file.width}w`).join(", ");
};

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
               'h-[75vh] w-full bg-white lg:self-end',
               { 'overflow-y-auto': images.length > 1 },
            ]"
         >
            <div class="sticky top-0 z-50">
               <!-- Drag Handle -->
               <span
                  class="absolute left-1/2 top-3 h-1 w-10 -translate-x-1/2 cursor-pointer rounded-full bg-pirate-950 lg:h-1.5 lg:w-24"
                  @click="emit('close')"
               />
               <!-- X Close Button  -->
               <button
                  v-if="images.length > 0"
                  class="text-pirate-850 absolute right-12 top-12 hidden h-14 w-14 items-center justify-center rounded-full border-2 bg-white hover:border-resin-500 lg:flex"
                  @click="emit('close')"
               >
                  <PhX :size="22" weight="bold" />
               </button>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-2">
               <div
                  v-if="images.length === 0"
                  class="col-span-full flex h-full items-center justify-center text-gray-500"
               >
                  No images to display
               </div>
               <div
                  v-for="(image, index) in images"
                  :key="index"
                  :class="{
                     'col-span-2':
                        images.length % 2 !== 0 && index === images.length - 1,
                  }"
                  class="relative h-72 lg:h-[75vh]"
               >
                  <div v-if="image.blurhash" class="absolute inset-0">
                     <BlurhashCanvas
                        :hash="image.blurhash"
                        :width="320"
                        :height="240"
                        class="h-full w-full"
                        :style="{
                           display: imageLoaded[index] ? 'none' : 'block',
                        }"
                     />
                  </div>
                  <NuxtImg
                     :src="getDefaultImage(index)"
                     :srcset="getSrcSet(index)"
                     :sizes="'(max-width: 1024px) 100vw, 50vw'"
                     :alt="`Property image ${index}`"
                     class="h-full w-full rounded-md object-cover object-center shadow-lg"
                     loading="lazy"
                     @load="handleImageLoad(index)"
                  />
               </div>
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
