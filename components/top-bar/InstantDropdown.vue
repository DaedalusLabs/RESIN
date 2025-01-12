<template>
   <div
      v-if="
         filteredSuggestions[0] && filteredSuggestions[0].hits.length && query
      "
      ref="dropdownContainer"
      class="absolute z-10 mt-1 w-full max-w-full rounded-xl bg-black py-4 shadow-lg"
   >
      <ul v-for="index in filteredSuggestions" :key="index.indexId">
         <p class="pb-2 pl-4 text-sm font-semibold text-white">Places</p>
         <li
            v-for="(suggestion, index) in index.hits"
            :key="index.id"
            class="flex cursor-pointer items-center overflow-hidden px-4 py-2 text-sm text-gray-400"
            @click="selectSuggestion(suggestion)"
         >
            <PhMagnifyingGlass :size="12" class="mr-2 flex-shrink-0" />
            <span class="w-full truncate font-semibold hover:text-resin-500">
               <span v-html="highlightQuery(suggestion)" />
            </span>
         </li>
      </ul>
   </div>
</template>

<script setup>
import { PhMagnifyingGlass } from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";
import { ref } from "vue";
import { onClickOutside } from "@vueuse/core";

const propertiesStore = usePropertiesStore();
const dropdownContainer = ref(null);

const props = defineProps({
   filteredSuggestions: {
      type: Array,
      required: true,
   },
   query: {
      type: String,
      required: true,
   },
});

const emit = defineEmits(["update:query", "close"]);

function selectSuggestion(suggestion) {
   const fullAddress = `${suggestion.location.street}, ${suggestion.location.city}, ${suggestion.location.country}`;
   emit(
      "update:query",
      fullAddress,
      suggestion.location.coordinates[1],
      suggestion.location.coordinates[0],
   );
   propertiesStore.addSearch(props.query);
}

function highlightQuery(suggestion) {
   const fullAddress = `${suggestion.location.street}, ${suggestion.location.city}, ${suggestion.location.country}`;
   const regex = new RegExp(`(${props.query})`, "gi");
   return fullAddress.replace(regex, '<span class="text-resin-500">$1</span>');
}

onClickOutside(dropdownContainer, (event) => {
   // Check if the click was on the search input or its container
   const searchContainer = document
      .querySelector("#default-search")
      ?.closest(".relative");
   if (!searchContainer?.contains(event.target)) {
      emit("close");
   }
});
</script>
