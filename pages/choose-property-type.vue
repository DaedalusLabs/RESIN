<template>
   <section
      class="mx-auto flex h-full w-10/12 flex-col items-center justify-center gap-10"
   >
      <h1 class="text-center text-2xl font-extrabold leading-tight text-white">
         {{ $t("propertyType") }}
      </h1>
      <div class="grid grid-cols-1 justify-items-center gap-6 md:grid-rows-3">
         <PropertyType
            v-for="propertyType in propertyTypes"
            :key="propertyType.id"
            :title="$t(propertyType.title)"
            :description="$t(propertyType.description)"
            class="w-full flex-grow"
            @click="addPropertyType(propertyType.id)"
         />
      </div>
      <div class="flex flex-col items-center justify-center gap-2">
         <NuxtLinkLocale v-if="selectedPropertyTypes.length > 0" to="map">
            <FlowbiteButton :text="$t('chooseLocation')" />
         </NuxtLinkLocale>
         <FlowbiteDisabledButton v-else :text="$t('chooseLocation')" />
         <NuxtLinkLocale
            :class="[
               'rounded-lg px-5 py-2.5 text-sm font-medium text-pirate-400 hover:bg-white hover:text-pirate-700',
               { invisible: selectedPropertyTypes.length >= 1 },
            ]"
            to="map"
         >
            {{ $t("skip") }}
         </NuxtLinkLocale>
      </div>
   </section>
</template>

<script setup>
const selectedPropertyTypes = ref([]);
const propertyTypes = ref([
   {
      id: 1,
      title: "propertyTypes.shortToMediumTermStay.title",
      description: "propertyTypes.shortToMediumTermStay.description",
   },
   {
      id: 2,
      title: "propertyTypes.longerTermResidentialHouse.title",
      description: "propertyTypes.longerTermResidentialHouse.description",
   },
   {
      id: 3,
      title: "propertyTypes.commercialProperty.title",
      description: "propertyTypes.commercialProperty.description",
   },
]);

const addPropertyType = (propertyTypeId) => {
   if (selectedPropertyTypes.value.includes(propertyTypeId)) {
      selectedPropertyTypes.value = selectedPropertyTypes.value.filter(
         (id) => id !== propertyTypeId,
      );
   } else {
      selectedPropertyTypes.value.push(propertyTypeId);
   }
};

definePageMeta({
   layout: "intro",
});
</script>
