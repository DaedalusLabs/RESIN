<template>
   <div id="animation-carousel" class="relative w-full" data-carousel="static">
      <!-- Carousel wrapper -->
      <div class="relative h-full w-full overflow-hidden">
         <div
            v-for="(item, index) in items"
            :key="index"
            :class="['duration-200 ease-linear', { hidden: index !== 0 }]"
            :data-carousel-item="index === 0 ? 'active' : ''"
         >
            <div v-if="blurhash" class="absolute inset-0">
               <BlurhashCanvas
                  :hash="blurhash"
                  :width="320"
                  :height="240"
                  class="h-full w-full"
                  :style="{ display: imageLoaded[index] ? 'none' : 'block' }"
               />
            </div>
            <NuxtImg
               :src="getDefaultImage(item)"
               :srcset="getSrcSet(item)"
               :sizes="sizes"
               class="absolute inset-0 h-full w-full object-cover"
               :alt="`Property image ${index}`"
               @load="handleImageLoad(index)"
            />
         </div>
      </div>
      <!-- Slider controls -->
      <button
         type="button"
         class="group absolute start-0 top-0 z-30 flex h-full cursor-pointer items-center justify-center px-4 focus:outline-none"
         data-carousel-prev
      >
         <span
            class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-white/30 group-hover:bg-white/50 group-focus:outline-none group-focus:ring-4 group-focus:ring-white dark:bg-gray-800/30 dark:group-hover:bg-gray-800/60 dark:group-focus:ring-gray-800/70"
         >
            <svg
               class="h-4 w-4 text-white dark:text-gray-800 rtl:rotate-180"
               aria-hidden="true"
               xmlns="http://www.w3.org/2000/svg"
               fill="none"
               viewBox="0 0 6 10"
            >
               <path
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M5 1 1 5l4 4"
               />
            </svg>
            <span class="sr-only">Previous</span>
         </span>
      </button>
      <button
         type="button"
         class="group absolute end-0 top-0 z-30 flex h-full cursor-pointer items-center justify-center px-4 focus:outline-none"
         data-carousel-next
      >
         <span
            class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-white/30 group-hover:bg-white/50 group-focus:outline-none group-focus:ring-4 group-focus:ring-white dark:bg-gray-800/30 dark:group-hover:bg-gray-800/60 dark:group-focus:ring-gray-800/70"
         >
            <svg
               class="h-4 w-4 text-white dark:text-gray-800 rtl:rotate-180"
               aria-hidden="true"
               xmlns="http://www.w3.org/2000/svg"
               fill="none"
               viewBox="0 0 6 10"
            >
               <path
                  stroke="currentColor"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="m1 9 4-4-4-4"
               />
            </svg>
            <span class="sr-only">Next</span>
         </span>
      </button>
   </div>
</template>

<script setup>
import { useFlowbite } from "~/composables/useFlowbite";
import BlurhashCanvas from './BlurhashCanvas.vue';

// initialize components based on data attribute selectors
onMounted(() => {
   useFlowbite(() => {
      initFlowbite();
   });
});

const imageLoaded = ref({});

const handleImageLoad = (index) => {
   imageLoaded.value[index] = true;
};

const getDefaultImage = (item) => {
   // If item is a string (legacy support), return it directly
   if (typeof item === 'string') return item;
   
   // If item is a thumbnail set, find the medium size or return first
   if (Array.isArray(item)) {
      const mediumThumbnail = item.find(thumb => thumb.width === 600);
      return mediumThumbnail?.url || item[0]?.url;
   }
   
   return '';
};

const getSrcSet = (item) => {
   // If item is a string (legacy support) or not an array, return empty srcset
   if (typeof item === 'string' || !Array.isArray(item)) return '';
   
   // If item is a thumbnail set, create srcset
   return item.map(thumb => `${thumb.url} ${thumb.width}w`).join(', ');
};

defineProps({
   items: {
      type: Array,
      required: true,
   },
   blurhash: {
      type: String,
      default: null,
   },
   sizes: {
      type: String,
      default: '100vw',
   }
});
</script>
