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
                  <NuxtLinkLocale
                     to="properties"
                     class="text-sm font-medium text-red-800 hover:text-red-900"
                  >
                     Go back to properties →
                  </NuxtLinkLocale>
               </div>
            </div>
         </div>
      </div>
   </section>

   <section v-else class="mb-28">
      <FlowbiteCheckToast
         :show-toast="isRequestSent && property?.isBitcasaHome"
         :text="`Information submitted`"
         @close-toast="isRequestSent = false"
      />
      <FlowbiteModal
         v-if="property?.isBitcasaHome"
         :is-open="isModalOpen"
         @update:is-open="isModalOpen = $event"
      >
         <template #title>
            <h3 class="text-lg font-bold text-gray-900">Contact agent</h3>
         </template>
         <div class="mt-4">
            <p class="text-gray-900">Property</p>
            <p class="font-bold text-gray-900">{{ propertyAddress }}</p>
            <br />
            <p>
               Thank you for your interest in this property. Please leave your
               preferred contact information and our agent will contact you
               shortly.
            </p>
            <div class="mt-6">
               <FlowbiteTextInput
                  label="Phone"
                  placeholder="Phone"
                  class="mb-4"
                  :error-messages="formError ? ['Please fill in the form'] : []"
                  :model-value="phone"
                  @update:model-value="phone = $event"
               />
            </div>
            <div class="mt-6 flex items-center justify-center">
               <div class="flex-grow border-t border-gray-300"></div>
               <span class="mx-3 text-sm font-medium text-gray-500">Or</span>
               <div class="flex-grow border-t border-gray-300"></div>
            </div>
            <div class="mt-6">
               <FlowbiteTextInput
                  label="Email"
                  placeholder="Email"
                  class="mb-4"
                  :error-messages="formError ? ['Please fill in the form'] : []"
                  :model-value="email"
                  @update:model-value="email = $event"
               />
            </div>
            <div class="mt-6 flex justify-center">
               <FlowbiteButton :text="`Submit`" @click="handleSendRequest" />
            </div>
         </div>
      </FlowbiteModal>

      <FlowbiteModal
         v-else
         :is-open="isModalOpen"
         @update:is-open="isModalOpen = $event"
      >
         <template #title>
            <div v-if="isRequestSent" class="mb-5 flex items-center gap-2">
               <PhCheck size="24" class="text-resin-500" />
               <h3>Tour requested</h3>
            </div>
            <h3 v-else class="mb-5 mr-3 text-lg">
               Send tour request for {{ propertyAddress }}
            </h3>
         </template>
         <div v-if="isRequestSent">
            <p>
               One of our agents will contact you shortly to make an
               appointment. Make sure to check your messages inbox in your
               profile.
            </p>
         </div>
         <div
            v-else
            class="flex items-center justify-between rounded-lg border border-gray-300 p-3"
         >
            <p class="mr-3 text-sm text-pirate-950">
               I want to request a tour to see {{ propertyAddress }}.
               <span class="text-gray-500">REF {{ referenceNumber }}</span>
            </p>
            <button
               class="rounded-lg bg-resin-500 px-6 py-3 text-sm font-semibold text-white"
               @click="handleSendRequest"
            >
               Send
            </button>
         </div>
      </FlowbiteModal>

      <FiltersDrawer
         :show-drawer="showFilterDrawer"
         @close="showFilterDrawer = false"
      />

      <TopBar
         class="relative mx-auto mt-10 hidden max-w-[75%] xl:block"
         @toggle-filters="showFilterDrawer = !showFilterDrawer"
      />

      <BackgroundOverlay :show="showDrawer" @close="showDrawer = false" />

      <FlowbiteImageDrawer
         :show-drawer="showDrawer"
         :image-urls="property?.images"
         @close="showDrawer = false"
      />

      <div class="relative xl:hidden">
         <NuxtImg
            v-if="property.images?.length"
            :src="property.images[0]"
            alt="Property Image"
            class="h-64 w-full object-cover"
         />

         <span
            v-if="property.isBitcasaHome"
            class="absolute bottom-4 right-4 z-10 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md hover:border-white hover:bg-resin-500 hover:text-white"
         >
            For Sale
         </span>
         <span
            v-else
            class="absolute bottom-4 right-4 z-10 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md hover:border-white hover:bg-resin-500 hover:text-white"
         >
            Rent to Own
         </span>

         <DetailsTopBar :property="property" />
      </div>

      <div class="mx-auto mt-16 flex w-10/12 justify-between lg:w-9/12">
         <div
            class="sticky top-16 hidden flex-shrink flex-col gap-5 lg:w-[40%] xl:flex"
         >
            <div>
               <NuxtImg
                  v-if="property.images?.length"
                  :src="property.images[0]"
                  alt="Property Image"
                  class="h-96 w-full object-cover"
               />
               <button
                  class="absolute right-4 top-4 z-50 flex h-10 w-10 items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
                  @click="showDrawer = true"
               >
                  <PhImages :size="20" />
               </button>
            </div>
            <div class="flex h-12 justify-between gap-3">
               <div class="flex justify-between gap-2">
                  <!--
                  <FlowbiteButton
                     v-if="property?.isBitcasaHome"
                     :text="`Contact Agent`"
                     :show-icon="true"
                     class="h-full"
                     @click="handleShowAgentModal"
                  />
                  <FlowbiteButton
                     v-else
                     :text="`Request Tour`"
                     :show-icon="false"
                     class="secondary h-full"
                     @click="handleShowTourModal"
                  />
                  -->
                  <NuxtLinkLocale
                     v-if="!property?.isBitcasaHome"
                     :to="`/properties/${route.params.id}/rent-to-own`"
                  >
                     <FlowbiteButton
                        class="h-full"
                        :text="buttonText"
                        @click="handleClick"
                     />
                  </NuxtLinkLocale>
               </div>
               <div class="flex flex-shrink gap-2">
                  <button
                     :v-if="isSupported"
                     class="flex h-full w-12 cursor-pointer items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
                     @click="startShare"
                  >
                     <PhExport class="h-6 w-6 text-black" />
                  </button>
                  <button
                     class="flex h-full w-12 cursor-pointer items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
                     @click="toggleFavorite"
                  >
                     <PhHeartStraight
                        :class="{ 'text-resin-500': isFavorite }"
                        :weight="isFavorite ? 'fill' : 'regular'"
                        class="h-6 w-6 text-black"
                     />
                  </button>
               </div>
            </div>
         </div>

         <!-- Property Details -->
         <div class="flex-1">
            <div class="container w-11/12 xl:ml-auto xl:mr-0">
               <h1 class="text-2xl font-extrabold leading-tight">
                  {{
                     property.location?.address?.street ||
                     "Address not available"
                  }}
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
               <p
                  class="my-12 rounded-lg bg-pirate-50 py-2 text-center text-sm font-medium text-pirate-300"
               >
                  {{ property.message }}
               </p>
            </div>
            <ClientOnly fallback-tag="span">
               <DetailsMap :property="property" />
            </ClientOnly>
            <DetailsNearby :property="property" />
            <DetailsBottomBar
               class="block xl:hidden"
               :property="property"
               @show-modal="handleShowModal"
            />
         </div>
      </div>
   </section>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { fixNestedStrings } from "~/utils/jsonParser";
