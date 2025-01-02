<template>
   <FlowbiteDrawer :is-open="showDrawer" slide-from="bottom">
      <template #title>

         <div>
            <nav class="mb-8 flex w-full justify-between">
               <button @click="emit('close')">
                  <PhX :size="24" />
               </button>
               <ais-clear-refinements>
                  <template v-slot="{ canRefine, refine, createURL }">
                     <button class="text-base font-normal text-resin-500 hover:underline" @click.prevent="refine"
                        v-if="canRefine">
                        Clear
                     </button>
                  </template>
                  <template v-slot:resetLabel>&nbsp;</template>

               </ais-clear-refinements>

            </nav>
            <h2 class="flex w-full items-center gap-2 justify-self-start text-2xl">
               Filters
               <ais-current-refinements>
                  <template v-slot="{ items, createURL }">
                     <span
                        class="flex h-6 w-6 items-center justify-center rounded-full bg-resin-500 text-sm text-white">
                        {{ items.length }}
                     </span>
                  </template>
               </ais-current-refinements>

            </h2>
            <hr class="my-4" />
         </div>
      </template>

      <template #content>
         <section class="p-4">
            <FiltersRefinementList attribute="resin-type" label="Type"></FiltersRefinementList>

            <FiltersRefinementList attribute="location.city" label="City"></FiltersRefinementList>
            <FiltersRefinementList attribute="location.country" label="Country"></FiltersRefinementList>
            <FiltersRefinementList attribute="location.district" label="District"></FiltersRefinementList>

            <FiltersRefinementRangeNumber attribute="price" label="Price" unit="€" />
            <FiltersRefinementRangeNumber attribute="property.bedrooms" label="Bedrooms" />
            <FiltersRefinementRangeNumber attribute="property.size" label="Area" unit="m²" />

            <div class="p-4">
               <div class="mb-4 p-4">
                  <ais-state-results>
                     <template v-slot="{ state: { query }, results: { hits } }">
                        <button class="w-full rounded-lg bg-resin-500 px-4 py-2 text-white hover:bg-resin-600"
                           @click="applyFilters">
                           Show {{ hits.length }} results
                        </button>
                     </template>
                  </ais-state-results>
               </div>

            </div>
         </section>

      </template>

   </FlowbiteDrawer>
</template>

<script setup>
import { PhX } from "@phosphor-icons/vue";
import { useFiltersStore } from "@/stores/filters";
import { storeToRefs } from 'pinia'
import { useSearchStore } from '@/stores/search'

const searchStore = useSearchStore()
const { refinements } = storeToRefs(searchStore)

// Watch for refinement changes
watch(() => refinements.value, (newRefinements) => {
  searchStore.updateRefinements(newRefinements)
}, { deep: true })



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
};
</script>
<style lang="postcss">
.filter-title {
   @apply mb-2 block text-base font-semibold text-gray-900;
}
</style>
