<template>
   <div class="mt-12 w-3/4" v-if="property.additional_details">
      <h2 class="mb-3 text-lg font-bold">{{ $t('property.details.additional.title') }}</h2>
      <ul class="grid grid-cols-2 gap-y-4">
         <template
            v-for="(value, key) in props.property.additional_details"
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
const { t } = useI18n();

const props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

const formatLabel = (key) => {
   // Convert snake_case to Title Case
   return key
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
      .join(' ');
};

const formatValue = (value) => {
   // Convert boolean to Yes/No
   if (typeof value === "boolean") {
      return value ? t('property.details.additional.yes') : t('property.details.additional.no');
   }
   return String(value);
};
</script>
