<template>
   <div>
      <FiltersDrawer
         :show-drawer="showFilterDrawer"
         @close="showFilterDrawer = false"
      />
      <div
         class="mx-auto flex w-11/12 flex-col items-center justify-center gap-8"
      >
         <TopBar
            class="mt-10"
            @toggle-filters="showFilterDrawer = !showFilterDrawer"
         />
         <div
            v-if="isLoading"
            class="flex w-full flex-col items-center justify-center gap-4"
         >
            <FlowbiteSkeleton v-for="i in 4" :key="i" />
         </div>
         <PropertyCard
            v-for="property in properties"
            v-else
            :key="property.id"
            :property="property"
            class="w-full md:w-96"
            @open-gallery="openGallery(property.images)"
         />
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

const showDrawer = ref(false);
const currentPropertyImages = ref([]);

const openGallery = (propertyImages) => {
   currentPropertyImages.value = propertyImages;
   showDrawer.value = true;
};
</script>
