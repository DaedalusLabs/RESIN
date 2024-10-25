<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-gray-900">
            Follow the steps below
         </h3>
      </template>
      <template #content>
         <div
            class="mx-auto flex flex-col gap-5 rounded-md border border-gray-200 p-6 px-10 shadow-md"
         >
            <!-- Backup Recovery Phrase Step -->
            <div class="flex items-center space-x-3">
               <div class="flex-shrink-0">
                  <PhArrowRight
                     :size="20"
                     weight="bold"
                     class="text-resin-600"
                  />
               </div>
               <div class="flex-grow">
                  <p class="text-base text-gray-500">Backup recovery phrase</p>
               </div>
            </div>

            <!-- Verify ID Step -->
            <div class="flex items-center space-x-3">
               <div class="flex-shrink-0">
                  <PhCheckCircle
                     :size="20"
                     weight="fill"
                     class="text-gray-400"
                  />
               </div>
               <div class="flex-grow">
                  <p class="text-base text-gray-500">Verify ID</p>
               </div>
            </div>

            <!-- Sign Contract Step -->
            <div class="flex items-center space-x-3">
               <div class="flex-shrink-0">
                  <PhCheckCircle
                     :size="20"
                     weight="fill"
                     class="text-gray-400"
                  />
               </div>
               <div class="flex-grow">
                  <p class="text-base text-gray-500">Sign contract</p>
               </div>
            </div>
         </div>

         <!-- Next Button -->
         <div class="mt-6 flex w-full justify-center">
            <FlowbiteButton text="Next" class="px-5 py-3" @click="handleNext" />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowRight, PhCheckCircle } from "@phosphor-icons/vue";

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
