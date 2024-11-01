<template>
   <div class="mb-6">
      <span class="filter-title">{{ label }}</span>
      <div class="mt-2">
         <div class="flex flex-col space-y-2">
            <div v-for="option in options" :key="option">
               <input
                  :id="option"
                  :checked="modelValue.includes(option)"
                  type="radio"
                  class="radio-base"
                  :value="option"
                  @change="handleChange"
               />
               <span class="ml-2 text-sm">{{ option }}</span>
            </div>
         </div>
      </div>
   </div>
</template>

<script setup>
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
   options: {
      type: Array,
      required: true,
   },
   modelValue: {
      type: String,
      required: true,
   },
});

const localModelValue = ref(props.modelValue);

watch(props.modelValue, (newVal) => {
   localModelValue.value = newVal;
});

const handleChange = (event) => {
   emit("update:modelValue", event.target.value);
};
</script>
