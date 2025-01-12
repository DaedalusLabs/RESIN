<template>
   <div v-if="property" class="overflow-hidden rounded-2xl bg-white shadow-lg">
      <div class="relative">
         <FlowbiteCarousel
            :items="property.images"
            :blurhash="property.blurhash"
            :class="[
               'z-0 w-full object-cover',
               compact ? 'h-32' : 'h-48 md:h-48',
            ]"
         />
         <button
            v-if="showMediaIcon && !compact"
            class="absolute right-4 top-4 z-10 flex h-10 w-10 items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
            @click="openGallery"
         >
            <PhImages :size="20" />
         </button>

         <span
            v-if="property['resin-type'] && !compact"
            class="absolute bottom-4 right-4 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md"
         >
            {{
               $t(
                  `property.types.${property["resin-type"]
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
      <div :class="['flex flex-col gap-2 text-left', compact ? 'p-2' : 'p-4']">
         <div class="flex items-center justify-between">
            <h3
               :class="[
                  'font-bold text-resin-500',
                  compact ? 'text-base' : 'text-lg',
               ]"
            >
               {{ property.title }}
            </h3>
            <PhHeartStraight
               v-if="!compact"
               :size="21"
               :class="{ 'text-resin-500': isFavorite }"
               class="text-pirate-200 hover:cursor-pointer"
               :weight="isFavorite ? 'fill' : 'regular'"
               @click="toggleFavorite"
            />
         </div>
         <p class="text-gray-600" :class="{ 'text-sm': compact }">
            {{ property.location.city }},
            {{ property.location.country }}
         </p>
         <div class="flex items-center justify-between">
            <p
               v-if="property['resin-type'] !== 'Buy Now'"
               :class="[
                  'font-bold text-gray-800',
                  compact ? 'text-xs' : 'text-sm',
               ]"
            >
               ${{ property.price.toLocaleString() }}
               {{ $t("property.card.perMonth") }}
            </p>
            <p
               v-else
               :class="[
                  'font-bold text-gray-800',
                  compact ? 'text-xs' : 'text-sm',
               ]"
            >
               ${{ property.price?.toLocaleString() }}
            </p>
            <button
               v-if="compact"
               class="text-resin-500 hover:text-resin-600"
               @click="openDetails"
            >
               <PhArrowRight :size="16" />
            </button>
         </div>
      </div>
      <div
         v-if="!compact"
         :class="['flex items-center justify-between pt-0', 'p-4 text-sm']"
      >
         <div class="flex items-center gap-4">
            <p class="flex items-center gap-1">
               <PhRuler :size="20" class="inline" />
               <span class="text-gray-500">
                  {{ property.property.size }}
                  {{ $t("property.card.size.squareMeters") }}
               </span>
            </p>
            <p class="flex items-center gap-1">
               <PhBed :size="20" class="inline" />
               <span class="text-gray-500">
                  {{ property.property.bedrooms }}
                  {{ $t("property.card.beds") }}
               </span>
            </p>
         </div>
         <FlowbiteButton :text="$t('details')" size="sm" @click="openDetails" />
      </div>
   </div>
</template>

<script setup>
import {
   PhBed,
   PhRuler,
   PhHeartStraight,
   PhImages,
   PhArrowRight,
} from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";
const propertiesStore = usePropertiesStore();

const toggleFavorite = () => {
   propertiesStore.toggleFavorite(props.property.id);
};

const props = defineProps({
   property: {
      type: Object,
      required: true,
   },
   showMediaIcon: {
      type: Boolean,
      default: true,
   },
   compact: {
      type: Boolean,
      default: false,
   },
});

const isFavorite = computed(() =>
   propertiesStore.isFavorite(props.property.id),
);

const emit = defineEmits(["open-gallery"]);

const openGallery = () => {
   emit("open-gallery", props.imageUrls);
};

const openDetails = () => {
   const localeRoute = useLocaleRoute();
   const route = localeRoute({
      name: "properties-id",
      params: { id: props.property.id.toString() },
   });
   if (route) {
      return navigateTo(route.fullPath);
   }
};
</script>
