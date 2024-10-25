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
   </div>
</template>

<script setup>
const currentStep = ref(-1);

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
};
</script>
