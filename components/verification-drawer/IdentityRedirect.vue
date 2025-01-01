<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-pirate-950">
            Let's verify your identity
         </h3>
      </template>
      <template #content>
        <section class="text-center">
        <p>You will be redirected to our identity provider to verify your identity.</p>
        <p>Once you are verified, you will be able to continue the verification process.</p>
        </section>
        </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhIdentificationCard, PhCamera } from "@phosphor-icons/vue";

const showDrawer = ref(true);
const emit = defineEmits(["close", "next"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

watchEffect(() => {
   showDrawer.value = props.show;
});

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   emit("next");
};
</script>
