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
            v-model="localValue"
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
const emit = defineEmits(["update:modelValue"]);

const props = defineProps({
   label: {
      type: String,
      required: true,
   },
   id: {
      type: String,
      required: true,
   },
   placeholder: {
      type: String,
      required: true,
   },
   modelValue: {
      type: Number,
      required: true,
   },
});

const localValue = ref(props.modelValue);

watch(props.modelValue, (newVal) => {
   localValue.value = newVal;
});

const updateValue = (event) => {
   const value = Number(event.target.value);
   emit("update:modelValue", value);
};

const decrease = () => {
   if (localValue.value > 0) {
      localValue.value -= 1;
      emit("update:modelValue", localValue.value);
   }
};

const increase = () => {
   localValue.value += 1;
   emit("update:modelValue", localValue.value);
};
</script>
