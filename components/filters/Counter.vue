<template>
   <div class="mb-6">
      <label class="filter-title">{{ label }}</label>
      <div class="relative flex items-center">
         <button
            type="button"
            class="counter-button rounded-s-lg"
            @click="decrease"
         >
            <PhMinus class="h-3 w-3" />
         </button>
         <input
            v-model="displayValue"
            type="text"
            class="counter-input"
            @input="updateValue"
         />
         <button
            type="button"
            class="counter-button rounded-e-lg"
            @click="increase"
         >
            <PhPlus class="h-3 w-3" />
         </button>
      </div>
   </div>
</template>

<script setup>
import { PhMinus, PhPlus } from "@phosphor-icons/vue";

const props = defineProps({
   modelValue: {
      type: Number,
      required: true,
   },
   label: {
      type: String,
      required: true,
   },
   min: {
      type: Number,
      default: 0,
   },
});

const emit = defineEmits(["update:modelValue"]);

const displayValue = ref(props.modelValue);

const updateValue = () => {
   const value = parseInt(displayValue.value) || 0;
   emit("update:modelValue", value);
};

const decrease = () => {
   if (props.modelValue > (props.min ?? 0)) {
      emit("update:modelValue", props.modelValue - 1);
   }
};

const increase = () => {
   emit("update:modelValue", props.modelValue + 1);
};

watch(
   () => props.modelValue,
   (newValue) => {
      displayValue.value = newValue;
   },
);
</script>
