<template>
      <ais-instant-search
      index-name="nostr_listing" :search-client="searchClient"
      :future="{ preserveSharedStateOnUnmount: true }" :initial-ui-state="hasQuery()"
      @state-change="handleStateChange">
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
            <ais-state-results>
                  <template #default="{ results }">
                     <MapLibre :map-center="mapCenter" :results="results.hits" />
                  </template>
               </ais-state-results>
            <template #fallback>
               <div
                  class="absolute inset-0 flex h-full w-full items-center justify-center bg-gray-100 dark:bg-gray-800"
               />
            </template>
         </ClientOnly>
      </div>
   </section>
</ais-instant-search>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { usePropertiesStore } from "~/stores/properties";
import { useSearchStore } from "~/stores/search";
import { useRoute } from "#imports";
import type { SearchClient } from "typesense-instantsearch-adapter";

const route = useRoute();
const propertiesStore = usePropertiesStore();

const searchStore = useSearchStore()
propertiesStore.initializeSearch();
const searchClient:SearchClient = propertiesStore.searchClient;

const { t } = useI18n();

useHead({
   title: t('map.title'),
});

const mapCenter = ref({ lat: 15.76, lng: -81.84 });

function updateMapCenter(lat, lng) {
   mapCenter.value = { lat, lng };
}

const showFilterDrawer = ref(false);

function hasQuery() {
   const searchQuery = route.query.q;
   if (searchQuery && typeof searchQuery === 'string') {
      searchStore.updateSearchState({ nostr_listing: { query: searchQuery } });
   }

   return searchStore.searchState;
}

onMounted(() => {
   // Check for query parameter and update search state

      
   //    searchClient.search({
   //       q: searchQuery,
   // });
   
});

const handleStateChange = ({uiState, setUiState}) => {
   console.log('handleStateChange', uiState)
  searchStore.updateSearchState(uiState)
  setUiState(uiState);
}

definePageMeta({
   layout: "map",
});
</script>

<style scoped>
.force-top {
   z-index: 9999; /* High z-index to ensure it appears above other elements */
}

.ais-InstantSearch, .ais-StateResults {
   @apply h-full;
}
</style>
