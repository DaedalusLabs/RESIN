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
   </div>
</template>

<script setup>
const currentStep = ref(-1);
const MAX_STEPS = 4;

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

   if (currentStep.value > MAX_STEPS) {
      currentStep.value = -1;
   }
};

const handleBack = () => {
   if (currentStep.value > 1) {
      currentStep.value -= 1;
   }
};
</script>
