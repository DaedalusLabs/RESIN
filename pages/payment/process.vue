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
                <div
class="bg-green-500 h-2 rounded-full transition-all duration-1000"
                    :style="{ width: `${timePercentage}%` }"></div>
            </div>
        </div>
        <hr class="my-6">
        <!-- Bitcoin Payment Section -->
        <template v-if="method === 'bitcoin'">
            <div class="mb-6">
                <div class="flex justify-center space-x-2 mb-6">
                    <button
v-for="network in ['on-chain', 'lightning', 'liquid']" :key="network"
                        :class="[
                            'px-4 py-2 rounded-lg font-medium',
                            selectedNetwork === network
                                ? 'bg-[#F7931A] text-white'
                                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                        ]" @click="selectedNetwork = network">
                        {{ network.charAt(0).toUpperCase() + network.slice(1) }}
                    </button>
                </div>

                <div class="text-center">
                    <a
:href="getQRData" target="_blank" rel="noopener noreferrer"
                        class="inline-block bg-white p-4 rounded-lg shadow-md mb-4 w-auto h-auto hover:shadow-lg transition-shadow">
                        <QRCode
:data="getQRData" :image="getQRImage" class="w-full h-full" :width="256"
                            :height="256" />
                    </a>

                    <div ref="btcAddressContainer" class="bg-gray-100 p-3 rounded-lg flex items-center justify-between mb-4">
                        <code class="text-sm font-mono">{{ truncateIfNeeded(getPaymentAddress || '', btcAddressContainer) }}</code>
                        <button
class="text-resin-500 hover:text-resin-600 ml-3 flex-shrink-0"
                            @click="copyToClipboard(getPaymentAddress || '')">
                            <PhCopy :size="20" />
                        </button>
                    </div>
                </div>
            </div>
        </template>

        <!-- USDT Payment Section -->
        <template v-if="method === 'usdt'">
            <div class="text-center mb-6">
                <a
:href="getQRData" target="_blank" rel="noopener noreferrer"
                    class="inline-block bg-white p-4 rounded-lg shadow-md mb-4 w-auto h-auto hover:shadow-lg transition-shadow">
                    <QRCode :data="getQRData" :image="getQRImage" class="w-full h-full" :width="256" :height="256" />
                </a>

                <div ref="usdtAddressContainer" class="bg-gray-100 p-3 rounded-lg flex items-center justify-between mb-4">
                    <code class="text-sm font-mono">{{ truncateIfNeeded(usdtAddress?.destination || '', usdtAddressContainer) }}</code>
                    <button
class="text-resin-500 hover:text-resin-600 ml-3 flex-shrink-0"
                        @click="copyToClipboard(usdtAddress?.destination || '')">
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
                                
                            </div>
                            <div class="bg-gray-100 p-3 rounded-lg flex items-center justify-between mb-4">
                                <code class="text-sm">{{ field.value }}</code>
                                <button
class="text-resin-500 hover:text-resin-600 ml-3 flex-shrink-0"
                                    @click="copyToClipboard(field.value)">
                                    <PhCopy :size="20" />
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div v-if="bankDetailsData.currency === 'EUR'" class="text-center">
                    <div class="inline-block bg-white p-4 rounded-lg shadow-md mb-4 w-auto h-auto">
                        <QRCode :data="getQRData" :image="getQRImage" class="w-full h-full" :width="256" :height="256" />
                    </div>
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
import type { BankDetails } from '~/types/BankDetails';

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

const bankDetailsData: BankDetails = {
    amount: amount.value,
    beneficiary: 'Resin Estate Ltd.',
    iban: 'NL91 ABNA 0417 1643 00',
    bicSwift: 'ABNANL2A',
    bankName: 'ABN AMRO Bank N.V.',
    description: `Payment for property ${route.query.id}`
};

const bankDetails = [
    { label: 'Amount', value: `$${bankDetailsData.amount.toFixed(2)}` },
    { label: 'Beneficiary', value: bankDetailsData.beneficiary },
    { label: 'IBAN', value: bankDetailsData.iban },
    { label: 'BIC/SWIFT', value: bankDetailsData.bicSwift },
    { label: 'Bank Name', value: bankDetailsData.bankName },
    { label: 'Description', value: bankDetailsData.description }
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
        
        return generateSepaQRString(convertToSepaQRData(bankDetailsData));
    }
    return '';
});

const convertToSepaQRData = (bankDetails: BankDetails) => {
    // Helper function to clean IBAN by removing spaces
    const cleanIBAN = (iban: string) => iban.replace(/\s/g, '');

    const sepaData = {
        serviceTag: 'BCD',                // Service Tag for SEPA Credit Transfer
        version: '002',                   // Version 2
        characterSet: '1',                // UTF-8
        identification: 'SCT',            // SEPA Credit Transfer
        bic: bankDetails.bicSwift,
        name: bankDetails.beneficiary,
        iban: cleanIBAN(bankDetails.iban),
        currency: 'EUR',                  // Euro is required for SEPA
        amount: bankDetails.amount.toFixed(2),
        purpose: '',                      // Can be left empty
        reference: '',                    // Can be left empty
        information: bankDetails.description
    };

    return sepaData;
}

const generateSepaQRString = (sepaData: {
    serviceTag: string;
    version: string;
    characterSet: string;
    identification: string;
    bic: string;
    name: string;
    iban: string;
    currency: string;
    amount: string;
    purpose: string;
    reference: string;
    information: string;
}): string => {
    return [
        sepaData.serviceTag,
        sepaData.version,
        sepaData.characterSet,
        sepaData.identification,
        sepaData.bic,
        sepaData.name,
        sepaData.iban,
        `${sepaData.currency.toUpperCase()}${sepaData.amount}`,
        sepaData.purpose,
        sepaData.reference,
        sepaData.information
    ].join('\n');
}

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

// Replace truncate function with dynamic version
const truncateIfNeeded = (str: string, containerRef: HTMLElement | null) => {
    if (!str || !containerRef) return str;

    // Create temporary span to measure text width
    const tempSpan = document.createElement('span');
    tempSpan.style.visibility = 'hidden';
    tempSpan.style.position = 'absolute';
    tempSpan.style.whiteSpace = 'nowrap';
    tempSpan.style.fontFamily = 'monospace'; // Match the font of our code element
    tempSpan.style.fontSize = '0.875rem'; // Match text-sm
    tempSpan.innerText = str;
    document.body.appendChild(tempSpan);

    const textWidth = tempSpan.offsetWidth;
    const containerWidth = containerRef.offsetWidth - 48; // Subtract padding and copy button width
    document.body.removeChild(tempSpan);

    if (textWidth <= containerWidth) {
        return str;
    }

    // If text doesn't fit, calculate how many characters we can show
    const charWidth = textWidth / str.length;
    const availableChars = Math.floor(containerWidth / charWidth);
    const charsPerSide = Math.floor((availableChars - 3) / 2); // -3 for the ...

    return `${str.slice(0, charsPerSide)}...${str.slice(-charsPerSide)}`;
};

// Add refs for containers
const btcAddressContainer = ref<HTMLElement | null>(null);
const usdtAddressContainer = ref<HTMLElement | null>(null);

definePageMeta({
    layout: "white",
    middleware: ['auth'],
    hideBottomBar: true
});
</script>