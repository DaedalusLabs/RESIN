<template>
   <section v-if="isLoading" class="container mx-auto mb-28 w-10/12 py-8">
      <div class="animate-pulse">
         <div class="h-64 w-full bg-gray-200"></div>
         <div class="mt-8 h-8 w-3/4 bg-gray-200"></div>
         <div class="mt-2 h-4 w-1/2 bg-gray-200"></div>
      </div>
   </section>

   <section v-else-if="error" class="container mx-auto mb-28 w-10/12 py-8">
      <div class="rounded-lg bg-red-50 p-4">
         <div class="flex">
            <div class="flex-shrink-0">
               <svg
                  class="h-5 w-5 text-red-400"
                  viewBox="0 0 20 20"
                  fill="currentColor"
               >
                  <path
                     fill-rule="evenodd"
                     d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                     clip-rule="evenodd"
                  />
               </svg>
            </div>
            <div class="ml-3">
               <h3 class="text-sm font-medium text-red-800">
                  Property Not Found
               </h3>
               <div class="mt-2 text-sm text-red-700">
                  <p>
                     The property you're looking for could not be found. Please
                     check the URL or go back to the properties list.
                  </p>
               </div>
               <div class="mt-4">
                  <NuxtLink
                     to="/properties"
                     class="text-sm font-medium text-red-800 hover:text-red-900"
                  >
                     Go back to properties →
                  </NuxtLink>
               </div>
            </div>
         </div>
      </div>
   </section>

   <section v-else class="mb-28">
      <!-- Property Image and Basic Info -->
      <FlowbiteCheckToast
         :show-toast="isRequestSent && property?.isBitcasaHome"
         :text="`Information submitted`"
         @close-toast="isRequestSent = false"
      />
      <ModalContactAgent
         v-if="property?.isBitcasaHome"
         :is-open="isModalOpen"
         :property-address="propertyAddress"
         @update:is-open="isModalOpen = $event"
         @send-request="handleSendRequest"
      />
      <ModalRequestTour
         v-else
         :is-open="isModalOpen"
         :is-request-sent="isRequestSent"
         :property-address="propertyAddress"
         :reference-number="referenceNumber"
         @update:is-open="isModalOpen = $event"
         @send-request="handleSendRequest"
      />

      <div class="relative">
         <NuxtImg
            v-if="property.images?.length"
            :src="property.images[0]"
            alt="Property Image"
            class="h-64 w-full object-cover"
         />
         <div
            v-else
            class="flex h-64 w-full items-center justify-center bg-gray-200"
         >
            <span class="text-gray-400">No image available</span>
         </div>
         <DetailsTopBar :property="property" />
      </div>

      <!-- Property Details -->
      <div class="container mx-auto mt-8 w-10/12">
         <h1 class="text-2xl font-extrabold leading-tight">
            {{ property.location?.address?.street || "Address not available" }}
         </h1>
         <p class="mt-1 text-sm">
            {{ property.location?.address?.city }},
            {{ property.location?.address?.country }}
         </p>
         <DetailsSize :property="property" />
         <DetailsPrices :property="property" />
         <DetailsSummary :property="property" />
         <DetailsKeyFeatures :property="property" />
         <DetailsAdditional :property="property" />
      </div>

      <ClientOnly fallback-tag="span">
         <DetailsMap :property="property" />
      </ClientOnly>
      <DetailsNearby :property="property" />
      <DetailsBottomBar :property="property" @show-modal="handleShowModal" />
   </section>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { fixNestedStrings } from "~/utils/jsonParser";
import ModalContactAgent from "~/components/Modal/contact-agent.vue";
import ModalRequestTour from "~/components/Modal/request-tour.vue";

const propertiesStore = usePropertiesStore();
const route = useRoute();
const error = ref(false);
const isLoading = ref(true);
const property = ref(null);
const isRequestSent = ref(false);
const isModalOpen = ref(false);
const referenceNumber = ref(0);

const handleShowModal = () => {
   isModalOpen.value = true;
};

const generateRef = () => {
   return Math.floor(Math.random() * 9000) + 1000;
};

onMounted(() => {
   referenceNumber.value = generateRef();
});

onMounted(async () => {
   try {
      const foundProperty = propertiesStore.properties.find(
         (p) => p.id === route.params.id,
      );

      if (!foundProperty) {
         error.value = true;
         return;
      }

      property.value = foundProperty;

      if (typeof property.value === "string") {
         fixNestedStrings(property);
      }

      propertiesStore.addViewedProperty(route.params.id);
   } catch (e) {
      console.error("Error loading property:", e);
      error.value = true;
   } finally {
      isLoading.value = false;
   }
});

const handleSendRequest = () => {
   isRequestSent.value = true;
   if (property?.value.isBitcasaHome) {
      isModalOpen.value = false;
   }
};

const propertyAddress = computed(() => {
   return `${property.value.location.address.street}, ${property.value.location.address.city}, ${property.value.location.address.country}`;
});

definePageMeta({
   layout: "white",
});

defineProps({
   showToasts: {
      type: Boolean,
      default: false,
   },
});
</script>

<style scoped>
.z-top {
   z-index: 1000;
}
</style>
