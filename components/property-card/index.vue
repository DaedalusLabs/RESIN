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
            class="absolute bottom-4 right-4 z-10 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md hover:border-white hover:bg-resin-500 hover:text-white"
         >
            {{ property?.isBitcasaHome ? "For Sale" : "Rent to Own" }}
         </span>
      </div>
      <div class="flex flex-col gap-2 p-4">
         <div class="flex items-center justify-between">
            <h3 class="text-lg font-bold text-resin-500">
               {{ property.location.address.street }}
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
            {{ property.location.address.city }},
            {{ property.location.address.country }}
         </p>
         <p
            v-if="!property.isBitcasaHome"
            class="text-sm font-bold text-gray-800"
         >
            ${{ property.pricingDetails.rentPerMonth.toLocaleString() }} per
            month
         </p>
         <p v-else class="text-sm font-bold text-gray-800">
            ${{ property.pricingDetails.propertyPrice.toLocaleString() }}
         </p>
      </div>
      <div class="flex items-center justify-between p-4 pt-0 text-sm">
         <div class="flex items-center gap-4">
            <p class="flex items-center gap-1">
               <PhRuler :size="20" class="inline" />
               <span class="text-gray-500">
                  {{ property.propertyDetails.size.houseSizeM2 }} m²
               </span>
            </p>
            <p class="flex items-center gap-1">
               <PhBed :size="20" class="inline" />
               <span class="text-gray-500">
                  {{ property.propertyDetails.bedrooms }} Beds
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
   isFavorite.value = !isFavorite.value;
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

const isFavorite = ref(propertiesStore.isFavorite(props.property.id));

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
