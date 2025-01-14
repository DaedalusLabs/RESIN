<template>
   <div class="mx-auto max-w-2xl p-4 py-20">
      <ResinAlert :show="showError" type="danger" :text="errorMessage" />
      <!-- Header -->
      <div class="mb-6 flex items-center justify-between">
         <h1 class="text-2xl font-bold">Payment Summary</h1>
         <img
            src="/images/logos/Resin_Hexagon_Orange_Fill.svg"
            alt="Resin"
            class="h-8"
         />
      </div>

      <!-- Payment Summary Card -->
      <div class="mb-6 rounded-lg bg-white p-6 shadow-md">
         <div class="space-y-4">
            <div class="flex justify-between">
               <span class="text-gray-600">Payment to</span>
               <span class="font-medium">{{ payment.recipient }}</span>
            </div>
            <div class="flex justify-between">
               <span class="text-gray-600">Property</span>
               <span class="font-medium">{{ payment.propertyName }}</span>
            </div>
            <div class="flex justify-between">
               <span class="text-gray-600">Period</span>
               <span class="font-medium">{{ payment.period }}</span>
            </div>
            <div
               class="mt-4 flex justify-between border-t pt-4 text-lg font-bold"
            >
               <span>Total Due</span>
               <span>${{ payment.amount.toFixed(2) }}</span>
            </div>
         </div>
      </div>

      <!-- Payment Methods -->
      <div class="space-y-4">
         <h2 class="mb-4 text-xl font-semibold">Select Payment Method</h2>

         <!-- Bitcoin -->
         <button
            class="flex w-full items-center justify-between rounded-lg bg-[#F7931A] p-4 text-white transition-colors hover:bg-[#E88A18]"
            @click="selectPaymentMethod('bitcoin')"
         >
            <span class="font-medium">Pay with Bitcoin</span>
            <img
               src="/icons/bitcoin-cryptocurrency.svg"
               alt="Bitcoin"
               class="h-6 w-6"
            />
         </button>

         <!-- USDT -->
         <button
            class="flex w-full items-center justify-between rounded-lg bg-[#26A17B] p-4 text-white transition-colors hover:bg-[#219069]"
            @click="selectPaymentMethod('usdt')"
         >
            <span class="font-medium">Pay with USD Tether</span>
            <img
               src="/icons/tether-cryptocurrency.svg"
               alt="Tether"
               class="h-6 w-6"
            />
         </button>

         <!-- Bank Transfer -->
         <button
            class="flex w-full items-center justify-between rounded-lg bg-gray-900 p-4 text-white transition-colors hover:bg-gray-800"
            @click="selectPaymentMethod('bank')"
         >
            <span class="font-medium">Bank Transfer (+1%)</span>
            <PhBank class="h-6 w-6" />
         </button>
      </div>
   </div>
</template>

<script setup lang="ts">
import { PhBank } from "@phosphor-icons/vue";

const runtimeConfig = useRuntimeConfig();
const router = useRouter();

// Mock payment data - in real app this would come from API/store
const payment = reactive({
   recipient: "Resin Estate",
   propertyName: "123 Main Street",
   period: "January 2024",
   amount: 1,
});

const showError = ref(false);
const errorMessage = ref("");

const selectPaymentMethod = (method: "bitcoin" | "usdt" | "bank") => {
   if (method === "bank") {
      router.push({
         path: "/payment/process",
         query: {
            amount: payment.amount,
            currency: "USD",
            method: method,
         },
      });
   } else {
      fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/payment/create`, {
         method: "POST",
         headers: {
            "Content-Type": "application/json",
         },
         body: JSON.stringify({
            amount: payment.amount,
            currency: "USD",
            method: method,
            userPubkey: useNostrStore().pubkey,
         }),
      })
         .then((response) => {
            if (!response.ok) {
               throw new Error(`HTTP error! status: ${response.status}`);
            }
            return response.json();
         })
         .then((data) => {
            router.push({
               path: `/payment/process`,
               query: {
                  method,
                  id: data.id,
               },
            });
         })
         .catch((error) => {
            console.error("Error creating payment:", error);
            errorMessage.value = "Failed to create payment. Please try again.";
            showError.value = true;
            setTimeout(() => {
               showError.value = false;
            }, 5000);
         });
   }
};

definePageMeta({
   layout: "white",
   middleware: ["auth"],
});
</script>
