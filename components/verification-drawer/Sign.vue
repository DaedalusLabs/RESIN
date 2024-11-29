<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-pirate-950">
            Sign your contract
         </h3>
         <p class="text-sm text-gray-500">
            Sign your contract with your autograph
         </p>
      </template>
      <template #content>
         <div
            class="mt-4 flex w-full flex-col items-center justify-center gap-4 text-gray-800"
         >
            <div
               class="flex h-56 w-full items-center justify-center rounded-md border border-gray-300"
            >
               <!-- signature canvas placeholder -->
               <div
                  class="flex h-full w-full items-center justify-center rounded-md border border-gray-300"
               >
                  <p class="text-sm text-gray-500">Sign your contract here</p>
               </div>
            </div>
         </div>
         <div class="mt-6 flex w-full justify-center">
            <FlowbiteButton
               text="Sign contract"
               class="px-5 py-3"
               @click="handleNext"
            />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
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
