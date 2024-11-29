<template>
   <div
      class="z-top fixed bottom-14 left-0 w-full bg-black/30 p-4 backdrop-blur-md"
   >
      <FlowbiteModal
         v-if="property.isBitcasaHome"
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

      <div
         v-if="!isModalOpen"
         class="mx-auto flex w-11/12 justify-center"
         :class="{ 'justify-between': !property.isBitcasaHome }"
      >
         <FlowbiteButton
            v-if="property.isBitcasaHome"
            :text="`Contact Agent`"
            :show-icon="false"
            @click="handleShowAgentModal"
         />
         <FlowbiteButton
            v-else
            :text="`Request Tour`"
            :show-icon="false"
            class="secondary"
            @click="handleShowTourModal"
         />
         <NuxtLink
            v-if="!property.isBitcasaHome"
            :to="localePath(`/properties/${route.params.id}/rent-to-own`)"
         >
            <FlowbiteButton :text="buttonText" @click="handleClick" />
         </NuxtLink>
      </div>
   </div>
</template>

<script setup>
import { PhCheck } from "@phosphor-icons/vue";
const route = useRoute();
const isModalOpen = ref(false);
const referenceNumber = ref(0);
const isRequestSent = ref(false);

const props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

const handleShowTourModal = () => {
   isModalOpen.value = true;
};

const handleShowAgentModal = () => {
   isModalOpen.value = true;
};

const handleSendRequest = () => {
   isRequestSent.value = true;
};

const propertyAddress = computed(() => {
   return `${props.property.location.address.street}, ${props.property.location.address.city}, ${props.property.location.address.country}`;
});

const generateRef = () => {
   return Math.floor(Math.random() * 9000) + 1000;
};

onMounted(() => {
   referenceNumber.value = generateRef();
});

const buttonText = computed(() => {
   return route.path.includes("rent-to-own")
      ? "Rent this property"
      : "Rent-to-own";
});

const emit = defineEmits(["show-drawer"]);

const handleClick = () => {
   if (buttonText.value === "Rent this property") {
      emit("show-drawer");
   }
};
</script>

<style scoped>
/* overwriting default resin color of button component */
.secondary {
   @apply bg-pirate-700 !important;
}
</style>
