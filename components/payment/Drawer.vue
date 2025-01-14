<template>
   <FlowbiteDrawer :is-open="showDrawer" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-pirate-950">Payment</h3>
      </template>

      <template #content>
         <PaymentBTCPay
            invoice="9r6xUjiZuzEfr5fFk9kf5W"
            @close="handleClose"
            @invoice_paid="handleInvoicePaid"
            @invoice_expired="handleInvoiceExpired"
         ></PaymentBTCPay>
      </template>
   </FlowbiteDrawer>
</template>

<script setup lang="ts">
const showDrawer = ref(true);

const emit = defineEmits(["close", "next"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

onMounted(() => {});

const handleClose = () => {
   console.log("handleClose");
};

const handleInvoiceExpired = () => {
   console.log("handleInvoiceExpired");
};

const handleInvoicePaid = () => {
   console.log("handleInvoicePaid");
};

watchEffect(() => {
   showDrawer.value = props.show;
});

const handleCloseDrawer = () => {
   emit("close");
};
</script>
