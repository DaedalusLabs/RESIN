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
                  {{ $t("property.notFound.title") }}
               </h3>
               <div class="mt-2 text-sm text-red-700">
                  <p>
                     {{ $t("property.notFound.message") }}
                  </p>
               </div>
               <div class="mt-4">
                  <NuxtLinkLocale
                     to="properties"
                     class="text-sm font-medium text-red-800 hover:text-red-900"
                  >
                     {{ $t("property.notFound.backToProperties") }}
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
            :property="property"
            @update:is-open="isModalOpen = $event"
            @send-request="handleSendRequest"
         />
         <ModalRequestTour
            v-else
            :is-open="isModalOpen"
            :is-request-sent="isRequestSent"
            :property="property"
            :reference-number="referenceNumber"
            @update:is-open="isModalOpen = $event"
            @send-request="handleSendRequest"
         />
      </ClientOnly>

      <BackgroundOverlay :show="showDrawer" @close="showDrawer = false" />

      <FlowbiteImageDrawer
         :show-drawer="showDrawer"
         :images="property.images"
         @close="showDrawer = false"
      />

      <div class="relative xl:hidden">
         <div v-if="property?.images?.length" class="relative h-64">
            <div v-if="property.images[0]?.blurhash" class="absolute inset-0">
               <BlurhashCanvas
                  :hash="property.images[0].blurhash"
                  :width="320"
                  :height="240"
                  class="h-full w-full"
                  :style="{ display: mobileImageLoaded ? 'none' : 'block' }"
               />
            </div>
            <NuxtImg
               :src="property.images[0]?.files[2]?.url"
               :srcset="
                  property.images[0]?.files
                     ?.map((file) => `${file.url} ${file.width}w`)
                     .join(', ')
               "
               sizes="(max-width: 1280px) 100vw, 1280px"
               alt="Property Image"
               class="absolute inset-0 h-full w-full object-cover"
               @load="mobileImageLoaded = true"
            />
         </div>

         <span
            v-if="property && property['resin-type'] === 'Buy Now'"
            class="absolute bottom-4 right-4 z-10 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md hover:border-white hover:bg-resin-500 hover:text-white"
         >
            {{ $t("property.types.buyNow") }}
         </span>
         <span
            v-else
            class="absolute bottom-4 right-4 z-10 cursor-default rounded-full border-2 border-resin-500 bg-white px-2 py-1 text-xs font-semibold text-resin-500 shadow-md hover:border-white hover:bg-resin-500 hover:text-white"
         >
            {{ $t("property.types.rentToOwn") }}
         </span>

         <DetailsTopBar :property="property" />
      </div>

      <div class="mx-auto mt-16 flex w-10/12 justify-between lg:w-9/12">
         <div
            class="sticky top-16 hidden flex-shrink flex-col gap-5 lg:w-[40%] xl:flex"
         >
            <div class="relative h-96">
               <div v-if="property?.images?.length">
                  <div
                     v-if="property.images[0]?.blurhash"
                     class="absolute inset-0"
                  >
                     <BlurhashCanvas
                        :hash="property.images[0].blurhash"
                        :width="320"
                        :height="240"
                        class="h-full w-full"
                        :style="{
                           display: desktopImageLoaded ? 'none' : 'block',
                        }"
                     />
                  </div>
                  <NuxtImg
                     :src="property.images[0]?.files[2]?.url"
                     :srcset="
                        property.images[0]?.files
                           ?.map((file) => `${file.url} ${file.width}w`)
                           .join(', ')
                     "
                     sizes="(max-width: 1280px) 40vw, 512px"
                     alt="Property Image"
                     class="absolute inset-0 h-full w-full object-cover"
                     @load="desktopImageLoaded = true"
                  />
               </div>
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
                  <NuxtLinkLocale v-else>
                     <FlowbiteButton
                        class="h-full"
                        :text="$t('property.actions.contactAgent')"
                        @click="handleShowModal"
                     />
                  </NuxtLinkLocale>
               </div>
               <div class="flex flex-shrink gap-2">
                  <button
                     :v-if="isSupported"
                     class="flex h-full w-12 cursor-pointer items-center justify-center rounded-full border-2 bg-white shadow-md hover:border-resin-500"
                     :title="$t('property.actions.share')"
                     @click="startShare"
                  >
                     <PhExport class="h-6 w-6 text-black" />
                  </button>
                  <button
                     class="flex h-full w-12 items-center justify-center rounded-full border-2 bg-white shadow-md"
                     :class="
                        isAuthenticated
                           ? 'cursor-pointer hover:border-resin-500'
                           : 'cursor-not-allowed opacity-50'
                     "
                     :title="$t('property.actions.favorite')"
                     @click="toggleFavorite"
                  >
                     <PhHeartStraight
                        :class="[
                           isAuthenticated ? 'text-black' : 'text-pirate-200',
                           { 'text-resin-500': isFavorite },
                        ]"
                        :weight="isFavorite ? 'fill' : 'regular'"
                        class="h-6 w-6"
                     />
                  </button>
               </div>
            </div>
         </div>

         <!-- Property Details -->
         <div class="flex-1">
            <div class="container w-full xl:ml-auto xl:mr-0 xl:w-11/12">
               <h1 class="text-2xl font-extrabold leading-tight">
                  {{
                     property.title ||
                     $t("property.details.addressNotAvailable")
                  }}
               </h1>
               <p class="mt-1 text-sm">
                  {{ property.location?.street }},
                  {{ property.location?.city }},
                  {{ property.location?.country }}
               </p>
               <ResinAlert
                  :show="showSuccessAlert"
                  :text="$t('property.details.requestSuccess')"
               />
               <DetailsSize :property="property" />
               <DetailsPrices :property="property" />
               <DetailsSummary :property="property" />
               <DetailsKeyFeatures :property="property" />
               <DetailsAdditional :property="property" />
               <p
                  v-if="
                     property.attribution && property.attribution?.length > 0
                  "
                  class="my-12 rounded-lg bg-pirate-50 py-2 text-center text-sm font-medium text-pirate-300"
               >
                  {{ property.attribution }}
               </p>
               <ClientOnly fallback-tag="span">
                  <DetailsMap :property="property" />
               </ClientOnly>
            </div>
         </div>

         <DetailsBottomBar
            class="block xl:hidden"
            :property="property"
            @show-modal="handleShowModal"
         />
      </div>
      <div class="mx-auto mt-8 px-4 md:container md:px-0 xl:w-10/12 xl:px-16">
         <DetailsNearby :property="property" />
      </div>
   </section>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { PhImages, PhExport, PhHeartStraight } from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";
