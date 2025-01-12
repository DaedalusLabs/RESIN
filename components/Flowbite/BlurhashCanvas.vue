<template>
   <canvas 
      ref="canvas" 
      :width="width" 
      :height="height" 
      :class="$attrs.class" 
      :style="{ width: '100%', height: '100%' }" 
   />
</template>

<script setup>
import { decode } from 'blurhash';

const props = defineProps({
   hash: {
      type: String,
      required: true
   },
   width: {
      type: Number,
      default: 32 // Small default size for performance
   },
   height: {
      type: Number,
      default: 32
   },
   punch: {
      type: Number,
      default: 1
   }
});

const canvas = ref(null);

const renderBlurhash = () => {
   if (!canvas.value) return;

   const pixels = decode(props.hash, props.width, props.height, props.punch);
   const ctx = canvas.value.getContext('2d');
   const imageData = ctx.createImageData(props.width, props.height);
   imageData.data.set(pixels);
   ctx.putImageData(imageData, 0, 0);
};

onMounted(() => {
   renderBlurhash();
});

// Update canvas when hash changes
watch(() => props.hash, () => {
   renderBlurhash();
});
</script> 