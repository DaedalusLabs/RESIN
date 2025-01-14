<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <div class="mb-5 flex items-center justify-between text-pirate-950">
         <div class="flex items-center gap-2">
            <NuxtLinkLocale to="my-resin">
               <PhCaretLeft :size="24" />
            </NuxtLinkLocale>
            <h1 class="text-2xl font-extrabold leading-tight">
               {{ t("transactions.title") }}
            </h1>
         </div>
         <button v-if="hasTransactions && isSupported" @click="startShare">
            <PhExport :size="28" class="text-xl text-pirate-950" />
         </button>
      </div>
      <div class="flex flex-col gap-8">
         <TransactionsBarChart
            v-if="hasTransactions"
            :total-amount="transactionsStore.getTotalAmount"
            :paid-amount="transactionsStore.getPaidOffAmount"
            :remaining-amount="transactionsStore.getToBePaidOffAmount"
            :progress-percentage="transactionsStore.getPayedOffPercentage"
         />
         <TransactionsListView
            :transactions="transactionsStore.getTransactions"
         />
      </div>
   </section>
</template>

<script setup lang="ts">
import { useTransactionsStore } from "~/stores/transactions";
import { useShare } from "@vueuse/core";
import { PhExport, PhCaretLeft } from "@phosphor-icons/vue";

const { t } = useI18n();

useHead({
   title: t("transactions.title"),
});

const { share, isSupported } = useShare();
const transactionsStore = useTransactionsStore();

const hasTransactions = computed(
   () => transactionsStore.getTransactions.length > 0,
);

function startShare() {
   share({
      title: "Resin",
      text: "Check out these listings on Resin",
      url: location.href,
   });
}

definePageMeta({
   layout: "white",
   middleware: ["auth"],
});
</script>
