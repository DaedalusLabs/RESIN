<template>
   <div
      class="z-1 fixed bottom-14 left-0 w-full bg-black/30 p-4 backdrop-blur-md"
   >
      <div
         class="mx-auto flex w-11/12 justify-end"
         :class="{ 'justify-between': property && property['resin-type'] !== 'Rent' }"
      >
         <FlowbiteButton
            v-if="property &&property['resin-type'] === 'Buy Now'"
            :text="`Contact Agent`"
            :show-icon="true"
            @click="handleShowAgentModal"
         />
         <FlowbiteButton
            v-else
            :text="`Request Tour`"
            :show-icon="false"
            class="secondary"
            @click="handleShowTourModal"
         />
         <NuxtLinkLocale
            v-if="property && property['resin-type'] === 'Rent'"
            :to="`/properties/${route.params.id}/rent-to-own`"
         >
            <FlowbiteButton :text="buttonText" @click="handleClick" />
         </NuxtLinkLocale>
      </div>
   </div>
</template>

<script setup>
const route = useRoute();

defineProps({
   property: {
      type: Object,
      required: true,
   },
});

const emit = defineEmits(["show-drawer", "show-modal"]);

const handleShowTourModal = () => {
   emit("show-modal");
};

const handleShowAgentModal = () => {
   emit("show-modal");
};

const buttonText = computed(() => {
   return route.path.includes("rent-to-own")
      ? "Rent this property"
      : "Rent-to-own";
});

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
