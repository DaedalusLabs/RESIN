<template>
   <FlowbiteDrawer :is-open="showDrawer">
      <template #title>
         <div>
            <nav class="mb-8 flex w-full justify-between">
               <button @click="emit('close')">
                  <PhX :size="24" />
               </button>
               <button
                  class="text-base font-normal text-resin-500 hover:underline"
                  @click="handleReset"
               >
                  Clear
               </button>
            </nav>
            <h2
               class="flex w-full items-center gap-2 justify-self-start text-2xl"
            >
               Filters
               <span class="count-badge">
                  {{ filtersStore.fi }}
               </span>
            </h2>
            <hr class="my-4" />
         </div>
      </template>

      <template #content>
         <div class="p-4">
            <FiltersRangeInput
               id="price"
               label="Price"
               placeholder="0"
               :model-start-value="filtersStore.price.from"
               :model-end-value="filtersStore.price.to"
               @update:model-start-value="
                  (value) =>
                     filtersStore.updateRangeFilter('price', 'from', value)
               "
               @update:model-end-value="
                  (value) =>
                     filtersStore.updateRangeFilter('price', 'to', value)
               "
            />

            <FiltersCheckboxGroup
               id="rentalPeriod"
               label="Rental Period"
               :options="filtersStore.filterOptions.rentalPeriod"
               :model-value="rentalPeriod"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter('rentalPeriod', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="sort"
               label="Sort"
               :options="filtersStore.filterOptions.sort"
               :model-value="filtersStore.sort"
               @update:model-value="
                  (value) => filtersStore.updateCheckboxFilter('sort', value)
               "
            />

            <hr class="my-8" />
            <!-- 
            <FiltersRadioButtonGroup
               :key="'availableSince'"
               v-model="filters.availableSince"
               label="Available since"
               :options="filterOptions.availableSince"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'availability'"
               v-model="filters.availability"
               label="Availability"
               :options="filterOptions.availability"
            />

            <hr class="my-8" />

            <FiltersRangeInput
               id="livingArea"
               v-model="filters.livingArea"
               label="Living Area"
               placeholder="0"
            />

            <hr class="my-8" />

            <FiltersRangeInput
               id="plotArea"
               v-model="filters.plotArea"
               label="Plot Area"
               placeholder="0"
            />

            <hr class="my-8" />

            <FiltersCounter
               id="rooms"
               v-model="filters.rooms"
               label="Rooms"
               placeholder="0"
            />

            <hr class="my-8" />

            <FiltersCounter
               id="bedrooms   "
               v-model="filters.bedrooms"
               label="Bedrooms"
               placeholder="0"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'outdoorSpace'"
               v-model="filters.outdoorSpace"
               label="Outdoor space"
               :options="filterOptions.outdoorSpaces"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'gardenOrientation'"
               v-model="filters.gardenOrientation"
               label="Garden orientation"
               :options="filterOptions.gardenOrientations"
            />

            <hr class="my-8" />

            <FiltersRangeInput
               id="gardenArea"
               v-model="filters.gardenArea"
               label="Garden area"
               placeholder="0"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'constructionType'"
               v-model="filters.constructionType"
               label="Construction type"
               :options="filterOptions.constructionType"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'purpose'"
               v-model="filters.purpose"
               label="Purpose"
               :options="filterOptions.purpose"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'constructionPeriod'"
               v-model="filters.constructionPeriod"
               label="Construction period"
               :options="filterOptions.constructionPeriod"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'locations'"
               v-model="filters.locations"
               label="Locations"
               :options="filterOptions.locations"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'garages'"
               v-model="filters.garages"
               label="Garages"
               :options="filterOptions.garages"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'features'"
               v-model="filters.features"
               label="Features"
               :options="filterOptions.features"
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               :key="'display'"
               v-model="filters.display"
               label="Display"
               :options="filterOptions.display"
            />
            -->
            <div class="mb-4 p-4">
               <button class="button-primary w-full" @click="applyFilters">
                  Show {{ filteredResults }} results
               </button>
            </div>
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhX } from "@phosphor-icons/vue";
import { useFiltersStore } from "@/stores/filters";

const filtersStore = useFiltersStore();
const rentalPeriod = ref(filtersStore.rentalPeriod);
const emit = defineEmits(["close", "apply-filters"]);
const filteredResults = ref(0);

defineProps({
   showDrawer: {
      type: Boolean,
      required: true,
   },
});

const handleReset = () => {
   filtersStore.clearAllFilters();
   rentalPeriod.value = [];
};

const applyFilters = () => {
   const filterState = filtersStore.getFilterState();
   console.log(filterState);
   emit("apply-filters", filterState);
};
</script>
<style lang="postcss">
.filter-title {
   @apply mb-2 block text-base font-semibold text-gray-900;
}

.input-base {
   @apply block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-resin-500 focus:ring-resin-500;
}

.checkbox-base {
   @apply h-4 w-4 rounded border-gray-300 bg-gray-100 text-resin-500 focus:ring-2 focus:ring-resin-500;
}

.counter-button {
   @apply h-11 border border-gray-300 bg-gray-100 p-3 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-100;
}

.counter-input {
   @apply block h-11 w-full border-x-0 border-gray-300 bg-gray-50 py-2.5 text-center text-sm text-gray-900;
}

.count-badge {
   @apply flex h-6 w-6 items-center justify-center rounded-full bg-resin-500 text-sm text-white;
}

.button-primary {
   @apply rounded-lg bg-resin-500 px-4 py-2 text-white hover:bg-resin-600;
}

.radio-base {
   @apply h-4 w-4 border-gray-300 bg-gray-100 text-resin-500 focus:ring-2 focus:ring-resin-500;
}
</style>
