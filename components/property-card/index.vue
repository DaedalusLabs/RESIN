<template>
   <div class="overflow-hidden rounded-2xl bg-white shadow-lg">
      <div class="relative">
         <FlowbiteCarousel
            :items="property.images"
            class="z-0 h-48 w-full object-cover md:h-48"
         />
         <button
            v-if="showMediaIcon"
            class="absolute right-4 top-4 z-10 flex h-10 w-10 items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
            @click="openGallery"
         >
            <PhImages :size="20" />
         </button>

         <span
            class="absolute bottom-4 right-4 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md hover:border-white hover:bg-resin-500 hover:text-white"
         >
            {{ $t(`property.types.${property['resin-type'].split(' ').map((word, index) => index === 0 ? word.toLowerCase() : word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join('')}`) }}
         </span>
      </div>
      <div class="flex flex-col gap-2 p-4">
         <div class="flex items-center justify-between">
            <h3 class="text-lg font-bold text-resin-500">
               {{ property.title }}
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
            {{ property.location.city }},
            {{ property.location.country }}
         </p>
         <p
            v-if="property['resin-type'] !== 'Buy Now'"
            class="text-sm font-bold text-gray-800"
         >
            ${{ property.price.toLocaleString() }} {{ $t('property.card.perMonth') }}
         </p>
         <p v-else class="text-sm font-bold text-gray-800">
            ${{ property.price?.toLocaleString() }}
         </p>
      </div>
      <div class="flex items-center justify-between p-4 pt-0 text-sm">
         <div class="flex items-center gap-4">
            <p class="flex items-center gap-1">
               <PhRuler :size="20" class="inline" />
               <span class="text-gray-500">
                  {{ property.property.size }} {{ $t('property.card.size.squareMeters') }}
               </span>
            </p>
            <p class="flex items-center gap-1">
               <PhBed :size="20" class="inline" />
               <span class="text-gray-500">
                  {{ property.property.bedrooms }} {{ $t('property.card.beds') }}
               </span>
            </p>
         </div>
         <FlowbiteButton :text="$t('details')" @click="openDetails" />
      </div>
   </div>
</template>

<script setup>
import { PhBed, PhRuler, PhHeartStraight, PhImages } from "@phosphor-icons/vue";
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
});

const isFavorite = computed(() => propertiesStore.isFavorite(props.property.id));

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
