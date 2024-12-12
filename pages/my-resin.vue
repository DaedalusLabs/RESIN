<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <h1 class="mb-10 text-2xl font-extrabold leading-tight">My Resin</h1>

      <div class="space-y-10">
         <section class="flex flex-col gap-4">
            <MyResinNavbar title="Properties" link="my-properties" />

            <div v-if="properties.length === 0">
               <p class="mt-20 gap-2 font-semibold text-pirate-950">
                  You have no properties yet
               </p>
            </div>
            <div v-else class="space-y-4">
               <FavoritesCard
                  v-for="property in displayedProperties"
                  :key="property.id"
                  :is-removable="false"
                  :property="property"
               />
               <div v-if="properties.length > 2" class="text-right">
                  <NuxtLinkLocale
                     to="my-properties"
                     class="text-sm text-pirate-600 hover:text-pirate-700"
                  >
                     View all properties ({{ properties.length }})
                  </NuxtLinkLocale>
               </div>
            </div>
         </section>
         <section>
            <MyResinNavbar title="Transactions" link="transactions" />
            <TransactionsBarChart
               class="mt-4"
               :total-amount="transactionsStore.getTotalAmount"
               :paid-amount="transactionsStore.getPaidOffAmount"
               :remaining-amount="transactionsStore.getToBePaidOffAmount"
               :progress-percentage="transactionsStore.getPayedOffPercentage"
            />
         </section>
         <section>
            <MyResinNavbar
               title="Agreements"
               link="my-agreements"
               class="mb-4"
            />
            <div v-if="transactionsStore.agreements.length === 0">
               <p class="mt-20 gap-2 font-semibold text-pirate-950">
                  You have no agreements yet
               </p>
            </div>
            <div v-else class="rounded-lg border border-gray-200 bg-white p-4">
               <MyResinAgreementCard
                  v-for="(agreement, index) in transactionsStore.getAgreements"
                  :key="agreement.id"
                  :agreement="agreement"
                  :class="[
                     'py-4',
                     index !== transactionsStore.getAgreements.length - 1
                        ? 'border-b border-gray-200'
                        : '',
                  ]"
               />
            </div>
         </section>
         <section>
            <MyResinNavbar title="Financials" link="financials" />
            <FinancialsEquityChart
               class="mt-4"
               :equity="transactionsStore.getPaidOffAmount"
               :pay-off-amount="transactionsStore.getToBePaidOffAmount"
            />
         </section>
         <section>
            <div class="mt-6 space-y-8">
               <FinancialsFeaturePlaceholder
                  v-for="(feature, index) in features"
                  :key="index"
                  :title="feature"
               />
            </div>
         </section>
      </div>
   </section>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { useTransactionsStore } from "~/stores/transactions";

const transactionsStore = useTransactionsStore();

const propertiesStore = usePropertiesStore();
const properties = propertiesStore.ownedProperties;
const displayedProperties = computed(() => properties.slice(0, 2));
const features = ["End lease", "Refinance", "Sell a property"];
definePageMeta({
   layout: "white",
});
</script>
