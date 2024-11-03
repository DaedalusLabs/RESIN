<template>
   <div
      class="z-top fixed bottom-14 left-0 w-full bg-black/30 p-4 backdrop-blur-md"
   >
      <FlowbiteModal
         ref="tourModal"
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

      <div class="mx-auto flex w-11/12 justify-between">
         <FlowbiteButton
            :text="`Request Tour`"
            :show-icon="false"
            class="secondary"
            @click="handleShowTourModal"
         />
         <NuxtLink
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
const tourModal = ref(null);
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
