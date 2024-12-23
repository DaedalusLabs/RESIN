<template>
   <div class="mt-12 w-3/4" v-if="property.propertyDetails?.additionalDetails">
      <h2 class="mb-3 text-lg font-bold">Additional Details</h2>
      <ul class="grid grid-cols-2 gap-y-4">
         <template
            v-for="(value, key) in props.property.propertyDetails?.additionalDetails"
            :key="key"
         >
            <li class="flex items-center">
               <p class="text-sm text-black">{{ formatLabel(key) }}</p>
            </li>
            <li class="flex">
               <FlowbiteBadge :text="formatValue(value)" class="ml-4" />
            </li>
         </template>
      </ul>
   </div>
</template>

<script setup>
const props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

const formatLabel = (key) => {
   // Convert camelCase to Title Case
   return key
      .replace(/([A-Z])/g, " $1")
      .replace(/^./, (str) => str.toUpperCase());
};

const formatValue = (value) => {
   // Convert boolean to Yes/No
   if (typeof value === "boolean") {
      return value ? "Yes" : "No";
   }
   return String(value);
};
</script>
