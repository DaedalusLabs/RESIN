<template>
   <div
      v-if="property && property.id"
      class="mx-auto mt-10 max-w-screen-md p-6"
   >
      <div class="mb-6 flex w-full items-center justify-between text-center">
         <h1 class="text-2xl font-extrabold">Rent-to-own</h1>
         <NuxtImg src="/images/logos/resin-text.png" alt="Resin" class="h-4" />
      </div>
      <ClientOnly fallback-tag="span">
         <ModalRequestTour
            :is-open="isModalOpen"
            :is-request-sent="isRequestSent"
            :property-address="propertyAddress"
            :reference-number="referenceNumber"
            @update:is-open="isModalOpen = $event"
            @send-request="handleSendRequest"
         />
      </ClientOnly>

      <FavoritesCard :property="property" :is-removable="false" class="mb-10" />

      <RentToOwnSection
         v-for="section in sections"
         :key="section.title"
         :title="section.title"
         :text="section.text"
      />

      <DetailsBottomBar
         v-show="!showDrawer"
         class="z-1"
         :property="property"
         @show-drawer="handleShowDrawer"
         @show-modal="handleShowModal"
      />
      <VerificationDrawer
         v-if="property.id !== null"
         v-show="showDrawer"
         :show="showDrawer"
         :skip-key-backup="skipKeyBackup"
         :property-id="property.id"
         class="fixed bottom-0 left-0 w-full p-4"
         :class="{ 'z-top': showDrawer }"
         @close="onCloseDrawer"
      />
      <ResinAlert
         :show="showSuccessAlert"
         text="Your information has been submitted successfully."
      />
   </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { usePropertiesStore } from "~/stores/properties";
import type { Property } from "~/types/property";

const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();
const route = useRoute();
const showDrawer = ref(false);

const property = ref<Property | null>(null);

const isModalOpen = ref(false);
const isRequestSent = ref(false);
const referenceNumber = ref(0);
const showSuccessAlert = ref(false);
const skipKeyBackup = ref(false);

onMounted(async () => {
   const foundProperty = await propertiesStore.get(route.params.id as string);
   if (!foundProperty) {
      return;
   }
   property.value = foundProperty;

   if (nostrStore.typeKey == NostrLoginType.Extension) {
      skipKeyBackup.value = true;
   }
});

const sections = [
   {
      title: "What is rent-to-own?",
      text: "Rent-to-own is a way to buy a home without a traditional mortgage. You rent the home for a period of time, and a portion of your rent goes towards your down payment. At the end of the rental period, you can buy the home using the down payment you've built up.",
   },
   {
      title: "How does it work?",
      text: "You'll pay a monthly rent that's slightly higher than market rate. The extra amount goes into a savings account that you can use as a down payment when you're ready to buy. You'll also have the option to buy the home at any time during your lease.",
   },
   {
      title: "What are the benefits?",
      text: "Rent-to-own lets you move into your dream home now, while building up a down payment. You can also lock in the purchase price today, protecting you from rising home prices. And since you're living in the home, you can make sure it's the right fit before committing to buy.",
   },
];

const onCloseDrawer = () => {
   showDrawer.value = false;
};

const handleShowModal = () => {
   isModalOpen.value = true;
};

function handleShowDrawer() {
   showDrawer.value = !showDrawer.value;
}

const propertyAddress = computed(() => {
   if (!property.value?.location) return "";
   return `${property.value.location.street}, ${property.value.location.city}, ${property.value.location.country}`;
});

const handleSendRequest = async () => {
   isModalOpen.value = false;

   showSuccessAlert.value = true;
   setTimeout(() => {
      showSuccessAlert.value = false;
   }, 5000);
};

definePageMeta({
   layout: "white",
   middleware: ["auth"],
});
</script>
