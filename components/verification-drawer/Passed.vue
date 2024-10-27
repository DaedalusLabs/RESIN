<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title />
      <template #content>
         <div
            class="mt-4 flex w-full flex-col items-center justify-center gap-4 text-gray-800"
         >
            <PhCheckCircle :size="96" class="text-green-300" />

            <h3
               class="flex items-center gap-2 text-xl font-semibold text-pirate-950"
            >
               Verification passed succesfully!
            </h3>
            <FlowbiteButton
               text="Continue"
               class="px-5 py-3"
               @click="handleNext"
            />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhCheckCircle } from "@phosphor-icons/vue";

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
