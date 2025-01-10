<template>
   <div class="w-full rounded-lg bg-white p-6 shadow">
      <template v-if="transactions.length > 0">
         <TransactionsListViewItem
            v-for="transaction in transactions"
            :key="transaction.id"
            :transaction="transaction"
         />
      </template>
      <div v-else class="flex flex-col items-center justify-center py-8 text-center">
         <div class="mb-4 rounded-full bg-gray-100 p-4">
            <PhCurrencyCircleDollar :size="48" class="text-gray-400" />
         </div>
         <h3 class="mb-2 text-lg font-semibold text-gray-900">{{ $t('transactions.noTransactions.title') }}</h3>
         <p class="text-sm text-gray-500">{{ $t('transactions.noTransactions.description') }}</p>
      </div>
   </div>
</template>

<script setup lang="ts">
import { PhCurrencyCircleDollar } from "@phosphor-icons/vue";

defineProps({
   transactions: {
      type: Array as PropType<Transaction[]>,
      required: true,
   },
});
</script>
