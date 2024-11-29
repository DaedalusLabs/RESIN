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
               <span
                  class="flex h-6 w-6 items-center justify-center rounded-full bg-resin-500 text-sm text-white"
               >
                  {{ filtersStore.activeFilterCount }}
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
               unit="SRD"
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
               label="Rental period"
               :options="filtersStore.filterOptions.rentalPeriod"
               :model-value="filtersStore.rentalPeriod"
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

            <FiltersRadioButtonGroup
               id="availableSince"
               v-model="filtersStore.availableSince"
               label="Available since"
               :options="filtersStore.filterOptions.availableSince"
               :model-value="filtersStore.availableSince"
               @update:model-value="
                  (value) =>
                     filtersStore.updateSelectFilter('availableSince', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="availability"
               label="Availability"
               :options="filtersStore.filterOptions.availability"
               :model-value="filtersStore.availability"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter('availability', value)
               "
            />

            <hr class="my-8" />

            <FiltersRangeInput
               id="livingArea"
               v-model="filtersStore.livingArea"
               label="Living Area"
               placeholder="0"
               :model-start-value="filtersStore.livingArea.from"
               :model-end-value="filtersStore.livingArea.to"
               unit="m²"
               @update:model-start-value="
                  (value) =>
                     filtersStore.updateRangeFilter('livingArea', 'from', value)
               "
               @update:model-end-value="
                  (value) =>
                     filtersStore.updateRangeFilter('livingArea', 'to', value)
               "
            />

            <hr class="my-8" />

            <FiltersRangeInput
               id="plotArea"
               v-model="filtersStore.plotArea"
               label="Plot Area"
               placeholder="0"
               :model-start-value="filtersStore.plotArea.from"
               :model-end-value="filtersStore.plotArea.to"
               unit="m²"
               @update:model-start-value="
                  (value) =>
                     filtersStore.updateRangeFilter('plotArea', 'from', value)
               "
               @update:model-end-value="
                  (value) =>
                     filtersStore.updateRangeFilter('plotArea', 'to', value)
               "
            />

            <hr class="my-8" />

            <FiltersCounter
               id="rooms"
               v-model="filtersStore.rooms"
               :model-start-value="filtersStore.rooms.from"
               :model-end-value="filtersStore.rooms.to"
               label="Rooms"
               placeholder="0"
               @update:model-start-value="
                  (value) => filtersStore.updateNumberFilter('rooms', value)
               "
            />

            <hr class="my-8" />

            <FiltersCounter
               id="bedrooms"
               v-model="filtersStore.bedrooms"
               label="Bedrooms"
               placeholder="0"
               :model-value="filtersStore.bedrooms"
               @update:model-value="
                  (value) => filtersStore.updateNumberFilter('bedrooms', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="outdoorSpace"
               label="Outdoor space"
               :options="filtersStore.filterOptions.outdoorSpaces"
               :model-value="filtersStore.outdoorSpace"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter('outdoorSpace', value)
               "
            />
            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="gardenOrientation"
               label="Garden orientation"
               :options="filtersStore.filterOptions.gardenOrientations"
               :model-value="filtersStore.gardenOrientation"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter(
                        'gardenOrientation',
                        value,
                     )
               "
            />

            <hr class="my-8" />

            <FiltersRangeInput
               id="gardenArea"
               v-model="filtersStore.gardenArea"
               label="Garden area"
               placeholder="0"
               :model-start-value="filtersStore.gardenArea.from"
               :model-end-value="filtersStore.gardenArea.to"
               unit="m²"
               @update:model-start-value="
                  (value) =>
                     filtersStore.updateRangeFilter('gardenArea', 'from', value)
               "
               @update:model-end-value="
                  (value) =>
                     filtersStore.updateRangeFilter('gardenArea', 'to', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="constructionType"
               label="Construction type"
               :options="filtersStore.filterOptions.constructionType"
               :model-value="filtersStore.constructionType"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter(
                        'constructionType',
                        value,
                     )
               "
            />

            <hr class="my-8" />
            <FiltersCheckboxGroup
               id="purpose"
               label="Purpose"
               :options="filtersStore.filterOptions.purpose"
               :model-value="filtersStore.purpose"
               @update:model-value="
                  (value) => filtersStore.updateCheckboxFilter('purpose', value)
               "
            />
            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="constructionPeriod"
               label="Construction period"
               :options="filtersStore.filterOptions.constructionPeriod"
               :model-value="filtersStore.constructionPeriod"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter(
                        'constructionPeriod',
                        value,
                     )
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="locations"
               label="Locations"
               :options="filtersStore.filterOptions.locations"
               :model-value="filtersStore.locations"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter('locations', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="garages"
               label="Garages"
               :options="filtersStore.filterOptions.garages"
               :model-value="filtersStore.garages"
               @update:model-value="
                  (value) => filtersStore.updateCheckboxFilter('garages', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="features"
               label="Features"
               :options="filtersStore.filterOptions.features"
               :model-value="filtersStore.features"
               @update:model-value="
                  (value) =>
                     filtersStore.updateCheckboxFilter('features', value)
               "
            />

            <hr class="my-8" />

            <FiltersCheckboxGroup
               id="features"
               label="Display"
               :options="filtersStore.filterOptions.display"
               :model-value="filtersStore.display"
               @update:model-value="
                  (value) => filtersStore.updateCheckboxFilter('display', value)
               "
            />

            <div class="mb-4 p-4">
               <button
                  class="w-full rounded-lg bg-resin-500 px-4 py-2 text-white hover:bg-resin-600"
                  @click="applyFilters"
               >
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
};

const applyFilters = () => {
   // TODO: send to api to get filtered results
   emit("close");
   console.log(filtersStore.getFilterState());
};
</script>
<style lang="postcss">
.filter-title {
   @apply mb-2 block text-base font-semibold text-gray-900;
}
</style>
