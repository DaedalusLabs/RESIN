<template>
   <div class="w-full rounded-lg bg-white p-4 shadow dark:bg-gray-800 md:p-6">
      <div class="mb-4 flex w-full items-start justify-between">
         <div class="flex-col items-center">
            <div class="mb-1 flex items-center text-gray-500 dark:text-white">
               <h5 class="mr-2 text-sm leading-none">My equity</h5>
               <PhInfo
                  aria-describedby="chart-info"
                  weight="fill"
                  :size="16"
                  class="cursor-pointer"
                  @click="showTooltip = !showTooltip"
               />
               <div
                  v-if="showTooltip"
                  id="chart-info"
                  class="absolute z-10 flex rounded-lg border border-gray-200 bg-white p-2 text-xs text-gray-500 shadow-lg dark:border-gray-700 dark:bg-gray-800"
                  @click="showTooltip = !showTooltip"
               >
                  <p class="text-sm">
                     Equity is the difference between the value of your assets
                     and liabilities.
                  </p>
                  <PhXCircle
                     class="text-gray-500 dark:text-gray-400"
                     size="20"
                  />
               </div>
            </div>
            <h2 class="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
               {{ formatCurrency(equity) }}
            </h2>
         </div>
      </div>
      <!-- Pie Chart -->
      <div v-if="hasData" class="py-6">
         <div class="h-[350px]">
            <Pie :data="chartData" :options="chartOptions" />
         </div>
      </div>
      <div v-else class="flex items-center justify-center py-6 text-gray-400">
         No equity data available
      </div>
   </div>
</template>

<script setup>
import { PhInfo, PhXCircle } from "@phosphor-icons/vue";
import { Pie } from "vue-chartjs";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";

ChartJS.register(ArcElement, Tooltip, Legend);

const { formatCurrency } = useFormatNumber();
const showTooltip = ref(false);

const props = defineProps({
   payOffAmount: {
      type: Number,
      required: true,
   },
   equity: {
      type: Number,
      required: true,
   },
});

const hasData = computed(() => props.equity > 0 || props.payOffAmount > 0);

const total = computed(() => props.equity + props.payOffAmount);

const chartData = computed(() => ({
   labels: ["Equity", "Payoff"],
   datasets: [
      {
         data: [props.equity, props.payOffAmount],
         backgroundColor: ["#F97316", "#6C6D6E"],
         borderWidth: 0,
      },
   ],
}));

const chartOptions = {
   responsive: true,
   maintainAspectRatio: false,
   plugins: {
      legend: {
         position: "bottom",
         labels: {
            font: {
               family: "Inter, sans-serif",
               size: 12,
            },
            color: "#6B7280",
            usePointStyle: true,
            pointStyle: "circle",
         },
      },
      tooltip: {
         callbacks: {
            label: function (context) {
               const label = context.label || "";
               const value = context.raw;
               const percentage = ((value / total.value) * 100).toFixed(2);
               return `${label}: ${percentage}%`;
            },
         },
      },
   },
};
</script>
