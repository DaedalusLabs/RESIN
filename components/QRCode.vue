<template>
   <div ref="qrCodeRef" :class="containerClass"></div>
</template>

<script setup lang="ts">
import type QRCodeStyling from "qr-code-styling";

const nuxtApp = useNuxtApp();
const createQRCode = nuxtApp.$createQRCode as (options: {
   data: string;
   image?: string;
   width?: number;
   height?: number;
}) => QRCodeStyling;

const props = defineProps<{
   data: string;
   image?: string;
   width?: number;
   height?: number;
   containerClass?: string;
}>();

const qrCodeRef = ref<HTMLDivElement | null>(null);
const qrCode = ref<QRCodeStyling | null>(null);

const createAndAppendQRCode = () => {
   if (!qrCodeRef.value || !props.data) return;

   qrCode.value = createQRCode({
      data: props.data,
      image: props.image,
      width: props.width,
      height: props.height,
   });

   qrCodeRef.value.innerHTML = "";
   if (qrCode.value) {
      qrCode.value.append(qrCodeRef.value);
   }
};

// Watch for changes after function definition
watch(
   () => props.data,
   (newData) => {
      if (newData) {
         if (qrCode.value) {
            qrCode.value.update({
               data: newData,
            });
         } else {
            createAndAppendQRCode();
         }
      }
   },
   { immediate: true },
);

watch(
   () => props.image,
   (newImage) => {
      if (newImage && qrCode.value) {
         qrCode.value.update({
            image: newImage,
         });
      }
   },
);

onMounted(() => {
   if (props.data) {
      createAndAppendQRCode();
   }
});

onBeforeUnmount(() => {
   if (qrCode.value) {
      qrCode.value = null;
   }
});
</script>
