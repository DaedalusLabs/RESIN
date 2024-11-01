<template>
   <div class="mb-6">
      <span class="filter-title">{{ label }}</span>
      <div class="mt-2">
         <div class="flex flex-col space-y-2">
            <div v-for="option in visibleOptions" :key="option">
               <input
                  :id="option"
                  :checked="modelValue.includes(option)"
                  type="checkbox"
                  class="checkbox-base"
                  :value="option"
                  @change="handleChange"
               />
               <span class="ml-2 text-sm">{{ option }}</span>
            </div>
            <div class="pt-2">
               <button
                  v-if="props.options.length > 4"
                  class="w-full text-start text-sm text-resin-500"
                  @click="toggleShowMore"
               >
                  {{
                     showMore
                        ? "Show Less"
                        : "Show More (" + (props.options.length - 4) + ")"
                  }}
               </button>
            </div>
         </div>
      </div>
   </div>
</template>

<script setup>
import { ref, watch } from "vue";

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
      type: Array,
      required: true,
   },
});

const localModelValue = ref(props.modelValue);
const showMore = ref(false);

watch(props.modelValue, (newVal) => {
   localModelValue.value = newVal;
});

const visibleOptions = computed(() => {
   return showMore.value ? props.options : props.options.slice(0, 4);
});

const handleChange = (event) => {
   const value = event.target.value;
   if (localModelValue.value.includes(value)) {
      localModelValue.value = localModelValue.value.filter((v) => v !== value);
   } else {
      localModelValue.value.push(value);
   }
   emit("update:modelValue", localModelValue.value);
};

const toggleShowMore = () => {
   showMore.value = !showMore.value;
};
</script>
