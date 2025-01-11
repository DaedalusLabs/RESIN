<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <h1 class="mb-10 text-2xl font-extrabold leading-tight">{{ $t('myResin.title') }}</h1>

      <div class="space-y-10">
         <section class="flex flex-col gap-4">
            <MyResinNavbar :title="$t('myResin.favorites.title')" link="favorites" />

            <div v-if="favorites.length === 0">
               <p class="mt-20 gap-2 font-semibold text-pirate-950">
                  {{ $t('myResin.favorites.noProperties') }}
               </p>
            </div>
            <div class="space-y-4">
               <FavoritesCard v-for="favorite in favorites" :key="favorite.id" :property="favorite"
                  @remove="removeFavorite(favorite.id)" />
            </div>
         </section>

         <section v-if="propertiesStore.ownedProperties.length > 0" class="flex flex-col gap-4">
            <MyResinNavbar title="Properties" link="my-properties" />

            <div v-if="propertiesStore.ownedProperties.length === 0">
               <p class="mt-20 gap-2 font-semibold text-pirate-950">
                  {{ $t('myResin.properties.noProperties') }}
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
         <section v-if="transactionsStore.getTotalAmount > 0">
            <MyResinNavbar title="Transactions" link="transactions" />
            <TransactionsBarChart
               class="mt-4"
               :total-amount="transactionsStore.getTotalAmount"
               :paid-amount="transactionsStore.getPaidOffAmount"
               :remaining-amount="transactionsStore.getToBePaidOffAmount"
               :progress-percentage="transactionsStore.getPayedOffPercentage"
            />
         </section>
         <section v-if="transactionsStore.getTotalAmount > 0">
            <MyResinNavbar
               title="Agreements"
               link="my-agreements"
               class="mb-4"
            />
            <div v-if="transactionsStore.getAgreements.length === 0">
               <p class="gap-2 font-semibold text-pirate-950">
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
         <section v-if="transactionsStore.getTotalAmount > 0">
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

const transactionsStore = useTransactionsStore();
const propertiesStore = usePropertiesStore();
const favorites = ref([]);
watch(() => propertiesStore.favorites, async (newFavorites) => {
   console.log('newFavorites', newFavorites);
   favorites.value = await Promise.all(newFavorites.map(async (id) => await propertiesStore.get(id)));
}, { immediate: true, deep: true });


const { t } = useI18n();
const features = [/*"End lease", "Refinance", "Sell a property" */];

useHead({
   title: t('my-resin.title'),
});

onMounted(() => {
});

const removeFavorite = (id) => {
   propertiesStore.toggleFavorite(id);
};

definePageMeta({
   layout: "white",
});
</script>
