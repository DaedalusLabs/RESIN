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
   </div>
</template>

<script setup>
const currentStep = ref(-1);
const MAX_STEPS = 12;

const emit = defineEmits(["close"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
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

   console.log(currentStep.value);

   if (currentStep.value > MAX_STEPS) {
      currentStep.value = -1;
      emit("close");
   }
};

const handleBack = () => {
   if (currentStep.value > 1) {
      currentStep.value -= 1;
   }
};
</script>
