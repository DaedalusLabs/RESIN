<template>
   <div>
      <div v-show="show" class="fixed inset-0 z-40 overflow-hidden">
         <div
            class="absolute inset-0 bg-black bg-opacity-75 transition-opacity"
            @click="backdropClickHandler"
         />
      </div>
      <transition :name="transitionName">
         <div
            v-show="show"
            v-touch:swipe="swipeHandler"
            :class="[drawerClasses, props.slideFrom === 'side' ? 'h-dvh' : '']"
         >
            <div
               v-if="slideFrom === 'bottom'"
               class="mx-auto mb-4 mt-3 h-1 w-8 cursor-pointer rounded-none bg-gray-300"
               @click="close"
            />

            <div v-if="!props.hideTitle" class="mb-4 flex items-center px-4">
               <h3
                  id="drawer-title"
                  class="w-full text-center text-xl font-bold text-gray-900"
               >
                  <slot name="title" />
               </h3>
            </div>

            <div class="flex-1 overflow-y-auto">
               <slot name="content" />
            </div>
         </div>
      </transition>
   </div>
</template>

<script setup>
const show = ref(false);

const props = defineProps({
   isOpen: {
      type: Boolean,
      required: true,
   },
   slideFrom: {
      type: String,
      default: "bottom", // "bottom" or "side"
      validator: (value) => ["bottom", "side"].includes(value),
   },
   hideTitle: {
      type: Boolean,
      default: false,
   },
   closeOnBackdropClick: {
      type: Boolean,
      default: true,
   },
});

const emit = defineEmits(["close"]);

const close = () => {
   emit("close");
   console.log("close");
};

watchEffect(() => {
   show.value = props.isOpen;
});

const swipeHandler = (direction) => {
   if (
      (props.slideFrom === "bottom" && direction === "down") ||
      (props.slideFrom === "side" && direction === "left")
   ) {
      close();
   }
};

const backdropClickHandler = () => {
   if (props.closeOnBackdropClick) {
      close();
   }
};

const drawerClasses = computed(() => {
   return props.slideFrom === "bottom"
      ? "absolute inset-x-0 bottom-0 z-50 flex max-h-svh w-full flex-col rounded bg-white p-7 pt-0 shadow-lg"
      : "absolute top-0 left-0 z-50 flex h-full w-3/4 flex-col  bg-white p-7 shadow-lg";
});

const transitionName = computed(() => {
   return props.slideFrom === "bottom" ? "slide-up" : "slide-left";
});
</script>

<style scoped>
/* Fade in/out for the backdrop */
.fade-enter-active,
.fade-leave-active {
   transition: opacity 0.2s ease-out;
}
.fade-enter-from,
.fade-leave-to {
   opacity: 0;
}
.fade-enter-to,
.fade-leave-from {
   opacity: 1;
}

/* Slide up/down for the modal */
.slide-up-enter-active,
.slide-up-leave-active {
   transition:
      transform 0.2s ease-out,
      opacity 0.2s ease-out;
}
.slide-up-enter-from,
.slide-up-leave-to {
   transform: translateY(100%);
   opacity: 0;
}
.slide-up-enter-to,
.slide-up-leave-from {
   transform: translateY(0);
   opacity: 1;
}

/* Slide left/right for the modal */
.slide-left-enter-active,
.slide-left-leave-active {
   transition:
      transform 0.2s ease-out,
      opacity 0.2s ease-out;
}
.slide-left-enter-from,
.slide-left-leave-to {
   transform: translateX(-100%); /* From outside the screen on the left */
   opacity: 0;
}
.slide-left-enter-to,
.slide-left-leave-from {
   transform: translateX(0); /* Fully visible */
   opacity: 1;
}
</style>
