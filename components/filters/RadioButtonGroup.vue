<template>
   <div class="mb-6">
      <span class="filter-title">{{ label }}</span>
      <div class="mt-2">
         <div class="flex flex-col space-y-2">
            <!-- Add 'v-bind:key' here with a unique identifier -->
            <div v-for="option in options" :key="option">
               <input
                  :id="option"
                  v-model="displayValue"
                  :value="option"
                  type="radio"
                  class="radio-base"
                  @input="updateValue"
               />
               <span class="ml-2 text-sm">{{ option }}</span>
            </div>
         </div>
      </div>
   </div>
</template>

<script setup lang="ts">
const props = defineProps({
   label: {
      type: String,
      required: true,
   },
   options: {
      type: Array as PropType<string[]>,
      required: true,
   },
   modelValue: {
      type: String,
      required: true,
   },
});

const emit = defineEmits(["update:modelValue"]);

const updateValue = () => {
   emit("update:modelValue", props.modelValue);
};

const displayValue = ref(props.modelValue);

watch(displayValue, (value) => {
   emit("update:modelValue", value);
});
</script>
