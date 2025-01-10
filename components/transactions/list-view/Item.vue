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
                  {{ transaction.property.name }}
               </h3>
               <p class="text-sm text-gray-500">
                  {{ transaction.status === "paid" ? "Paid" : "Due" }}
                  <span v-if="transaction.status === 'pending'">
                     {{ formatDate(transaction.dueDate) }}
                  </span>
                  <span v-else-if="transaction.payment">
                     {{ formatDate(transaction.payment.date) }}
                  </span>
               </p>
            </div>
         </div>

         <!-- Right side with amount and arrow -->
         <div class="flex items-center gap-1">
            <p class="text-lg font-semibold">
               {{ formatCurrency(parseFloat(transaction.amount)) }}
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
import type { Transaction } from "~/stores/transactions";
const { formatCurrency } = useFormatNumber();

defineProps({
   transaction: {
      type: Object as PropType<Transaction>,
      required: true,
   },
});

function formatDate(dateString: string) {
   const date = new Date(dateString);
   const month = String(date.getMonth() + 1).padStart(2, "0");
   const day = String(date.getDate()).padStart(2, "0");
   const year = date.getFullYear();

   return `${month}/${day}/${year}`;
}
</script>
