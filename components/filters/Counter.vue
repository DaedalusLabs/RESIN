<template>
   <div class="mb-6">
      <label class="filter-title">{{ label }}</label>
      <div class="relative flex items-center">
         <button
            type="button"
            class="h-11 rounded-s-lg border border-gray-300 bg-gray-100 p-3 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-100"
            @click="decrease"
         >
            <PhMinus class="h-3 w-3" weight="bold" />
         </button>
         <input
            v-model="localValue"
            type="text"
            class="block h-11 w-full border-x-0 border-gray-300 bg-gray-50 py-2.5 text-center text-sm text-gray-900"
            @input="updateValue"
         />
         <button
            type="button"
            class="h-11 rounded-e-lg border border-gray-300 bg-gray-100 p-3 hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-100"
            @click="increase"
         >
            <PhPlus class="h-3 w-3" weight="bold" />
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
