<template>
   <div class="mb-6">
      <label class="filter-title" :for="id">{{ label }}</label>
      <div class="mt-2 flex items-center space-x-2">
         <input
            :value="modelStartValue"
            type="number"
            class="input-base"
            :placeholder="placeholder"
            min="0"
            @change="updateStartValue"
         />
         <span>To</span>
         <input
            :value="modelEndValue"
            type="number"
            class="input-base"
            :placeholder="placeholder"
            :min="modelStartValue"
            @change="updateEndValue"
         />
      </div>
   </div>
</template>

<script setup>
const emit = defineEmits(["update:modelStartValue", "update:modelEndValue"]);

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
   modelStartValue: {
      type: Number,
      required: true,
   },
   modelEndValue: {
      type: Number,
      required: true,
   },
});

const updateStartValue = (event) => {
   const value = Number(event.target.value);
   emit("update:modelStartValue", value);
   if (value > props.modelEndValue) {
      emit("update:modelEndValue", value);
   }
};

const updateEndValue = (event) => {
   const value = Number(event.target.value);
   emit("update:modelEndValue", value);
   if (value < props.modelStartValue) {
      emit("update:modelStartValue", value);
   }
};
</script>
