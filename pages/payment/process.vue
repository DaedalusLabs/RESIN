<template>
    <div class="max-w-2xl mx-auto p-4">
        <!-- Amount Display -->
        <div class="flex justify-between items-center mb-8">
            <div class="text-2xl font-bold">Amount Due</div>
            <div class="text-right">
                <div class="text-2xl font-bold text-black">${{ amount }}</div>
                <div v-if="method === 'bitcoin' && btcAddress?.destination" class="text-gray-600">≈ {{ btcAmount }} BTC
                </div>
            </div>
        </div>

        <!-- Countdown Timer -->
        <div v-if="method !== 'bank'" class="bg-green-50 border border-green-200 rounded-lg p-4 mb-6">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <PhTimer class="w-5 h-5 text-green-600" />
                    <span class="text-green-700">Time remaining</span>
                </div>
                <div class="text-xl font-mono text-green-700">{{ formattedTimeRemaining }}</div>
            </div>
            <div class="mt-2 bg-green-100 rounded-full h-2">
                <div class="bg-green-500 h-2 rounded-full transition-all duration-1000"
                    :style="{ width: `${timePercentage}%` }"></div>
            </div>
        </div>
        <hr class="my-6">
        <!-- Bitcoin Payment Section -->
        <template v-if="method === 'bitcoin'">
            <div class="mb-6">
                <div class="flex justify-center space-x-2 mb-6">
                    <button v-for="network in ['on-chain', 'lightning', 'liquid']" :key="network"
                        @click="selectedNetwork = network" :class="[
                            'px-4 py-2 rounded-lg font-medium',
                            selectedNetwork === network
                                ? 'bg-[#F7931A] text-white'
                                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                        ]">
                        {{ network.charAt(0).toUpperCase() + network.slice(1) }}
                    </button>
                </div>

                <div class="text-center">
                    <a :href="getQRData" target="_blank" rel="noopener noreferrer"
                        class="inline-block bg-white p-4 rounded-lg shadow-md mb-4 w-auto h-auto hover:shadow-lg transition-shadow">
                        <QRCode :data="getQRData" :image="getQRImage" class="w-full h-full" :width="256"
                            :height="256" />
                    </a>

                    <div class="bg-gray-100 p-3 rounded-lg flex items-center justify-between mb-4">
                        <code class="text-sm break-all">{{ getPaymentAddress }}</code>
                        <button @click="copyToClipboard(getPaymentAddress || '')"
                            class="text-resin-500 hover:text-resin-600">
                            <PhCopy :size="20" />
                        </button>
                    </div>
                </div>
            </div>
        </template>

        <!-- USDT Payment Section -->
        <template v-if="method === 'usdt'">
            <div class="text-center mb-6">
                <a :href="getQRData" target="_blank" rel="noopener noreferrer"
                    class="inline-block bg-white p-4 rounded-lg shadow-md mb-4 w-[256px] h-[256px] hover:shadow-lg transition-shadow">
                    <QRCode :data="getQRData" :image="getQRImage" class="w-full h-full" :width="256" :height="256" />
                </a>

                <div class="bg-gray-100 p-3 rounded-lg flex items-center justify-between mb-4">
                    <code class="text-sm break-all">{{ usdtAddress?.destination }}</code>
                    <button @click="copyToClipboard(usdtAddress?.destination || '')"
                        class="text-resin-500 hover:text-resin-600">
                        <PhCopy :size="20" />
                    </button>
                </div>
            </div>
        </template>

        <!-- Bank Transfer Section -->
        <template v-if="method === 'bank'">
            <div class="space-y-4 mb-6">
                <div class="bg-white rounded-lg shadow-md p-6">
                    <div class="space-y-4">
                        <div v-for="field in bankDetails" :key="field.label" class="flex flex-col">
                            <div class="flex justify-between items-center mb-1">
                                <span class="text-gray-600">{{ field.label }}</span>
                                <button @click="copyToClipboard(field.value)"
                                    class="text-resin-500 hover:text-resin-600">
                                    <PhCopy :size="20" />
                                </button>
                            </div>
                            <div class="bg-gray-100 p-3 rounded-lg">
                                <code class="text-sm">{{ field.value }}</code>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="text-center">
                    <a :href="getQRData" target="_blank" rel="noopener noreferrer"
                        class="inline-block bg-white p-4 rounded-lg shadow-md mb-4 w-64 h-64 hover:shadow-lg transition-shadow">
                        <QRCode :data="getQRData" :image="getQRImage" class="w-full h-full" :width="256"
                            :height="256" />
                    </a>
                </div>
            </div>
        </template>

        <!-- Alert -->
        <ResinAlert :show="showCopiedAlert" text="Copied" />
    </div>
</template>

<script setup lang="ts">
import { PhCopy, PhTimer } from "@phosphor-icons/vue";
import { onMounted, ref } from "vue";
import type { PaymentDetails } from "~/types/PaymentDetails";

const runtimeConfig = useRuntimeConfig();
const route = useRoute();
const router = useRouter();
const showCopiedAlert = ref(false);

const method = ref(route.query.method as string);
const amount = ref(Number(route.query.amount) || 0);
const selectedNetwork = ref('on-chain');
const statusCheckInterval = ref<ReturnType<typeof setInterval>>();

// Mock addresses and payment details
const btcAddress = ref<PaymentDetails>();
const lightningInvoice = ref<PaymentDetails>();
const liquidAddress = ref<PaymentDetails>();
const usdtAddress = ref<PaymentDetails>();
const expirationTime = ref(0);

