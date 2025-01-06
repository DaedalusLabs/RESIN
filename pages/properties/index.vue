<template>
   <div class="mx-auto mb-[5vh] w-11/12">
      <ais-instant-search
         index-name="nostr_listing" :search-client="searchClient"
         :future="{ preserveSharedStateOnUnmount: true }" :initial-ui-state="searchStore.searchState"
         @state-change="handleStateChange">
     
         <KeepAlive>
            <FiltersDrawer :show-drawer="showFilterDrawer" @close="showFilterDrawer = false" />
         </KeepAlive>
         <div class="mx-auto flex flex-col items-center justify-center gap-8">
            <TopBar class="relative mt-10 w-full" @toggle-filters="showFilterDrawer = !showFilterDrawer" />
            <div v-if="isLoading" class="grid w-full justify-items-stretch gap-6 lg:grid-cols-2 xl:grid-cols-3">
               </div>
               <div
            v-else
            class="w-full"
         >
               <ais-infinite-hits :show-previous="false">

               <template
               #default="{
               items,
            }">
                  <div v-if="items.length === 0" class="text-center text-white">
                     No properties to display
                  </div>

                  <PropertyCard
v-for="property in items" :key="property.id" :property="property"
                  @open-gallery="openGallery(property.images)" />
               </template>
               </ais-infinite-hits>
            </div>

        
            <BackgroundOverlay :show="showDrawer" @close="showDrawer = false" />

            <FlowbiteImageDrawer
:show-drawer="showDrawer" :image-urls="currentPropertyImages"
               @close="showDrawer = false" />
         </div>
      </ais-instant-search>

   </div>
</template>

<script setup>
import { ref, onMounted, watchEffect } from "vue";
import { usePropertiesStore } from "~/stores/properties";
import { useSearchStore } from "~/stores/search";
import { useRoute } from "#imports";

const route = useRoute();
const propertiesStore = usePropertiesStore();
const searchStore = useSearchStore();

const handleStateChange = ({uiState, setUiState}) => {
  searchStore.updateSearchState(uiState)
  setUiState(uiState);
}

const isLoading = ref(true);
const properties = ref([]);
const showFilterDrawer = ref(false);
propertiesStore.initializeSearch();
const searchClient = propertiesStore.searchClient;

onMounted(async () => {
   isLoading.value = false;
   // Check for query parameter and update search state
   const searchQuery = route.query.q;
   if (searchQuery && typeof searchQuery === 'string') {
      console.log(route.query.q);
      searchStore.updateRefinements({ query: searchQuery });
   }
});

watchEffect(() => {
   properties.value = propertiesStore.filteredProperties;
});

const showDrawer = ref(false);
const currentPropertyImages = ref([]);

const openGallery = (propertyImages) => {
   currentPropertyImages.value = propertyImages;
   showDrawer.value = true;
};
</script>

<style scoped>
   .ais-InfiniteHits {
      @apply grid w-full justify-items-stretch gap-6 lg:grid-cols-2 xl:grid-cols-3;
   }
</style>