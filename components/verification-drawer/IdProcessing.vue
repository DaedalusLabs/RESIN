<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-gray-900">
            We are processing your data
         </h3>
      </template>
      <template #content>
         <div class="mx-auto flex flex-col gap-5">
            <div class="mx-auto flex w-1/2 justify-center">
               <img src="/images/processing-id.png" alt="Processing ID" />
            </div>
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

const handleCloseDrawer = () => {
   emit("close");
};

watchEffect(() => {
   showDrawer.value = props.show;
   if (props.show) {
      setTimeout(() => {
         emit("next");
      }, 3000);
   }
});
</script>
