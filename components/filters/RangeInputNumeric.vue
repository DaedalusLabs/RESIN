<template>
   <div class="mb-6">
      <label class="filter-title" :for="id">{{ label }}</label>
      <div class="mt-2 flex items-center space-x-2">
         <input
            :id="`${id}-start`"
            v-model="startValue"
            type="number"
            :min="rangeMin"
            :max="rangeMax"
            :placeholder="placeholder || rangeMin.toString()"
            class="range-input"
            @change="emitStartValue"
         />
         <span class="px-2 text-sm font-medium">To</span>
         <input
            :id="`${id}-end`"
            v-model="endValue"
            type="number"
            :min="rangeMin"
            :max="rangeMax"
            :placeholder="placeholder || rangeMax.toString()"
            class="range-input"
            @change="emitEndValue"
         />
         <span v-if="unit" class="text-sm">{{ unit }}</span>
      </div>
   </div>
</template>

<script setup>
import { ref, watch } from "vue";

const props = defineProps({
   id: {
      type: String,
      required: true,
   },
   label: {
      type: String,
      required: true,
   },
   placeholder: {
      type: String,
      required: false,
      default: null,
   },
   rangeMin: {
      type: Number,
      required: true,
   },
   rangeMax: {
      type: Number,
      required: true,
   },
   valueMin: {
      type: Number,
      required: false,
      default: null,
   },
   valueMax: {
      type: Number,
      required: false,
      default: null,
   },
   unit: {
      type: String,
      required: false,
      default: null,
   },
});

const emit = defineEmits(["update:model-start", "update:model-end"]);

const startValue = ref(props.valueMin);
const endValue = ref(props.valueMax);

watch(
   () => props.valueMin,
   (newVal) => {
      startValue.value = newVal;
   },
);

watch(
   () => props.valueMax,
   (newVal) => {
      endValue.value = newVal;
   },
);

const emitStartValue = () => {
   emit("update:model-start", startValue.value);
};

const emitEndValue = () => {
   emit("update:model-end", endValue.value);
};
</script>

<style scoped>
.range-input {
   @apply flex-1 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-resin-500 focus:ring-resin-500;
}
</style>
