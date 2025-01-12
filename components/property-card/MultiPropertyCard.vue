<template>
   <div
      v-if="properties.length"
      class="overflow-hidden rounded-2xl bg-white shadow-lg"
   >
      <div class="relative">
         <FlowbiteCarousel
            :items="properties[currentIndex].thumbnails"
            :blurhash="properties[currentIndex].blurhash"
            class="z-0 h-48 w-full object-cover"
         />

         <span
            v-if="properties[currentIndex]['resin-type']"
            class="absolute bottom-4 right-4 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md"
         >
            {{
               $t(
                  `property.types.${properties[currentIndex]["resin-type"]
                     .split(" ")
                     .map((word, index) =>
                        index === 0
                           ? word.toLowerCase()
                           : word.charAt(0).toUpperCase() +
                             word.slice(1).toLowerCase(),
                     )
                     .join("")}`,
               )
            }}
         </span>
      </div>

      <transition :name="transitionName" mode="out-in">
         <div :key="currentIndex">
            <div class="flex flex-col gap-2 p-4 text-left">
               <div class="flex items-center justify-between">
                  <h3 class="text-lg font-bold text-resin-500">
                     {{ properties[currentIndex].title }}
                  </h3>
                  <PhHeartStraight
                     :size="21"
                     :class="{ 'text-resin-500': isFavorite }"
                     class="text-pirate-200 hover:cursor-pointer"
                     :weight="isFavorite ? 'fill' : 'regular'"
                     @click="toggleFavorite"
                  />
               </div>
               <p class="text-gray-600">
                  {{ properties[currentIndex].location.city }},
                  {{ properties[currentIndex].location.country }}
               </p>
               <div class="flex items-center justify-between">
                  <p
                     v-if="properties[currentIndex]['resin-type'] !== 'Buy Now'"
                     class="text-sm font-bold text-gray-800"
                  >
                     ${{ properties[currentIndex].price.toLocaleString() }}
                     {{ $t("property.card.perMonth") }}
                  </p>
                  <p v-else class="text-sm font-bold text-gray-800">
                     ${{ properties[currentIndex].price?.toLocaleString() }}
                  </p>
               </div>
            </div>

            <div class="flex items-center justify-between p-4 pt-0 text-sm">
               <div class="flex items-center gap-4">
                  <p class="flex items-center gap-1">
                     <PhRuler :size="20" class="inline" />
                     <span class="text-gray-500">
                        {{ properties[currentIndex].property.size }}
                        {{ $t("property.card.size.squareMeters") }}
                     </span>
                  </p>
                  <p class="flex items-center gap-1">
                     <PhBed :size="20" class="inline" />
                     <span class="text-gray-500">
                        {{ properties[currentIndex].property.bedrooms }}
                        {{ $t("property.card.beds") }}
                     </span>
                  </p>
               </div>
               <FlowbiteButton
                  :text="$t('details')"
                  size="sm"
                  @click="openDetails"
               />
            </div>
         </div>
      </transition>

      <!-- Property Navigation -->
      <div
         class="relative flex items-center justify-between border-t border-gray-100 px-4 py-3"
      >
         <button
            class="flex items-center gap-2 text-sm text-gray-600 hover:text-resin-500 disabled:opacity-50 disabled:hover:text-gray-600"
            :disabled="currentIndex === 0"
            @click="previousProperty"
         >
            <PhCaretLeft :size="20" />
            {{ $t("previous") }}
         </button>
         <span
            class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-sm font-medium text-gray-600"
         >
            {{
               $t("property.card.countof", {
                  count: currentIndex + 1,
                  total: properties.length,
               })
            }}
         </span>
         <button
            class="flex items-center gap-2 text-sm text-gray-600 hover:text-resin-500 disabled:opacity-50 disabled:hover:text-gray-600"
            :disabled="currentIndex === properties.length - 1"
            @click="nextProperty"
         >
            {{ $t("next") }}
            <PhCaretRight :size="20" />
         </button>
      </div>
   </div>
</template>

<script setup>
import {
   PhBed,
   PhRuler,
   PhHeartStraight,
   PhCaretLeft,
   PhCaretRight,
} from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";

const propertiesStore = usePropertiesStore();
const currentIndex = ref(0);
const transitionName = ref("slide-right");

const props = defineProps({
   properties: {
      type: Array,
      required: true,
   },
});

const isFavorite = computed(() =>
   propertiesStore.isFavorite(props.properties[currentIndex.value].id),
);

const toggleFavorite = () => {
   propertiesStore.toggleFavorite(props.properties[currentIndex.value].id);
};

const nextProperty = () => {
   if (currentIndex.value < props.properties.length - 1) {
      transitionName.value = "slide-left";
      currentIndex.value++;
   }
};

const previousProperty = () => {
   if (currentIndex.value > 0) {
      transitionName.value = "slide-right";
      currentIndex.value--;
   }
};

const openDetails = () => {
   const localeRoute = useLocaleRoute();
   const route = localeRoute({
      name: "properties-id",
      params: { id: props.properties[currentIndex.value].id.toString() },
   });
   if (route) {
      return navigateTo(route.fullPath);
   }
};

const thumbnailUrls = computed(() => {
   const property = props.properties[currentIndex.value];
   if (!property.thumbnails) return property.images;
   return property.thumbnails.map(thumbnailSet => {
      // Find the thumbnail with width 600 for default display
      const mediumThumbnail = thumbnailSet.find(thumb => thumb.width === 600);
      return mediumThumbnail ? mediumThumbnail.url : thumbnailSet[0].url;
   });
});
</script>

<style scoped>
.slide-left-enter-active,
.slide-right-enter-active,
.slide-left-leave-active,
.slide-right-leave-active {
   transition: all 0.2s ease;
}

.slide-left-enter-from {
   opacity: 0;
   transform: translateX(20px);
}

.slide-left-leave-to {
   opacity: 0;
   transform: translateX(-20px);
}

.slide-right-enter-from {
   opacity: 0;
   transform: translateX(-20px);
}

.slide-right-leave-to {
   opacity: 0;
   transform: translateX(20px);
}
</style>
