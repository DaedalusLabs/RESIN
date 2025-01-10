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
      <div v-if="hasData" id="pie-chart" class="py-6"></div>
      <div v-else class="flex items-center justify-center py-6 text-gray-400">
         No equity data available
      </div>
   </div>
</template>

<script setup>
import { PhInfo, PhXCircle } from "@phosphor-icons/vue";
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

const getChartOptions = () => {
   return {
      series: [props.equity, props.payOffAmount],
      colors: ["#F97316", "#6C6D6E"],
      chart: {
         height: 350,
         type: "pie",
      },
      labels: ["Equity", "Payoff"],
      stroke: {
         width: 0,
      },
      plotOptions: {
         pie: {
            dataLabels: {
               offset: -40,
               minAngleToShowLabel: 10,
            },
            donut: {
               size: "0%", // Ensures no donut hole for a solid pie chart
            },
         },
      },
      dataLabels: {
         enabled: true,
         textAnchor: "middle",
         style: {
            fontFamily: "Inter, sans-serif",
            fontWeight: "400",
            fontSize: "16px",
            colors: ["#FFFF"], // Text color on the pie chart
         },
         formatter: function (val, opts) {
            return (
               opts.w.config.labels[opts.seriesIndex] +
               " " +
               val.toFixed(2) +
               "%"
            );
         },
         dropShadow: {
            enabled: false,
         },
      },
      legend: {
         position: "bottom",
         fontSize: "12px",
         fontFamily: "Inter, sans-serif",
         labels: {
            colors: "#6B7280",
         },
         markers: {
            width: 8,
            height: 8,
            radius: 12,
         },
      },
      responsive: [
         {
            breakpoint: 480,
            options: {
               chart: {
                  height: 300,
               },
            },
         },
      ],
   };
};

onMounted(async () => {
   if (import.meta.client && hasData.value) {
      const ApexCharts = (await import("apexcharts")).default;
      const chart = new ApexCharts(
         document.getElementById("pie-chart"),
         getChartOptions(),
      );
      chart.render();
   }
});
</script>
