<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <h1 class="mb-10 text-2xl font-extrabold leading-tight">My Resin</h1>

      <div class="space-y-10">
         <section class="flex flex-col gap-4">
            <MyResinNavbar title="Properties" link="my-properties" />
            <!-- TODO: Add properties -->
         </section>
         <section>
            <MyResinNavbar title="Transactions" link="my-transactions" />
            <!-- TODO: Add transactions -->
         </section>
         <section>
            <MyResinNavbar title="Agreements" link="my-agreements" />
            <!-- TODO: Add agreements -->
         </section>
         <section>
            <MyResinNavbar title="Financials" link="my-financials" />
            <!-- TODO: Add finances -->
         </section>
      </div>
   </section>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";

definePageMeta({
   layout: "white",
});
</script>
