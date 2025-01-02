<template>
  <button
    :type="type"
    :disabled="disabled"
    :class="[
      'inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-medium transition-colors duration-200 focus:outline-none',
      {
        // Variants
        'bg-resin-500 text-white hover:bg-resin-600': variant === 'primary' && !disabled,
        'border border-resin-500 bg-transparent text-resin-500 hover:bg-resin-500 hover:text-white': variant === 'outline' && !disabled,
        'border border-pirate-400 text-white hover:bg-white hover:text-resin-500': variant === 'secondary' && !disabled,
        'bg-white bg-opacity-25 text-pirate-300 hover:cursor-not-allowed': disabled,
        
        // Sizes
        'px-3 py-1.5 text-xs': size === 'sm',
        'px-4 py-2 text-sm': size === 'md',
        'px-6 py-3 text-base': size === 'lg',
        
        // Full width
        'w-full': fullWidth,
        
        // Custom classes
        [className || '']: Boolean(className)
      }
    ]"
    @click="!disabled && $emit('click')"
  >
    <!-- Prepend Icon -->
    <span v-if="$slots['icon-prepend'] || prependIcon" class="mr-2">
      <slot name="icon-prepend">
        <component :is="prependIcon" :size="iconSize" />
      </slot>
    </span>

    <!-- Text -->
    <slot>{{ text }}</slot>

    <!-- Append Icon -->
    <span v-if="$slots['icon-append'] || appendIcon" class="ml-2">
      <slot name="icon-append">
        <component :is="appendIcon" :size="iconSize" />
      </slot>
    </span>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { ButtonProps } from './types';

const props = withDefaults(defineProps<ButtonProps>(), {
  type: 'button',
  variant: 'primary',
  size: 'md',
  disabled: false,
  fullWidth: false
});

const iconSize = computed(() => {
  switch (props.size) {
    case 'sm': return 16;
    case 'lg': return 24;
    default: return 20;
  }
});

defineEmits<{
  (e: 'click'): void
}>();
</script> 