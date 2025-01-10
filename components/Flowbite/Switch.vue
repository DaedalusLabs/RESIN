<template>
  <label class="relative inline-flex cursor-pointer items-center">
    <input
      type="checkbox"
      :checked="modelValue"
      class="peer sr-only"
      @change="handleChange"
    />
    <div
      class="peer h-6 w-11 rounded-full bg-gray-200 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-resin-500 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-resin-300 dark:border-gray-600 dark:bg-gray-700 dark:peer-focus:ring-resin-800"
    ></div>
    <span v-if="label" class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-300">
      {{ label }}
    </span>
  </label>
</template>

<script setup lang="ts">
defineProps({
  modelValue: {
    type: Boolean,
    required: true
  },
  label: {
    type: String,
    default: ''
  }
});

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>();

function handleChange(event: Event) {
  const target = event.target as HTMLInputElement;
  emit('update:modelValue', target.checked);
}
</script> 