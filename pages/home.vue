<template>
   <ais-instant-search
      index-name="nostr_listing"
      :search-client="searchClient"
      :future="{ preserveSharedStateOnUnmount: true }"
   >
      <section class="mx-auto flex w-10/12 flex-col py-20">
         <NuxtImg
            src="/images/logos/resin-text.png"
            alt="RESIN Logo"
            class="mb-10 w-24"
         />
         <FlowbiteInstantSearchbar
            class="flex-grow"
            :query="query"
            @update:query="updateQuery"
         />

         <h2 class="mb-3 mt-10 text-xl font-bold leading-tight text-pirate-950">
            {{ $t("home.recentSearches.title") }}
         </h2>
         <div class="flex gap-4 overflow-scroll">
            <p v-if="!searches.length" class="text-pirate-950">
               {{ $t("home.recentSearches.noSearches") }}
            </p>
            <HomeRecentSearch
               v-for="search in searches"
               :key="search"
               class="w-auto flex-shrink-0 cursor-pointer"
               :search="search"
               @click="searchPropertiesOnMap(search)"
            />
         </div>
         <h2 class="text-pirate-950leading-tight mb-3 mt-10 text-xl font-bold">
            {{ $t("home.recentlyViewed.title") }}
         </h2>
         <div class="flex gap-4 overflow-scroll">
            <p v-if="!viewedProperties.length" class="text-pirate-950">
               {{ $t("home.recentlyViewed.noProperties") }}
            </p>
            <FavoritesCard
               v-for="property in viewedProperties"
               :key="property"
               class="flex-shrink-0 cursor-pointer"
               :property="property"
            />
         </div>
         <section v-show="propertiesStore.trendingAreas.length">
            <h2
               class="mb-3 mt-10 text-xl font-bold leading-tight text-pirate-950"
            >
               {{ $t("home.trendingAreas.title") }}
            </h2>
            <div class="flex flex-col gap-4">
               <p
                  v-if="!propertiesStore.trendingAreas.length"
                  class="text-pirate-950"
               >
                  {{ $t("home.trendingAreas.noAreas") }}
               </p>
               <HomeAreaCard
                  v-for="area in propertiesStore.trendingAreas"
                  :key="area"
                  class="flex-shrink-0"
                  :area="area"
               />
            </div>
         </section>
      </section>
   </ais-instant-search>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
const propertiesStore = usePropertiesStore();
propertiesStore.init();
const searchClient = propertiesStore.searchClient;

const query = ref("");
const searches = ref([propertiesStore.searches.reverse()]);
const viewedProperties = ref([propertiesStore.viewedLocations.reverse()]);

const { t } = useI18n();
useHead({
   title: t("home.title"),
});

definePageMeta({
   layout: "white",
   middleware: ["auth"],
});

watchEffect(() => {
   searches.value = propertiesStore.searches.reverse();
   viewedProperties.value = propertiesStore.viewedLocations.reverse();
});

function searchPropertiesOnMap(search) {
   const localeRoute = useLocaleRoute();
   const route = localeRoute({
      name: "map",
      query: { q: search },
   });
   if (route) {
      return navigateTo(route.fullPath);
   }
}

onMounted(() => {});

function updateQuery(newQuery, latitude, longitude) {
   query.value = newQuery;
   if ((latitude, longitude)) {
      const localeRoute = useLocaleRoute();
      const long = longitude;
      const lat = latitude;
      const route = localeRoute({
         name: "map",
         query: { lat, lng: long },
      });
      if (route) {
         return navigateTo(route.fullPath);
      }
   }
}
</script>

<style scoped>
::-webkit-scrollbar {
   display: none;
}
</style>
