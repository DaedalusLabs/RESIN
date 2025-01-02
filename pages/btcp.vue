<template>
   <div>
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
const propertiesStore = usePropertiesStore();
propertiesStore.initializeSearch();
const searchClient = propertiesStore.searchClient;

const query = ref("");
const searches = ref([propertiesStore.searches.reverse()]);
const viewedProperties = ref([propertiesStore.viewedLocations.reverse()]);

definePageMeta({
   layout: "white",
   middleware: ['auth']

});

const filteredSuggestions = computed(() => {
   if (!query.value) return [];
   return propertiesStore.properties.filter(
      (suggestion) =>
         suggestion.location.address.city.toLowerCase().includes(query.value) ||
         suggestion.location.address.street
            .toLowerCase()
            .includes(query.value) ||
         suggestion.location.address.country
            .toLowerCase()
            .includes(query.value),
   );
});

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


function searchProperties(search) {
   query.value = search;
   const property = propertiesStore.findPropertyBySearchQuery(search);
   if (!property) return;
   updateQuery(
      search,
      property.location.coordinates[1],
      property.location.coordinates[0],
   );
}
</script>

<style scoped>
::-webkit-scrollbar {
   display: none;
}
</style>
