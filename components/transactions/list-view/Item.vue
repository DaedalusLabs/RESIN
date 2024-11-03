<template>
   <div
      class="border-b-2 border-gray-100 py-4 first:pt-0 last:border-0 last:pb-0"
   >
      <div
         class="flex items-center justify-between"
         :class="
            transaction.status === 'paid'
               ? 'text-pirate-950'
               : 'text-pirate-300'
         "
      >
         <!-- Left side with icon and date -->
         <div class="flex items-center gap-4">
            <div
               :class="
                  transaction.status === 'paid'
                     ? 'text-resin-500'
                     : 'text-pirate-300'
               "
            >
               <PhCheckCircle
                  v-if="transaction.status === 'paid'"
                  size="32"
                  weight="regular"
               />
               <PhClock v-else size="32" weight="regular" />
            </div>
            <div>
               <h3 class="text-lg font-semibold text-gray-900">
                  {{ getMonth(transaction.paid) }}
                  {{ getYear(transaction.paid) }}
               </h3>
               <p class="text-sm text-gray-500">
                  {{ transaction.status === "paid" ? "Paid" : "Due" }}
                  <span v-if="transaction.status === 'due'">
                     {{ formatDate(transaction.due) }}
                  </span>
                  <span v-else>
                     {{ formatDate(transaction.paid) }}
                  </span>
               </p>
            </div>
         </div>

         <!-- Right side with amount and arrow -->
         <div class="flex items-center gap-1">
            <p class="text-lg font-semibold">
               {{ formatCurrency(transaction.amount) }}
            </p>
            <PhCaretRight
               v-if="transaction.status === 'pending'"
               size="20"
               weight="bold"
            />
         </div>
      </div>
   </div>
</template>

<script setup lang="ts">
import { PhCheckCircle, PhClock, PhCaretRight } from "@phosphor-icons/vue";
const { formatCurrency } = useFormatNumber();

defineProps({
   transaction: {
      type: Object as PropType<Transaction>,
      required: true,
   },
});

function getMonth(date: Date) {
   return date.toLocaleDateString("en-US", { month: "short" });
}

function getYear(date: Date) {
   return date.getFullYear();
}

function formatDate(date: Date) {
   const month = String(date.getMonth() + 1).padStart(2, "0");
   const day = String(date.getDate()).padStart(2, "0");
   const year = date.getFullYear();

   return `${month}/${day}/${year}`;
}
</script>
