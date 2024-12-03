<template>
   <div class="mx-auto w-9/12">
      <FiltersDrawer
         :show-drawer="showFilterDrawer"
         @close="showFilterDrawer = false"
      />
      <div class="mx-auto flex flex-col items-center justify-center gap-8">
         <TopBar
            class="relative mt-10 w-full"
            @toggle-filters="showFilterDrawer = !showFilterDrawer"
         />
         <div
            class="grid w-full justify-items-stretch gap-6 md:grid-cols-2 lg:grid-cols-3"
         >
            <div
               v-if="isLoading"
               class="flex w-full flex-col items-center justify-center gap-4"
            >
               <FlowbiteSkeleton v-for="i in 4" :key="i" />
            </div>
            <div v-if="properties.length === 0" class="text-center text-white">
               No properties to display
            </div>
            <PropertyCard
               v-for="property in properties"
               v-else
               :key="property.id"
               :property="property"
               @open-gallery="openGallery(property.images)"
            />
         </div>
      </div>
      <FlowbiteImageDrawer
         :show-drawer="showDrawer"
         :image-urls="currentPropertyImages"
         @close="showDrawer = false"
      />
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
const isLoading = ref(true);
const properties = ref([]);
const showFilterDrawer = ref(false);

onMounted(() => {
   const propertiesStore = usePropertiesStore();
   properties.value = propertiesStore.filteredProperties;
   setTimeout(() => {
      isLoading.value = false;
   }, 1000);
});

watchEffect(() => {
   const propertiesStore = usePropertiesStore();
   properties.value = propertiesStore.filteredProperties;
});

const showDrawer = ref(false);
const currentPropertyImages = ref([]);

const openGallery = (propertyImages) => {
   currentPropertyImages.value = propertyImages;
   showDrawer.value = true;
};
</script>
