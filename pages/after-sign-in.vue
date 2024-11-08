<template>
   <section
      class="mx-auto flex w-10/12 flex-col items-center justify-center text-pirate-950"
   >
      <div class="mb-8 mt-10">
         <div
            class="inline-block rounded-full bg-resin-500 px-4 py-1 text-sm text-white"
         >
            Thank you
         </div>

         <h1 class="mt-4 text-3xl font-bold">
            Your agreement is being reviewed
         </h1>

         <p class="mt-4 text-base">
            You will be notified within 2 working days about your contract
            approval.
         </p>
      </div>

      <!-- Property Image -->
      <div class="relative mb-8 overflow-hidden rounded-2xl">
         <img
            src="/images/mock/house.png"
            alt="Thumbnail of your new property"
            class="h-[300px] w-full object-cover"
         />
      </div>

      <!-- Next Steps Card -->
      <div
         class="mx-auto w-11/12 -translate-y-24 rounded-xl bg-white p-6 shadow-lg"
      >
         <h2 class="mb-6 text-xl font-semibold text-resin-500">Next steps</h2>

         <ul class="space-y-4">
            <li class="flex items-center gap-3">
               <div class="text-resin-500">
                  <PhArrowRight :size="20" weight="bold" />
               </div>
               <p class="text-gray-500">Contract approval</p>
            </li>

            <li class="flex items-center gap-3">
               <PhCheckCircle :size="20" weight="fill" class="text-gray-400" />
               <p class="text-gray-500">First payment</p>
            </li>

            <li class="flex items-center gap-3">
               <PhCheckCircle :size="20" weight="fill" class="text-gray-400" />
               <p class="text-gray-500">Get your keys!</p>
            </li>
         </ul>
      </div>
   </section>
</template>

<script setup>
import { PhArrowRight, PhCheckCircle } from "@phosphor-icons/vue";
definePageMeta({
   layout: "white",
});
</script>
