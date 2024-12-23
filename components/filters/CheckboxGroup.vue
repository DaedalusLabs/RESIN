<template>
   <div class="mb-6">
      <span class="filter-title">{{ label }}</span>
      <div class="mt-2">
         <div class="flex flex-col space-y-2">
            <div v-for="option in visibleOptions" :key="option.value" class="flex justify-between items-center">
               <input
                  :id="option.value"
                  :checked="option.isRefined"
                  type="checkbox"
                  class="h-4 w-4 rounded border-gray-300 bg-gray-100 text-resin-500 focus:ring-2 focus:ring-resin-500"
                  :value="option.value"
                  @change="handleChange"
               />
               <div class="ml-2 text-sm grow">{{ props.valueProperty ? option[valueProperty] : option.label }}</div>
               <div class="text-sm text-gray-400">{{ option.count }}</div>
            </div>
            <div v-show="props.options.length > 5" class="pt-2">
               <button
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
import { ref, computed } from "vue";

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
   valueProperty: {
      type: String,
      required: false,
   }
});

const showMore = ref(false);

const visibleOptions = computed(() => {
   return showMore.value ? props.options : props.options.slice(0, 4);
});

const handleChange = (event) => {
   emit("update:modelValue", event);
};

const toggleShowMore = () => {
   showMore.value = !showMore.value;
};
</script>
