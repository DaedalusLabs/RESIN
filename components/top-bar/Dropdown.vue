<template>
   <div
      v-if="filteredSuggestions.length"
      class="z-top absolute mt-1 flex-1 rounded-xl bg-black py-4 shadow-lg"
   >
      <ul>
         <p class="pb-2 pl-4 text-sm font-semibold text-white">Places</p>
         <li
            v-for="(suggestion, index) in filteredSuggestions"
            :key="index"
            class="flex cursor-pointer items-center px-4 py-2 text-sm text-gray-400"
            @click="selectSuggestion(suggestion)"
         >
            <PhMagnifyingGlass :size="12" class="mr-2 flex-shrink-0" />
            <span class="truncate font-semibold hover:text-resin-500">
               <template
                  v-for="(part, i) in highlightQuery(suggestion)"
                  :key="i"
               >
                  <span :class="{ 'text-resin-500': part.highlight }">
                     {{ part.text }}
                  </span>
               </template>
            </span>
         </li>
      </ul>
   </div>
</template>

<script setup>
import { PhMagnifyingGlass } from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";
const propertiesStore = usePropertiesStore();

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

const emit = defineEmits(["update:query"]);

function selectSuggestion(suggestion) {
   const fullAddress = `${suggestion.location.street}, ${suggestion.location.city}, ${suggestion.location.country}`;
   emit(
      "update:query",
      fullAddress,
      suggestion.location.coordinates.latitude,
      suggestion.location.coordinates.longitude,
   );
   propertiesStore.addSearch(fullAddress);
}

function highlightQuery(suggestion) {
   const fullAddress = `${suggestion.location.street}, ${suggestion.location.city}, ${suggestion.location.country}`;
   const regex = new RegExp(`(${props.query})`, "gi");
   const parts = [];
   let lastIndex = 0;
   let match;

   while ((match = regex.exec(fullAddress)) !== null) {
      if (match.index > lastIndex) {
         parts.push({
            text: fullAddress.slice(lastIndex, match.index),
            highlight: false,
         });
      }
      parts.push({ text: match[0], highlight: true });
      lastIndex = regex.lastIndex;
   }

   if (lastIndex < fullAddress.length) {
      parts.push({ text: fullAddress.slice(lastIndex), highlight: false });
   }

   return parts;
}
</script>
