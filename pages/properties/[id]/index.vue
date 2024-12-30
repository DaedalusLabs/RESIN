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
      <ClientOnly fallback-tag="span">
        
         <ModalContactAgent
            v-if="isBuyNow"
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
      </ClientOnly>
      

   

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
            v-if="property && property['resin-type'] === 'Buy Now'"
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
                 
                  <NuxtLinkLocale
                     v-if="property && property['resin-type'] === 'Rent to Own'"
                     :to="`/properties/${route.params.id}/rent-to-own`"
                  >
                     <FlowbiteButton
                        class="h-full"
                        :text="buttonText"
                        @click="handleClick"
                     />
                  </NuxtLinkLocale>
                  <NuxtLinkLocale
                     v-else
                   
                  >
                     <FlowbiteButton
                        class="h-full"
                        :text="'Contact agent'"
                        @click="handleShowModal"
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
                     property.title ||
                     "Address not available"
                  }}
               </h1>
               <p class="mt-1 text-sm">
                  {{ property.location?.street }},
                  {{ property.location?.city }},
                  {{ property.location?.country }}
               </p>
               <ResinAlert
                  :show="showSuccessAlert"
                  text="Your information has been submitted successfully."
               />
               <DetailsSize :property="property" />
               <DetailsPrices :property="property" />
               <DetailsSummary :property="property" />
               <DetailsKeyFeatures :property="property" />
               <DetailsAdditional :property="property" />
               <p
                  v-if="property.attribution && property.attribution?.length > 0"
                  class="my-12 rounded-lg bg-pirate-50 py-2 text-center text-sm font-medium text-pirate-300"
               >
                  {{ property.attribution }}
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
import { ref, computed, onMounted } from "vue";

const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();
const appConfig = useAppConfig();

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
const showSuccessAlert = ref(false);

const handleShowModal = () => {
   isModalOpen.value = true;
};

const generateRef = () => {
   return Math.floor(Math.random() * 9000) + 1000;
};

const buttonText = computed(() => {
   return property.value && property.value['resin-type'] !== 'Rent to Own'
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

const toggleFavorite = () => {
   isFavorite.value = !isFavorite.value;
   propertiesStore.toggleFavorite(property.value.id);
};

onMounted(() => {
   referenceNumber.value = generateRef();
});

onMounted(async () => {
   try {
      const foundProperty = await propertiesStore.get(
         route.params.id,
      );

      if (!foundProperty) {
         error.value = true;
         return;
      }

      isFavorite.value = propertiesStore.isFavorite(foundProperty.id);

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

const handleSendRequest = async() => {
   isModalOpen.value = false;
   
   showSuccessAlert.value = true;
   setTimeout(() => {
      showSuccessAlert.value = false;
   }, 5000);  
};

const propertyAddress = computed(() => {
   return `${property.value.location.street}, ${property.value.location.city}, ${property.value.location.country}`;
});

const isBuyNow = computed(() => {
   return property.value && property.value['resin-type'] === 'Buy Now';
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
