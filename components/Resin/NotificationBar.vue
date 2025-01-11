<template>
  <Transition
    :enter-active-class="'transition duration-300 ease-out'"
    :enter-from-class="`transform ${position === 'top' ? '-translate-y-full' : 'translate-y-full'} opacity-0`"
    :enter-to-class="'transform translate-y-0 opacity-100'"
    :leave-active-class="'transition duration-200 ease-in'"
    :leave-from-class="'transform translate-y-0 opacity-100'"
    :leave-to-class="`transform ${position === 'top' ? '-translate-y-full' : 'translate-y-full'} opacity-0`"
  >
    <div
      v-if="show"
      class="fixed inset-x-0 z-50 px-4 py-3 shadow-lg text-white"
      :class="[
        position === 'top' ? 'top-0' : 'bottom-0',
        colorClass
      ]"
    >
      <div class="container mx-auto flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="animate-spin">
            <PhSpinner :size="24" weight="bold" />
          </div>
          <div>
            <p class="font-medium">{{ title }}</p>
            <p class="text-sm opacity-90">{{ description }}</p>
          </div>
        </div>
        <div v-if="countdown !== undefined" class="text-sm opacity-90">
          {{ countdown }}
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { PhSpinner } from "@phosphor-icons/vue";

const props =defineProps({
  show: {
    type: Boolean,
    required: true
  },
  position: {
    type: String,
    default: 'bottom',
    validator: (value) => ['top', 'bottom'].includes(value)
  },
  title: {
    type: String,
    required: true
  },
  description: {
    type: String,
    required: true
  },
  countdown: {
    type: String,
    default: undefined
  },
  color: {
    type: String,
    default: 'resin',
    validator: (value) => ['resin', 'blue'].includes(value)
  }
});

const colorClass = computed(() => ({
  'resin': 'bg-gradient-to-r from-resin-500 to-resin-600',
  'blue': 'bg-gradient-to-r from-blue-500 to-blue-600'
}[props.color]));
</script> 