<template>
   <section></section>
</template>

<script setup lang="ts">
const isScriptLoaded = ref(false);
const invoicePaid = ref(false);

// Declare the BTCPay window type
declare global {
   interface Window {
      btcpay: {
         showInvoice: (invoiceId: string) => void;
         onModalReceiveMessage: (callback: (event: unknown) => void) => void;
         hideFrame: () => void;
      };
   }
}

useHead({
   script: [
      {
         src: "https://btcpay.resin.estate/modal/btcpay.js",
         defer: true,
      },
   ],
});

const emit = defineEmits(["invoice_paid", "invoice_expired", "close"]);

onMounted(() => {
   const checkBTCPay = setInterval(() => {
      if (window.btcpay) {
         console.log("BTCPay is loaded");
         isScriptLoaded.value = true;
         registerEventListeners();
         clearInterval(checkBTCPay);
         showInvoice("9r6xUjiZuzEfr5fFk9kf5W");
      }
   }, 100);
});

const registerEventListeners = () => {
   window.btcpay.onModalReceiveMessage((event: unknown) => {
      console.log("onModalReceiveMessage", event);
      if (typeof event === "object" && event !== null && "data" in event) {
         const eventData = event.data as { status: string };
         if (eventData.status) {
            switch (event.data.status) {
               case "complete":
               case "paid":
                  invoicePaid.value = true;
                  emit("invoice_paid");
                  break;
               case "expired":
                  window.btcpay.hideFrame();
                  emit("invoice_expired");
                  break;
            }
         }
      } else if (event.data === "close") {
         emit("close");
      }
   });
};

// Function to show the invoice modal
const showInvoice = (invoiceId: string) => {
   if (window.btcpay) {
      window.btcpay.showInvoice(invoiceId);
   }
};
</script>
