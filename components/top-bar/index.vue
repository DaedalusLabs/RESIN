<template>
   <div :class="isMap ? 'lg:bg-black lg:bg-opacity-80 lg:p-7' : 'bg-none'">
      <div
         class="flex w-full items-center gap-2"
         :class="isMap ? 'lg:justify-between' : 'lg:justify-center'"
      >
         <!-- RESIN Logo -->
         <NuxtImg
            src="/images/logos/resin-text.png"
            alt="Logo"
            class="hidden h-10 xl:block"
            :class="isMap ? '' : 'absolute bottom-2 left-0'"
         />

         <!-- Search Bar Parent -->
         <div
            class="relative flex flex-grow gap-3"
            :class="isMap ? '' : 'justify-center'"
         >
            <NuxtLink
               :to="localePath('map')"
               @click="propertiesStore.resetLocations()"
            >
               <FlowbiteIconButton
                  :icon="showListIcon ? 'rows' : 'map'"
                  description="view map"
               />
            </NuxtLink>
            <!-- Search Bar -->
            <div class="relative max-w-xl flex-grow">
               <FlowbiteSearchbar
                  class="w-full"
                  :query="query"
                  @update:query="updateQuery"
               />
               <!-- Dropdown -->
               <TopBarDropdown
                  :filtered-suggestions="filteredSuggestions"
                  :query="query"
                  class="absolute left-0 mt-2 w-full"
                  @update:query="updateQuery"
               />
            </div>
            <FlowbiteIconButton
               icon="filter"
               description="filter properties"
               @click="emits('toggle-filters')"
            />
         </div>

         <!-- View Properties Button (conditionally rendered) -->
         <NuxtLink
            v-if="isMap"
            class="force-top absolute bottom-20 hidden lg:relative lg:bottom-0 lg:block lg:h-full"
            :to="localePath('properties')"
         >
            <FlowbiteButton
               :text="`View ${visibleLocationsAmount} properties`"
               class="rounded bg-resin-500 px-4 py-2 text-white hover:bg-resin-600 lg:py-4"
            />
         </NuxtLink>
      </div>
   </div>
</template>

<script setup>
const propertiesStore = usePropertiesStore();
const route = useRoute();
const showListIcon = ref(route.fullPath.includes("map"));

const query = ref("");
const mapCenter = ref(null);
const visibleLocationsAmount = ref(propertiesStore.filteredProperties.length);
const suggestions = propertiesStore.properties;

const emits = defineEmits([
   "update:map-center",
   "toggle-filters",
   "update:property-list",
]);

const filteredSuggestions = computed(() => {
   if (!query.value) return [];
   return suggestions.filter(
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

function updateQuery(newQuery, lat, lng) {
   query.value = newQuery;
   if (lat !== undefined && lng !== undefined) {
      mapCenter.value = { lat, lng };
      emits("update:map-center", lat, lng);
   }
   // remove ',' from query
   propertiesStore.filterLocations(query.value.replace(/,/g, ""));
}

defineProps({
   isMap: {
      type: Boolean,
      required: false,
      default: false,
   },
});
</script>
