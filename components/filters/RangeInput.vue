<template>
   <div class="mb-6">
      <label class="filter-title" :for="id">{{ label }}</label>
      <div class="mt-2 flex items-center space-x-2">
         <!-- Start Range Dropdown -->
         <select
            :id="id + '-start'"
            v-model="localModelStartValue"
            class="flex-1 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-resin-500 focus:ring-resin-500"
            @change="updateStartValue"
         >
            <option
               v-for="option in rangeOptions"
               :key="option"
               :value="option"
               :selected="option === localModelStartValue"
            >
               {{ option === Infinity ? "No max." : option + " " + unit }}
            </option>
         </select>

         <span class="px-2 text-sm font-medium">To</span>

         <!-- End Range Dropdown -->
         <select
            :id="id + '-end'"
            v-model="localModelEndValue"
            class="flex-1 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-resin-500 focus:ring-resin-500"
            @change="updateEndValue"
         >
            <option
               v-for="option in rangeOptions"
               :key="option"
               :value="option"
               :selected="option === localModelEndValue"
            >
               {{ option === Infinity ? "No max." : option + " " + unit }}
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
   valueMin: {
      type: Number,
      required: true,
   },
   valueMax: {
      type: Number,
      required: true,
   },
   unit: {
      type: String,
      required: true,
   },
});

const localModelStartValue = ref(props.modelStartValue);
const localModelEndValue = ref(props.modelEndValue);

// Define the range options for dropdown
const rangeOptions = [0, 500, 1000, 5000, 10000, 20000, Infinity];

const updateStartValue = () => {
   if (localModelStartValue.value > localModelEndValue.value) {
      localModelEndValue.value = localModelStartValue.value;
   }
   emit("update:modelStartValue", localModelStartValue.value);
};

const updateEndValue = () => {
   if (localModelEndValue.value < localModelStartValue.value) {
      localModelStartValue.value = localModelEndValue.value;
   }
   emit("update:modelEndValue", localModelEndValue.value);
};

watch(props.modelStartValue, (newVal) => {
   localModelStartValue.value = newVal;
});

watch(props.modelEndValue, (newVal) => {
   localModelEndValue.value = newVal;
});
</script>
