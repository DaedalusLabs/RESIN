<template>
   <div class="flex flex-col gap-4">
      <div
         class="group relative flex flex-col items-center rounded-lg border-2 border-pirate-100 bg-white p-4 shadow-sm transition-transform duration-300 ease-in-out"
      >
         <div class="flex w-full justify-between">
            <NuxtImg
               :src="property?.images[0]"
               alt="Favorite image"
               class="mr-4 h-16 w-16 rounded-md object-cover object-center"
            />
            <div class="w-full flex-1">
               <h3 class="truncate text-lg font-bold text-resin-500">
                  {{ property?.location.address.street }}
               </h3>
               <p class="text-sm text-pirate-950">
                  {{ property?.location.address.city }},
                  {{ property?.location.address.country }}
               </p>
               <p class="text-sm font-bold text-pirate-950">
                  ${{ property?.pricingDetails.rentPerMonth }} per month
               </p>
            </div>
         </div>
         <div class="mt-2 w-full">
            <div class="mt-1 h-2 w-full rounded-full bg-pirate-100">
               <div
                  class="h-full rounded-full bg-resin-500 transition-all duration-300"
                  :style="{ width: `${ownershipPercentage}%` }"
               ></div>
            </div>
            <div
               class="mt-1 flex items-center justify-between text-xs text-gray-500"
            >
               <span>{{ ownershipPercentage }}% ownership</span>
            </div>
         </div>
      </div>
   </div>
</template>

<script setup>
defineProps({
   property: {
      type: Object,
      required: true,
   },
   ownershipPercentage: {
      type: Number,
      required: true,
   },
});
</script>
