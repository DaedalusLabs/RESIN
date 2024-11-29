<template>
   <FlowbiteDrawer
      :is-open="showDrawer"
      slide-from="side"
      @close="handleCloseDrawer"
   >
      <template #title>
         <div>
            <h2 class="text-lg font-semibold">Menu</h2>
         </div>
      </template>

      <template #content>
         <div class="p-4">
            <ul>
               <li>
                  <a href="#">Home</a>
               </li>
               <li>
                  <a href="#">About</a>
               </li>
               <li>
                  <a href="#">Contact</a>
               </li>
            </ul>
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
const emit = defineEmits(["close"]);
defineProps({
   showDrawer: Boolean,
});

const handleCloseDrawer = () => {
   emit("close");
};
</script>
