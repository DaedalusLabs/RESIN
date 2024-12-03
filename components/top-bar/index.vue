<template>
   <div :class="isMap ? 'lg:bg-black lg:bg-opacity-80 lg:p-7' : 'bg-none'">
      <div
         class="flex w-full items-center gap-2 lg:mx-auto lg:w-9/12"
         :class="{ 'lg:justify-center': !isMap, 'lg:justify-between': isMap }"
      >
         <!-- RESIN Logo -->
         <NuxtImg
            src="/images/logos/resin-text.png"
            alt="Logo"
            class="hidden h-10 lg:block"
            :class="{ 'lg:order-first': !isMap }"
         />

         <!-- Search Bar Parent -->
         <div
            class="flex flex-grow gap-3 lg:w-1/2 lg:flex-grow-0"
            :class="{ 'lg:mx-auto': !isMap }"
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
            <FlowbiteSearchbar
               class="flex-grow"
               :query="query"
               @update:query="updateQuery"
            />
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

      <!-- Dropdown -->
      <div class="relative">
         <TopBarDropdown
            :filtered-suggestions="filteredSuggestions"
            :query="query"
            class="absolute left-0 mt-2 w-full lg:w-auto"
            @update:query="updateQuery"
         />
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