import {
   PhCheck,
   PhExport,
   PhHeartStraight,
   PhImages,
} from "@phosphor-icons/vue";
import { useShare } from "@vueuse/core";

const propertiesStore = usePropertiesStore();
const route = useRoute();
const error = ref(false);
const isLoading = ref(true);
const property = ref(null);
const isRequestSent = ref(false);
const isModalOpen = ref(false);
const referenceNumber = ref(0);
const email = ref("");
const formError = ref(false);
const phone = ref("");
const isFavorite = ref(null);
const showFilterDrawer = ref(false);
const showDrawer = ref(false);

const handleShowModal = () => {
   isModalOpen.value = true;
};

/*
const handleShowAgentModal = () => {
   isModalOpen.value = true;
};

const handleShowTourModal = () => {
   isModalOpen.value = true;
};
*/

const toggleFavorite = () => {
   isFavorite.value = !isFavorite.value;
   propertiesStore.toggleFavorite(property.value.id);
};

const buttonText = computed(() => {
   return route.path.includes("rent-to-own")
      ? "Rent this property"
      : "Rent-to-own";
});

const { share, isSupported } = useShare();

function startShare() {
   share({
      title: "Resin",
      text: "Check out these listings on Resin",
      url: location.href,
   });
}

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

      isFavorite.value = propertiesStore.isFavorite(foundProperty.id);

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
   if (property?.value.isBitcasaHome) {
      if (email.value || phone.value) {
         isModalOpen.value = false;
         isRequestSent.value = true;
         formError.value = false;
      } else {
         formError.value = true;
         console.log("Form error");
      }
   } else {
      isRequestSent.value = true;
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

.secondary {
   @apply bg-pirate-700 !important;
}
</style>
