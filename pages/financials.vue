<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <div class="mb-5 flex items-center justify-between text-pirate-950">
         <div class="flex items-center gap-2">
            <NuxtLinkLocale to="my-resin">
               <PhCaretLeft :size="24" />
            </NuxtLinkLocale>
            <h1 class="text-2xl font-extrabold leading-tight">Financials</h1>
         </div>
         <button :v-if="isSupported" @click="startShare">
            <PhExport :size="28" class="text-xl text-pirate-950" />
         </button>
      </div>
      <div class="flex flex-col gap-8">
         <ClientOnly fallback-tag="span">
            <FinancialsEquityChart
               :equity="transactionsStore.getPaidOffAmount"
               :pay-off-amount="transactionsStore.getToBePaidOffAmount"
            />
         </ClientOnly>
         <div class="flex flex-col gap-4">
            <FinancialsPropertyOwnershipCard
               v-for="property in propertiesStore.ownedProperties"
               :key="property.id"
               :property="property"
               :ownership-percentage="
                  transactionsStore.getOwnerShipPercentage(property.id)
               "
            />
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { useTransactionsStore } from "~/stores/transactions";
import { useShare } from "@vueuse/core";
import { PhExport, PhCaretLeft } from "@phosphor-icons/vue";

const { share, isSupported } = useShare();
const transactionsStore = useTransactionsStore();
const propertiesStore = usePropertiesStore();

const { t } = useI18n();

useHead({
   title: t('financials.title'),
});

function startShare() {
   share({
      title: "Resin",
      text: "Check out these listings on Resin",
      url: location.href,
   });
}

definePageMeta({
   layout: "white",
});
</script>
