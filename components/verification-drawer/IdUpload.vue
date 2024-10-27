<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-gray-900">
            Get ready to upload your ID
         </h3>
      </template>
      <template #content>
         <div class="mx-auto flex flex-col gap-5">
            <!-- Image Placeholder -->
            <div class="flex justify-center">
               <img src="/images/mock-id.png" alt="Mock ID" />
            </div>
         </div>

         <!-- Navigation Buttons -->
         <div class="mt-6 flex w-full justify-center gap-4">
            <FlowbiteBorderButton
               text="Back"
               :show-icon="false"
               @click="handleBack"
            >
               <template #icon-prepend>
                  <PhArrowLeft :size="24" weight="bold" />
               </template>
            </FlowbiteBorderButton>
            <FlowbiteButton text="Next" class="px-5 py-3" @click="handleNext" />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowLeft } from "@phosphor-icons/vue";
const showDrawer = ref(true);
const emit = defineEmits(["close", "next", "back"]);

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

const handleBack = () => {
   emit("back");
};
</script>
