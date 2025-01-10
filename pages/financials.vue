<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <div class="mb-5 flex items-center justify-between text-pirate-950">
         <div class="flex items-center gap-2">
            <NuxtLinkLocale to="my-resin">
               <PhCaretLeft :size="24" />
            </NuxtLinkLocale>
            <h1 class="text-2xl font-extrabold leading-tight">{{ t('financials.title') }}</h1>
         </div>
         <button v-if="hasData && isSupported" @click="startShare">
            <PhExport :size="28" class="text-xl text-pirate-950" />
         </button>
      </div>
      <div class="flex flex-col gap-8">
         <template v-if="hasData">
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
         </template>
         <div v-else class="flex flex-col items-center justify-center py-16 text-center">
            <div class="mb-4 rounded-full bg-gray-100 p-4">
               <PhChartPie :size="48" class="text-gray-400" />
            </div>
            <h3 class="mb-2 text-lg font-semibold text-gray-900">{{ t('financials.noData.title') }}</h3>
            <p class="text-sm text-gray-500">{{ t('financials.noData.description') }}</p>
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { useTransactionsStore } from "~/stores/transactions";
import { usePropertiesStore } from "~/stores/properties";
import { useShare } from "@vueuse/core";
import { PhExport, PhCaretLeft, PhChartPie } from "@phosphor-icons/vue";

const { share, isSupported } = useShare();
const transactionsStore = useTransactionsStore();
const propertiesStore = usePropertiesStore();

const { t } = useI18n();

useHead({
   title: t('financials.title'),
});

const hasData = computed(() => 
   transactionsStore.getPaidOffAmount > 0 || 
   transactionsStore.getToBePaidOffAmount > 0 || 
   propertiesStore.ownedProperties.length > 0
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
});
</script>
