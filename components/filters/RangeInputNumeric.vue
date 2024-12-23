<template>
    <div class="mb-6">
      <label class="filter-title" :for="id">{{ label }}</label>
      <div class="mt-2 flex items-center space-x-2">
        <input
          type="number"
          :id="`${id}-start`"
          v-model="startValue"
          :min="rangeMin"
          :max="rangeMax"
          :placeholder="placeholder || rangeMin.toString()"
          class="range-input"
          @change="emitStartValue"
        >
        <span class="px-2 text-sm font-medium">To</span>
        <input
          type="number"
          :id="`${id}-end`"
          v-model="endValue"
          :min="rangeMin"
          :max="rangeMax"
          :placeholder="placeholder || rangeMax.toString()"
          class="range-input"
          @change="emitEndValue"
        >
        <span v-if="unit" class="text-sm">{{ unit }}</span>
      </div>
    </div>
  </template>
  
  <script setup>
  import { ref, watch } from 'vue'
  
  const props = defineProps({
    id: {
      type: String,
      required: true
    },
    label: {
      type: String,
      required: true
    },
    placeholder: String,
    rangeMin: {
      type: Number,
      required: true
    },
    rangeMax: {
      type: Number,
      required: true
    },
    valueMin: Number,
    valueMax: Number,
    unit: String
  })
  
  const emit = defineEmits(['update:model-start', 'update:model-end'])
  
  const startValue = ref(props.valueMin)
  const endValue = ref(props.valueMax)
  
  watch(() => props.valueMin, (newVal) => {
    startValue.value = newVal
  })
  
  watch(() => props.valueMax, (newVal) => {
    endValue.value = newVal
  })
  
  const emitStartValue = () => {
    emit('update:model-start', startValue.value)
  }
  
  const emitEndValue = () => {
    emit('update:model-end', endValue.value)
  }
  </script>
  
  <style scoped>
  .range-input {
    @apply flex-1 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-resin-500 focus:ring-resin-500;
  }
  </style>
  