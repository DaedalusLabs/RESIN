<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <div class="mb-5 flex items-center justify-between text-pirate-950">
         <div class="flex items-center gap-2">
            <NuxtLinkLocale to="my-resin">
               <PhCaretLeft :size="24" />
            </NuxtLinkLocale>
            <h1 class="text-2xl font-extrabold leading-tight">
               {{ t("financials.title") }}
            </h1>
         </div>
         <button v-if="hasData && isSupported" @click="startShare">
            <PhExport :size="28" class="text-xl text-pirate-950" />
         </button>
      </div>
      <div class="flex flex-col gap-8">
         <template v-if="transactionsStore.isLoading">
            <div class="flex items-center justify-center py-16">
               <div class="loader"></div>
            </div>
         </template>
         <template v-else-if="transactionsStore.error">
            <div
               class="flex flex-col items-center justify-center py-16 text-center"
            >
               <div class="mb-4 rounded-full bg-red-100 p-4">
                  <PhWarning :size="48" class="text-red-400" />
               </div>
               <h3 class="mb-2 text-lg font-semibold text-gray-900">
                  {{ t("common.error") }}
               </h3>
               <p class="text-sm text-gray-500">
                  {{ transactionsStore.error }}
               </p>
            </div>
         </template>
         <template v-else-if="hasData">
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
         <div
            v-else
            class="flex flex-col items-center justify-center py-16 text-center"
         >
            <div class="mb-4 rounded-full bg-gray-100 p-4">
               <PhChartPie :size="48" class="text-gray-400" />
            </div>
            <h3 class="mb-2 text-lg font-semibold text-gray-900">
               {{ t("financials.noData.title") }}
            </h3>
            <p class="text-sm text-gray-500">
               {{ t("financials.noData.description") }}
            </p>
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { useTransactionsStore } from "~/stores/transactions";
import { usePropertiesStore } from "~/stores/properties";
import { useShare } from "@vueuse/core";
import {
   PhExport,
   PhCaretLeft,
   PhChartPie,
   PhWarning,
} from "@phosphor-icons/vue";
import { useNDK } from "~/composables/useNDK";
import { NostrAPI } from "~/lib/nostr_api";

const { share, isSupported } = useShare();
const transactionsStore = useTransactionsStore();
const propertiesStore = usePropertiesStore();
const { t } = useI18n();

useHead({
   title: t("financials.title"),
});

const nostrStore = useNostrStore();
watch(
   () => nostrStore.authenticated,
   async (isAuthenticated) => {
      console.log("finacials: watch authenticated");
      if (isAuthenticated) {
         const ndk = useNDK();

         console.log("finacials: is authenticated");
         const nostrApi = new NostrAPI(ndk);
         console.log("fetching transactions");
         await transactionsStore.fetchTransactions(nostrApi);
      } else {
         console.log("finacials: is not authenticated");
      }
   },
);

const hasData = computed(
   () =>
      transactionsStore.getPaidOffAmount > 0 ||
      transactionsStore.getToBePaidOffAmount > 0 ||
      propertiesStore.ownedProperties.length > 0,
);

function startShare() {
   share({
      title: "Resin",
      text: "Check out these listings on Resin",
      url: location.href,
   });
}

// Fetch data when component mounts
onMounted(async () => {
   console.log("finacials: on mounted", nostrStore.authenticated);
   if (nostrStore.authenticated) {
      console.log("finacials: authenticated");
      const ndk = useNDK();
      const nostrApi = new NostrAPI(ndk);
      await transactionsStore.fetchTransactions(nostrApi);
   }
});

definePageMeta({
   layout: "white",
});
</script>
