<!-- FinancialStatsCard.vue -->
<template>
   <div class="w-full rounded-lg bg-white p-6 shadow">
      <!-- Header with Total Amount -->
      <div class="mb-2 border-b border-pirate-200 pb-2">
         <div class="flex items-center justify-between">
            <dt class="text-base font-normal text-pirate-500">Total amount</dt>

            <span
               class="inline-block rounded-md bg-resin-50 px-3 py-1 text-sm font-medium text-resin-500"
            >
               Payed off {{ progressPercentage.toFixed(1) }}%
            </span>
         </div>
         <dd class="text-3xl font-bold text-pirate-900">
            {{ formatCurrency(totalAmount) }}
         </dd>
         <div class="mt-2"></div>
      </div>

      <!-- Payment Statistics -->
      <div class="mb-6 flex justify-between">
         <div>
            <dt class="text-base font-normal text-pirate-500">Paid off</dt>
            <dd class="text-2xl font-bold text-resin-500">
               {{ formatCurrency(paidAmount) }}
            </dd>
         </div>
         <div>
            <dt class="text-right text-base font-normal text-pirate-500">
               To be paid
            </dt>
            <dd class="text-2xl font-bold text-pirate-600">
               {{ formatCurrency(remainingAmount) }}
            </dd>
         </div>
      </div>

      <!-- Progress Bars -->
      <div class="space-y-2 border-y border-dashed border-pirate-200 py-3">
         <div class="h-2 w-1/3 rounded bg-pirate-500"></div>
         <div class="h-2 w-full rounded bg-resin-500"></div>
      </div>

      <!-- Scale -->
      <div class="mb-4 mt-8 flex justify-between text-sm text-pirate-500">
         <span class="rotate">0</span>
         <span v-for="i in 5" :key="i" class="rotate">
            {{ formatCurrency((totalAmount / 5) * i) }}
         </span>
      </div>
   </div>
</template>

<script setup lang="ts">
const { formatCurrency } = useFormatNumber();

defineProps({
   totalAmount: {
      type: Number,
      required: true,
   },
   paidAmount: {
      type: Number,
      required: true,
   },
   remainingAmount: {
      type: Number,
      required: true,
   },
   progressPercentage: {
      type: Number,
      required: true,
   },
});
</script>

<style scoped>
.rotate {
   transform: rotate(-30deg);
}
</style>
