<template>
   <div
      class="z-1 fixed bottom-14 left-0 w-full bg-black/30 p-4 backdrop-blur-md"
   >
      <div
         class="mx-auto flex w-11/12 justify-end"
    
      >
         <FlowbiteButton
            v-if="property &&property['resin-type'] === 'Buy Now'"
            :text="$t('property.details.actions.contactAgent')"
            :show-icon="true"
            @click="handleShowAgentModal"
         />
         <section v-else>
            <FlowbiteButton
               
               :text="$t('property.details.actions.requestTour')"
               :show-icon="false"
               class="secondary"
               @click="handleShowTourModal"
            />
            <NuxtLinkLocale
               v-if="property && property['resin-type'] !== 'Buy now'"
               :to="`/properties/${route.params.id}/rent-to-own`"
            >
               <FlowbiteButton :text="buttonText" @click="handleClick" />
            </NuxtLinkLocale>
         </section>
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
      ? $t('property.details.actions.rentProperty')
      : $t('property.details.actions.rentToOwn');
});

const handleClick = () => {
   if (buttonText.value === $t('property.details.actions.rentProperty')) {
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
