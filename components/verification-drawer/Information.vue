<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3
            class="flex items-center gap-2 text-xl font-semibold text-pirate-950"
         >
            Your information
         </h3>
         <p>For the Rent-To-Own agreement</p>
      </template>
      <template #content>
         <form @submit.prevent="handleNext">
            <div class="max-h-[300px] space-y-4 overflow-scroll">
               <input type="text" placeholder="First name" class="form-input" />
               <input type="text" placeholder="Last name" class="form-input" />
               <input type="email" placeholder="Email" class="form-input" />
               <input type="text" placeholder="Address" class="form-input" />
               <input type="text" placeholder="City" class="form-input" />
               <input type="text" placeholder="State" class="form-input" />
               <!-- Zip Code and country -->
               <input type="text" placeholder="Zip Code" class="form-input" />
               <input type="text" placeholder="Country" class="form-input" />
            </div>
            <!-- Navigation Buttons -->
            <div class="mt-6 flex justify-center gap-4">
               <FlowbiteBorderButton
                  text="Back"
                  class="px-5 py-3"
                  :show-icon="false"
                  @click="handleBack"
               >
                  <template #icon-prepend>
                     <PhArrowLeft :size="24" weight="bold" />
                  </template>
               </FlowbiteBorderButton>
               <FlowbiteBorderButton
                  text="Next"
                  class="px-5 py-3 transition-all duration-200"
                  :class="{
                     'cursor-not-allowed bg-gray-100 opacity-50 hover:bg-gray-100':
                        !isCompleteAndCorrect,
                  }"
                  @click="handleNext"
               >
                  <template #icon-append>
                     <PhArrowRight :size="24" weight="bold" />
                  </template>
               </FlowbiteBorderButton>
            </div>
         </form>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowLeft, PhArrowRight } from "@phosphor-icons/vue";
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
</script>

<style scoped>
.form-input {
   @apply w-full rounded-lg border border-gray-300 px-2 py-4 focus:outline-none focus:ring-0;
}
</style>
