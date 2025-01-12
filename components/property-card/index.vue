<template>
   <div v-if="property" class="overflow-hidden rounded-2xl bg-white shadow-lg">
      <div class="relative">
         <FlowbiteCarousel
            :items="property.images.map((img) => img.files)"
            :blurhash="property.images[0]?.blurhash"
            :class="[
               'z-0 w-full object-cover',
               compact ? 'h-32' : 'h-48 md:h-48',
            ]"
         />
         <button
            v-if="showMediaIcon && !compact"
            class="z-1 absolute right-4 top-4 flex h-10 w-10 items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
            title="Open gallery"
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
               :class="[
                  { 'text-resin-500': isFavorite },
                  isAuthenticated
                     ? 'hover:cursor-pointer'
                     : 'cursor-not-allowed opacity-50',
               ]"
               class="text-pirate-200"
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
         <NuxtLinkLocale :to="propertyLink">
            <FlowbiteButton :text="$t('details')" size="sm" />
         </NuxtLinkLocale>
      </div>
   </div>
</template>

<script setup lang="ts">
import {
   PhBed,
   PhRuler,
   PhHeartStraight,
   PhImages,
   PhArrowRight,
} from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";
import { useNostrStore } from "~/stores/nostr";
import type { Property } from "~/types/property";

const props = defineProps({
   property: {
      type: Object as PropType<Property>,
      required: true,
   },
   compact: {
      type: Boolean,
      default: false,
   },
   showMediaIcon: {
      type: Boolean,
      default: true,
   },
});

const emit = defineEmits(["openGallery"]);

const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();

const isAuthenticated = computed(() => nostrStore.authenticated);
const isFavorite = computed(() =>
   props.property ? propertiesStore.isFavorite(props.property.id) : false,
);

const propertyLink = computed(() => {
   return props.property.slug
      ? `/properties/${props.property.id}/${props.property.slug}`
      : `/properties/${props.property.id}`;
});

const toggleFavorite = () => {
   if (isAuthenticated.value) {
      propertiesStore.toggleFavorite(props.property.id);
   }
};

const openGallery = () => {
   emit("openGallery", props.property.images);
};

const openDetails = () => {
   const localeRoute = useLocaleRoute();
   const route = localeRoute({
      name: "properties-id-slug",
      params: {
         id: props.property.id.toString(),
         slug: props.property.slug || props.property.id,
      },
   });
   if (route) {
      return navigateTo(route.fullPath);
   }
};
</script>
