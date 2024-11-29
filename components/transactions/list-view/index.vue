<template>
   <div class="w-full rounded-lg bg-white p-6 shadow">
      <TransactionsListViewItem
         v-for="transaction in transactions"
         :key="transaction.id"
         :transaction="transaction"
      />
   </div>
</template>

<script setup lang="ts">
defineProps({
   transactions: {
      type: Array as PropType<Transaction[]>,
      required: true,
   },
});
</script>
