<template>
   <canvas
      ref="canvas"
      :width="width"
      :height="height"
      :class="[$attrs.class, 'transition-opacity duration-200']"
      :style="{
         width: '100%',
         height: '100%',
         visibility: isVisible ? 'visible' : 'hidden',
         opacity: shouldFadeOut ? '0' : '1',
      }"
   />
</template>

<script setup>
import { decode } from "blurhash";

const props = defineProps({
   hash: {
      type: String,
      required: true,
   },
   width: {
      type: Number,
      default: 32,
   },
   height: {
      type: Number,
      default: 32,
   },
   punch: {
      type: Number,
      default: 1,
   },
   priority: {
      type: Boolean,
      default: false,
   },
});

const emit = defineEmits(["blurhash-ready"]);

const canvas = ref(null);
const isVisible = ref(false);
const shouldFadeOut = ref(false);
const pixelCache = new Map();

const decodeHash = (hash) => {
   if (pixelCache.has(hash)) {
      return pixelCache.get(hash);
   }
   const pixels = decode(hash, props.width, props.height, props.punch);
   pixelCache.set(hash, pixels);
   return pixels;
};

const renderBlurhash = () => {
   if (!canvas.value || !isVisible.value) return;

   requestAnimationFrame(() => {
      try {
         const pixels = decodeHash(props.hash);
         const ctx = canvas.value.getContext("2d");
         const imageData = ctx.createImageData(props.width, props.height);
         imageData.data.set(pixels);
         ctx.putImageData(imageData, 0, 0);
         emit("blurhash-ready");
      } catch (error) {
         console.warn("Failed to render blurhash:", error);
      }
   });
};

// Method to trigger the fade out
const fadeOut = () => {
   shouldFadeOut.value = true;
};

// Expose the fadeOut method to parent components
defineExpose({ fadeOut });

onMounted(() => {
   if (props.priority) {
      isVisible.value = true;
      renderBlurhash();
      return;
   }

   const observer = new IntersectionObserver(
      (entries) => {
         entries.forEach((entry) => {
            if (entry.isIntersecting) {
               isVisible.value = true;
               renderBlurhash();
               observer.disconnect();
            }
         });
      },
      {
         rootMargin: "50px",
      },
   );

   if (canvas.value) {
      observer.observe(canvas.value);
   }

   onUnmounted(() => {
      observer.disconnect();
   });
});

// Update canvas when hash changes and component is visible
watch(
   () => props.hash,
   () => {
      if (isVisible.value) {
         shouldFadeOut.value = false;
         renderBlurhash();
      }
   },
);

// Cleanup cache when component is unmounted
onUnmounted(() => {
   pixelCache.clear();
});
</script>