import { useNostrStore } from "~/stores/nostr";
import { fixNestedStrings } from "~/utils/jsonParser";
import ModalContactAgent from "~/components/Modal/contact-agent.vue";
import ModalRequestTour from "~/components/Modal/request-tour.vue";
import { propertyImageUtils } from "~/types/property";
import type { Property } from "~/types/property";
import BlurhashCanvas from "~/components/Flowbite/BlurhashCanvas.vue";

const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();
const { t } = useI18n();

const route = useRoute();
const router = useRouter();
const error = ref(false);
const isLoading = ref(true);
const property = ref<Property | null>(null);
const isRequestSent = ref(false);
const isModalOpen = ref(false);
const referenceNumber = ref(0);
const showDrawer = ref(false);
const showSuccessAlert = ref(false);

const isAuthenticated = computed(() => nostrStore.isAuthenticated);
const isFavorite = computed(() =>
   property.value ? propertiesStore.isFavorite(property.value.id) : false,
);

// Add refs for tracking image load states
const mobileImageLoaded = ref(false);
const desktopImageLoaded = ref(false);

onMounted(async () => {
   try {
      const foundProperty = await propertiesStore.get(
         route.params.id as string,
      );

      if (!foundProperty) {
         error.value = true;
         return;
      }

      property.value = foundProperty;

      if (typeof property.value === "string") {
         fixNestedStrings(property);
      }

      // Check if current URL matches the expected slug
      const currentPath = route.path;
      const expectedPath = `/properties/${route.params.id}/${property.value.slug}`;

      if (property.value.slug && currentPath !== expectedPath) {
         // Redirect to the correct URL with the slug
         await router.replace(expectedPath);
         return;
      }

      propertiesStore.addViewedProperty(route.params.id as string);
   } catch (e) {
      console.error("Error loading property:", e);
      error.value = true;
   } finally {
      isLoading.value = false;
   }
});

const handleShowModal = () => {
   isModalOpen.value = true;
};

const buttonText = computed(() => {
   return property.value && property.value["resin-type"] !== "Rent to Own"
      ? t("property.actions.rentProperty")
      : t("property.actions.rentToOwn");
});

const { share, isSupported } = useShare();

function startShare() {
   share({
      title: `${property.value?.title} in ${property.value?.location?.city}, ${property.value?.location?.country}`,
      text: "Check out these listings on Resin",
      url: location.href,
   });
}

const toggleFavorite = () => {
   if (isAuthenticated.value && property.value) {
      isFavorite.value = !isFavorite.value;
      propertiesStore.toggleFavorite(property.value.id);
   }
};

onMounted(() => {
   referenceNumber.value = 0;
});

const handleSendRequest = async () => {
   isModalOpen.value = false;

   showSuccessAlert.value = true;
   setTimeout(() => {
      showSuccessAlert.value = false;
   }, 5000);
};

const isBuyNow = computed(() => {
   return property.value && property.value["resin-type"] === "Buy Now";
});

definePageMeta({
   layout: "white",
});

useHead({
   title: () =>
      property.value
         ? `${property.value.title} in ${property.value.location?.city}, ${property.value.location?.country}`
         : "Property Details",
   meta: [
      {
         name: "og:title",
         content: () =>
            property.value
               ? `${property.value.title} in ${property.value.location?.city}, ${property.value.location?.country}`
               : "Property Details",
      },
      {
         name: "og:description",
         content: () =>
            property.value
               ? `${property.value.description}`
               : "Property Details",
      },
      {
         name: "og:image",
         content: () =>
            property.value
               ? `${propertyImageUtils.getSmallestImage(property.value.images[0]?.files)?.url}`
               : "/android-chrome-256x256.png",
      },
      {
         name: "description",
         content: () =>
            property.value
               ? `${property.value.description}`
               : "Property Details",
      },
   ],
});

useSeoMeta({
   title: () =>
      property.value
         ? `${property.value.title} in ${property.value.location?.city}, ${property.value.location?.country}`
         : "Property Details",
   description: () =>
      property.value ? `${property.value.description}` : "Property Details",
   ogTitle: () =>
      property.value
         ? `${property.value.title} in ${property.value.location?.city}, ${property.value.location?.country}`
         : "Property Details",
   ogDescription: () =>
      property.value ? `${property.value.description}` : "Property Details",
   ogImage: () =>
      property.value
         ? `${propertyImageUtils.getSmallestImage(property.value.images[0]?.files)?.url}`
         : "/android-chrome-256x256.png",
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
