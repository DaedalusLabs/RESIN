<template>
   <div class="mt-12 flex space-x-8">
      <div v-if="property['resin-type'] !== 'Buy Now'">
         <h3 class="mb-2 text-xs">Rent Price</h3>
         <p class="text-lg font-semibold">
            $ {{ property.price.toLocaleString() }}
            per month
         </p>
      </div>
      <div v-else>
         <h3 class="mb-2 text-xs">Property Price</h3>
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
