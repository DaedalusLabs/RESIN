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
               <img src="/images/upload-id.png" alt="ID upload illustration" />
            </div>

            <!-- Tips Section -->
            <div class="mx-auto max-w-md">
               <h4 class="font-bold text-pirate-950">Tips</h4>
               <ul class="mt-2 space-y-3 text-sm">
                  <li class="flex items-center gap-2">
                     <PhCheckCircle
                        :size="20"
                        weight="bold"
                        class="text-green-500"
                     />
                     <span>Upload a color photo or file</span>
                  </li>
                  <li class="flex items-center gap-2">
                     <PhCheckCircle
                        :size="20"
                        weight="bold"
                        class="text-green-500"
                     />
                     <span>Take the photo in a well-lit room</span>
                  </li>
                  <li class="flex items-center gap-2">
                     <PhXCircle :size="20" weight="bold" class="text-red-500" />
                     <span>Don't edit images of your document</span>
                  </li>
               </ul>
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
import { PhCheckCircle, PhXCircle, PhArrowLeft } from "@phosphor-icons/vue";

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
