<template>
   <div class="mt-12 flex space-x-8">
      <div v-if="property['resin-type'] !== 'Buy Now'">
         <h3 class="mb-2 text-xs">{{ $t('property.details.prices.rentPrice') }}</h3>
         <p class="text-lg font-semibold">
            $ {{ property.price.toLocaleString() }}
            {{ $t('property.details.prices.perMonth') }}
         </p>
      </div>
      <div v-else>
         <h3 class="mb-2 text-xs">{{ $t('property.details.prices.propertyPrice') }}</h3>
         <p class="text-lg font-semibold">
            $ {{ property.price.toLocaleString() }}
         </p>
      </div>
   </div>
</template>

<script setup>
defineProps({
   property: {
      type: Object,
      required: true,
   },
});
</script>
