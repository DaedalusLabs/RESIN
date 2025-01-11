<template>
  <Transition
    enter-active-class="transition duration-300 ease-out"
    enter-from-class="transform translate-y-2 opacity-0"
    enter-to-class="transform translate-y-0 opacity-100"
    leave-active-class="transition duration-200 ease-in"
    leave-from-class="transform translate-y-0 opacity-100"
    leave-to-class="transform translate-y-2 opacity-0"
  >
    <div
      v-if="show"
      class="fixed left-1/2 z-50 w-[75%] -translate-x-1/2 transform sm:w-auto"
      :class="positionClass"
    >
      <div
        class="flex w-full items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium sm:w-auto"
        :class="alertClasses"
        role="alert"
      >
        <component :is="icon" :size="20" weight="fill" class="shrink-0" />
        <span>{{ text }}</span>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { PhCheckCircle, PhInfo, PhWarning, PhX } from "@phosphor-icons/vue";

type AlertType = 'success' | 'info' | 'warning' | 'danger';
type AlertPosition = 'top' | 'bottom';

const props = defineProps({
  show: {
    type: Boolean,
    required: true,
  },
  text: {
    type: String,
    required: true,
  },
  type: {
    type: String as () => AlertType,
    default: 'success'
  },
  position: {
    type: String as () => AlertPosition,
    default: 'bottom'
  }
});

const positionClass = computed(() => {
  return props.position === 'top' ? 'top-20' : 'bottom-20';
});

const alertClasses = computed(() => {
  switch (props.type) {
    case 'info':
      return 'bg-blue-100 text-blue-800';
    case 'warning':
      return 'bg-yellow-100 text-yellow-800';
    case 'danger':
      return 'bg-red-100 text-red-800';
    case 'success':
    default:
      return 'bg-green-100 text-green-800';
  }
});

const icon = computed(() => {
  switch (props.type) {
    case 'info':
      return PhInfo;
    case 'warning':
      return PhWarning;
    case 'danger':
      return PhX;
    case 'success':
    default:
      return PhCheckCircle;
  }
});
</script> 