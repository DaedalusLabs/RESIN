<template>
   <section
      class="mx-auto flex w-10/12 flex-col items-center justify-start gap-6 py-10"
   >
      <h1 class="text-center text-2xl font-extrabold leading-tight text-white">
         {{ $t("propertyType") }}
      </h1>
      <div class="grid grid-cols-1 justify-items-center gap-4 md:grid-rows-3">
         <PropertyType
            v-for="propertyType in propertyTypes"
            :key="propertyType.id"
            :id="propertyType.id"
            :title="$t(propertyType.title)"
            :description="$t(propertyType.description)"
            class="w-full flex-grow"
            :checked="settingsStore.propertyTypes.includes(propertyType.id)"
            @checked="togglePropertyType(propertyType.id)"
         />
      </div>
      <div class="flex flex-col items-center justify-center gap-2">
         <NuxtLinkLocale v-if="settingsStore.propertyTypes.length > 0" to="map">
            <FlowbiteButton :text="$t('chooseLocation')" />
         </NuxtLinkLocale>
         <FlowbiteDisabledButton v-else :text="$t('chooseLocation')" />
         <NuxtLinkLocale
            :class="[
               'rounded-lg px-5 py-2.5 text-sm font-medium text-pirate-400 hover:bg-white hover:text-pirate-700',
               { invisible: settingsStore.propertyTypes.length >= 1 },
            ]"
            to="map"
         >
            {{ $t("skip") }}
         </NuxtLinkLocale>
      </div>
   </section>
</template>

<script setup>
import { useSettingsStore } from '@/stores/settings';

const settingsStore = useSettingsStore();

const propertyTypes = ref([
   {
      id: 'short-term-stay',
      title: "propertyTypes.shortToMediumTermStay.title",
      description: "propertyTypes.shortToMediumTermStay.description",
   },
   {
      id: 'longer-term-residential-house',
      title: "propertyTypes.longerTermResidentialHouse.title",
      description: "propertyTypes.longerTermResidentialHouse.description",
   },
   {
      id: 'commercial-property',
      title: "propertyTypes.commercialProperty.title",
      description: "propertyTypes.commercialProperty.description",
   },
]);

const togglePropertyType = (propertyTypeId) => {
   if (settingsStore.propertyTypes.includes(propertyTypeId)) {
      settingsStore.propertyTypes = settingsStore.propertyTypes.filter(
         (id) => id !== propertyTypeId,
      );
   } else {
      settingsStore.propertyTypes.push(propertyTypeId);
   }
};



definePageMeta({
  layout: "intro",
});
</script>
