<template>
   <section
      class="relative flex h-full flex-col items-center justify-between overflow-hidden"
   >
      <FiltersDrawer
         :show-drawer="showFilterDrawer"
         @close="showFilterDrawer = false"
      />
      <!-- Show the map only when the filter drawer is closed -->
      <div v-show="!showFilterDrawer" class="h-full w-full">
         <TopBar
            class="force-top absolute left-0 right-0 top-0 px-5 pt-10"
            :is-map="true"
            @update:map-center="updateMapCenter"
            @toggle-filters="showFilterDrawer = !showFilterDrawer"
         />
         <FlowbiteToast />

         <ClientOnly fallback-tag="span">
            <MapLibre :map-center="mapCenter" />
            <template #fallback>
               <div
                  class="absolute inset-0 flex h-full w-full items-center justify-center bg-gray-100 dark:bg-gray-800"
               />
            </template>
         </ClientOnly>
      </div>
   </section>
</template>

<script setup>
const mapCenter = ref({ lat: 5.852036, lng: -55.154996 });

function updateMapCenter(lat, lng) {
   mapCenter.value = { lat, lng };
}

const showFilterDrawer = ref(false);

definePageMeta({
   layout: "map",
});
</script>

<style scoped>
.force-top {
   z-index: 9999; /* High z-index to ensure it appears above other elements */
}
</style>