const bankDetails = [
    { label: 'Amount', value: `$${amount.value.toFixed(2)}` },
    { label: 'Beneficiary', value: 'Resin Estate Ltd.' },
    { label: 'IBAN', value: 'NL91 ABNA 0417 1643 00' },
    { label: 'BIC/SWIFT', value: 'ABNANL2A' },
    { label: 'Bank Name', value: 'ABN AMRO Bank N.V.' },
    { label: 'Description', value: `Payment for property ${route.query.id}` }
];

// Replace mock BTC price calculation with network-specific amount
const btcAmount = computed(() => {
    if (method.value === 'bitcoin' && btcAddress.value) {
        switch (selectedNetwork.value) {
            case 'on-chain':
                return btcAddress.value?.amount?.toFixed(8) || '0.00000000';
            case 'lightning':
                return lightningInvoice.value?.amount?.toFixed(8) || '0.00000000';
            case 'liquid':
                return liquidAddress.value?.amount?.toFixed(8) || '0.00000000';
            default:
                return '0.00000000';
        }
    }
    return '0.00000000';
});

// Timer logic
const timeRemaining = ref(0);
const timer = ref<ReturnType<typeof setInterval>>();

const formattedTimeRemaining = computed(() => {
    const minutes = Math.floor(timeRemaining.value / 60);
    const seconds = timeRemaining.value % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
});

const timePercentage = computed(() => {
    const totalTime = 15 * 60; // 15 minutes in seconds
    return (timeRemaining.value / totalTime) * 100;
});

const updateTimeRemaining = () => {
    const now = Math.floor(Date.now() / 1000); // Current time in seconds
    if (expirationTime.value) {
        const remaining = expirationTime.value - now;
        timeRemaining.value = remaining > 0 ? remaining : 0;

        if (remaining <= 0) {
            clearInterval(timer.value);
            router.push('/payment/expired');
        }
    }
};

// Add status check function
const checkInvoiceStatus = () => {
    if (!route.query.id) return;
    
    fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/payment/invoice/${route.query.id}`)
        .then(res => res.json())
        .then(data => {
            if (data.status === 'Settled' || data.status === 'Complete') {
                clearInterval(statusCheckInterval.value);
                router.push('/payment/complete');
            } else if (data.status === 'expired') {
                clearInterval(statusCheckInterval.value);
                router.push('/payment/expired');
            }
        })
        .catch(err => console.error('Error checking invoice status:', err));
};

onMounted(() => {
    if (method.value !== 'bank') {
        if (method.value === 'bitcoin' || method.value === 'usdt') {
            fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/payment/details/${route.query.id}`)
                .then(res => res.json())
                .then(data => {
                    if (method.value === 'bitcoin') {
                        btcAddress.value = data.paymentMethods['BTC-CHAIN'];
                        lightningInvoice.value = data.paymentMethods['BTC-LN'];
                        liquidAddress.value = data.paymentMethods['LBTC-CHAIN'];
                    } else {
                        usdtAddress.value = data.paymentMethods['USDT-CHAIN'];
                    }
                    expirationTime.value = data.expirationTime;
                    amount.value = data.amount;

                    // Initialize timer after getting expiration time
                    updateTimeRemaining();
                    timer.value = setInterval(updateTimeRemaining, 1000);
                    checkInvoiceStatus();
                    // Start polling for invoice status
                    statusCheckInterval.value = setInterval(checkInvoiceStatus, 5000); // Check every 5 seconds
                })
                .catch(err => console.error(err));
        } else {
            router.push('/404');
        }
    } else if (method.value === 'bank') { /* empty */ } else {
        router.push('/404');
    }
});

onUnmounted(() => {
    if (timer.value) clearInterval(timer.value);
    if (statusCheckInterval.value) clearInterval(statusCheckInterval.value);
});

// Helper functions
const getPaymentAddress = computed(() => {
    switch (selectedNetwork.value) {
        case 'on-chain':
            return btcAddress.value?.destination;
        case 'lightning':
            return lightningInvoice.value?.destination;
        case 'liquid':
            return liquidAddress.value?.destination;
        case 'usdt':
            return usdtAddress.value?.destination;
        default:
            return '';
    }
});

const getQRData = computed(() => {
    if (method.value === 'bitcoin' && btcAddress.value) {
        switch (selectedNetwork.value) {
            case 'on-chain':
                return btcAddress.value.paymentLink;
            case 'lightning':
                return lightningInvoice.value?.paymentLink || '';
            case 'liquid':
                return liquidAddress.value?.paymentLink || '';
            case 'usdt':
                return usdtAddress.value?.paymentLink || '';
        }
    } else if (method.value === 'usdt' && usdtAddress.value) {
        return usdtAddress.value.paymentLink;
    } else if (method.value === 'bank') {
        const sepaData = {
            name: bankDetails[1].value,
            iban: bankDetails[2].value,
            amount: amount.value,
            reference: bankDetails[5].value
        };
        return JSON.stringify(sepaData);
    }
    return '';
});

const getQRImage = computed(() => {
    switch (method.value) {
        case 'bitcoin':
            return '/icons/bitcoin-cryptocurrency.svg';
        case 'usdt':
            return '/icons/tether-cryptocurrency.svg';
        default:
            return '/images/logos/Resin_Hexagon_Orange_Fill.svg';
    }
});

const copyToClipboard = async (text: string) => {
    try {
        await navigator.clipboard.writeText(text);
        showCopiedAlert.value = true;
        setTimeout(() => {
            showCopiedAlert.value = false;
        }, 2000);
    } catch (err) {
        console.error('Failed to copy text: ', err);
    }
};

definePageMeta({
    layout: "white",
    middleware: ['auth'],
    hideBottomBar: true
});
</script>