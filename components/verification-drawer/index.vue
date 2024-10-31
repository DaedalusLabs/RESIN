<template>
   <div>
      <VerificationDrawerIntroduction
         :show="currentStep === 1"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerBackup
         :show="currentStep === 2"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerVerify
         :show="currentStep === 3"
         @close="handleCloseDrawer"
         @next="handleNext"
         @back="handleBack"
      />
      <VerificationDrawerValidated
         :show="currentStep === 4"
         @close="handleCloseDrawer"
         @next="handleNext"
         @back="handleBack"
      />
      <VerificationDrawerIdentity
         :show="currentStep === 5"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerCountrySelect
         :show="currentStep === 6"
         @close="handleCloseDrawer"
         @next="handleNext"
         @back="handleBack"
      />
      <VerificationDrawerIdUploadTips
         :show="currentStep === 7"
         @close="handleCloseDrawer"
         @next="handleNext"
         @back="handleBack"
      />
      <VerificationDrawerIdUpload
         :show="currentStep === 8"
         @close="handleCloseDrawer"
         @next="handleNext"
         @back="handleBack"
      />
      <VerificationDrawerIdProcessing
         :show="currentStep === 9"
         @close="handleCloseDrawer"
         @next="handleNext"
         @back="handleBack"
      />
      <VerificationDrawerPassed
         :show="currentStep === 10"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerIntroduction
         :is-verified="true"
         :show="currentStep === 11"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerInformation
         :show="currentStep === 12"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerContract
         :show="currentStep === 13"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
      <VerificationDrawerSign
         :show="currentStep === 14"
         @close="handleCloseDrawer"
         @next="handleNext"
      />
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";

const propertiesStore = usePropertiesStore();
const currentStep = ref(-1);
const MAX_STEPS = 14;

const emit = defineEmits(["close"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
   propertyId: {
      type: Number,
      required: true,
   },
});

watchEffect(() => {
   if (props.show) {
      currentStep.value = 1;
   }
});

const handleCloseDrawer = () => {
   emit("close");
   currentStep.value = -1;
};

const handleNext = () => {
   currentStep.value += 1;

   if (currentStep.value === MAX_STEPS) {
      currentStep.value = -1;
      emit("close");
      const localeRoute = useLocaleRoute();
      const route = localeRoute({
         name: "after-sign-in",
      });
      if (route) {
         propertiesStore.addOwnedProperty(props.propertyId);
         return navigateTo(route.fullPath);
      }
   }
};

const handleBack = () => {
   if (currentStep.value > 1) {
      currentStep.value -= 1;
   }
};
</script>
