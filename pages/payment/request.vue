<template>
    <div class="max-w-2xl mx-auto p-4 py-20">
        <ResinAlert
            :show="showError"
            type="danger"
            :text="errorMessage"
        />
        <!-- Header -->
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-2xl font-bold">Payment Summary</h1>
            <img src="/images/logos/Resin_Hexagon_Orange_Fill.svg" alt="Resin" class="h-8" />
        </div>

        <!-- Payment Summary Card -->
        <div class="bg-white rounded-lg shadow-md p-6 mb-6">
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
                <div class="flex justify-between text-lg font-bold mt-4 pt-4 border-t">
                    <span>Total Due</span>
                    <span>${{ payment.amount.toFixed(2) }}</span>
                </div>
            </div>
        </div>

        <!-- Payment Methods -->
        <div class="space-y-4">
            <h2 class="text-xl font-semibold mb-4">Select Payment Method</h2>

            <!-- Bitcoin -->
            <button @click="selectPaymentMethod('bitcoin')"
                class="w-full p-4 bg-[#F7931A] text-white rounded-lg flex items-center justify-between hover:bg-[#E88A18] transition-colors">
                <span class="font-medium">Pay with Bitcoin</span>
                <img src="/icons/bitcoin-cryptocurrency.svg" alt="Bitcoin" class="w-6 h-6" />
            </button>

            <!-- USDT -->
            <button @click="selectPaymentMethod('usdt')"
                class="w-full p-4 bg-[#26A17B] text-white rounded-lg flex items-center justify-between hover:bg-[#219069] transition-colors">
                <span class="font-medium">Pay with USD Tether</span>
                <img src="/icons/tether-cryptocurrency.svg" alt="Tether" class="w-6 h-6" />
            </button>

            <!-- Bank Transfer -->
            <button @click="selectPaymentMethod('bank')"
                class="w-full p-4 bg-gray-900 text-white rounded-lg flex items-center justify-between hover:bg-gray-800 transition-colors">
                <span class="font-medium">Bank Transfer (+1%)</span>
                <PhBank class="w-6 h-6" />
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
    recipient: 'Resin Estate',
    propertyName: '123 Main Street',
    period: 'January 2024',
    amount: 1
});

const showError = ref(false);
const errorMessage = ref('');

const selectPaymentMethod = (method: 'bitcoin' | 'usdt' | 'bank') => {
    if (method === 'bank') {
        router.push({
            path: '/payment/process',
            query: {
                amount: payment.amount,
                currency: 'USD',
                method: method,
            }
        })
    } else {
        fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/payment/create`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                amount: payment.amount,
                currency: 'USD',
                method: method,
                userPubkey: useNostrStore().pubkey,
            })
        })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.json();
            })
            .then(data => {
                router.push({
                    path: `/payment/process`,
                    query: {
                        method,
                        id: data.id
                    }
                });
            })
            .catch(error => {
                console.error('Error creating payment:', error);
                errorMessage.value = 'Failed to create payment. Please try again.';
                showError.value = true;
                setTimeout(() => {
                    showError.value = false;
                }, 5000);
            });
    }
};

definePageMeta({
    layout: "white",
    middleware: ['auth']

});
</script>