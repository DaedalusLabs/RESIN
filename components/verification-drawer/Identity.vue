<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-gray-900">
            Let's verify your identity
         </h3>
      </template>
      <template #content>
         <div class="mx-auto mt-3 flex flex-col gap-5">
            <div class="flex items-center gap-3">
               <div class="rounded-full bg-gray-100 p-3">
                  <PhIdentificationCard :size="36" />
               </div>
               <div class="flex flex-col">
                  <span class="text-sm font-semibold text-gray-500"
                     >Step 1</span
                  >
                  <span class="text-md font-semibold text-gray-900"
                     >Provide an identity document</span
                  >
               </div>
            </div>
            <div class="flex items-center gap-3">
               <div class="rounded-full bg-gray-100 p-3">
                  <PhCamera :size="36" />
               </div>
               <div class="flex flex-col">
                  <span class="text-sm font-semibold text-gray-500"
                     >Step 2</span
                  >
                  <span class="text-md font-semibold text-gray-900"
                     >Get ready for a live face scan</span
                  >
               </div>
            </div>

            <!-- Next Button -->
            <div class="mt-6 flex w-full justify-center">
               <FlowbiteButton
                  text="Next"
                  class="px-5 py-3"
                  @click="handleNext"
               />
            </div></div
      ></template>
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
