<template>
   <div class="group relative flex items-center rounded-lg">
      <NuxtImg
         :src="area.image"
         alt="Area image"
         class="mr-4 h-16 w-16 rounded-md object-cover object-center"
      />
      <div class="min-w-0 flex-1">
         <h3 class="truncate text-lg font-bold">
            {{ area.name }}
         </h3>
         <p class="text-sm text-pirate-950">
            {{ area.city }}, {{ area.country }}
         </p>
         <p class="text-sm font-bold text-pirate-950">
            From ${{ area.price }} per month
         </p>
      </div>
   </div>
</template>

<script setup>
defineProps({
   area: {
      type: Object,
      required: true,
   },
});
</script>
