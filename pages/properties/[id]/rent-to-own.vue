<template>
   <div class="mx-auto mt-10 max-w-screen-md p-6" v-if="property && property.id">
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
         v-if="!showDrawer"
         class="z-1"
         :property="property"
         @show-drawer="handleShowDrawer"
         @show-modal="handleShowModal"
      />
      <VerificationDrawer
         v-if="property.id !== null"
         v-show="showDrawer"
         :show="showDrawer"

         :property-id="property.id"
         class="fixed bottom-0 left-0 w-full p-4"
         :class="{ 'z-top': showDrawer }"
         @close="showDrawer = false"
      />
      <ResinAlert
                  :show="showSuccessAlert"
                  text="Your information has been submitted successfully."
               />
   </div>
</template>

<script setup>
import { ref } from "vue";
import { usePropertiesStore } from "~/stores/properties";
const propertiesStore = usePropertiesStore();
const route = useRoute();
const showDrawer = ref(false);

const property = ref({
});

const isModalOpen = ref(false);
const isRequestSent = ref(false);
const referenceNumber = ref(0);
const showSuccessAlert = ref(false);

onMounted(async () => {
   const foundProperty = await propertiesStore.get(route.params.id);
   if (!foundProperty) {
      return;
   }
   property.value = foundProperty;
});

const sections = ref([
   {
      title: "Own without the bank",
      text: "Rent-to-own offers a flexible path to homeownership. You can start living in this modern home at Mahonylaan 5, Paramaribo, Suriname, while     gradually working towards owning it. Pay monthly rent with a portion       going towards the purchase price, making it easier to transition from    renting to owning without a large upfront payment. Enjoy the benefits  of homeownership, including building equity and having a place to call       your own, while taking your time to complete the purchase..",
   },
   {
      title: "Low interest rates",
      text: " Enjoy lower interest rates compared to traditional loans and save  more over time as you work towards homeownership.",
   },
   {
      title: "Grow your equity",
      text: " Build equity over time by owning part of the property as you    continue paying rent.",
   },
   {
      title: "Rent payment options",
      text: "Flexible rent payment plans are available to suit your financial    situation and help you transition into homeownership with ease.",
   },
]);

const handleShowModal = () => {
   isModalOpen.value = true;
};

function handleShowDrawer() {
   showDrawer.value = !showDrawer.value;
}

const propertyAddress = computed(() => {
   return `${property.value.location.street}, ${property.value.location.city}, ${property.value.location.country}`;
});

const handleSendRequest = async() => {
   isModalOpen.value = false;
   
   showSuccessAlert.value = true;
   setTimeout(() => {
      showSuccessAlert.value = false;
   }, 5000);  
};

definePageMeta({
   layout: "white",
});
</script>
