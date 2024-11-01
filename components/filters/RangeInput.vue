<template>
   <div class="mb-6">
      <label class="filter-title" :for="id">{{ label }}</label>
      <div class="mt-2 flex items-center space-x-2">
         <!-- Start Range Dropdown -->
         <select
            :id="id + '-start'"
            class="input-base"
            :value="modelStartValue"
            @change="updateStartValue"
         >
            <option
               v-for="option in rangeOptions"
               :key="option"
               :value="option"
            >
               {{ option }} {{ unit }}
            </option>
         </select>

         <span>To</span>

         <!-- End Range Dropdown -->
         <select
            :id="id + '-end'"
            class="input-base"
            :value="modelEndValue"
            @change="updateEndValue"
         >
            <option
               v-for="option in rangeOptions"
               :key="option"
               :value="option"
            >
               {{ option !== Infinity ? option + " " + unit : "No max." }}
            </option>
         </select>
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
      default: 0,
   },
   modelEndValue: {
      type: Number,
      required: true,
      default: Infinity,
   },
   unit: {
      type: String,
      required: true,
   },
});

// Define the range options for dropdown
const rangeOptions = [0, 500, 1000, 5000, 10000, 20000, Infinity];

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
