<template>
   <ais-refinement-list
      :attribute="attribute"
      :sort-by="['name']"
      :transform-items="transformItems"
      :limit="7"
   >
      <template #default="{ items, refine }">
         <FiltersCheckboxGroup
            :id="`${attribute}Refinement`"
            :label="label"
            value-property="value"
            :options="items"
            :model-value="items"
            :capitalize-words="capitalizeWords"
            @update:model-value="(event) => refine(event.currentTarget.value)"
         />
      </template>
   </ais-refinement-list>
</template>

<script setup>
import { computed } from "vue";
import { useSearchStore } from "@/stores/search";

const props = defineProps({
   attribute: {
      type: String,
      required: true,
   },
   label: {
      type: String,
      required: true,
   },
   capitalizeWords: {
      type: Boolean,
      required: false,
      default: false,
   },
});

const searchStore = useSearchStore();

const transformItems = computed(() => (items) => {
   const storedRefinements = searchStore.refinements[props.attribute] || [];
   return items.map((item) => ({
      ...item,
      isRefined: storedRefinements.includes(item.value) || item.isRefined,
   }));
});
</script>
