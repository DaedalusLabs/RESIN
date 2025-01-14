<template>
   <div
      role="status"
      class="flex h-56 w-full max-w-sm animate-pulse items-center justify-center rounded-lg bg-pirate-950 dark:bg-gray-950"
   >
      <div class="flex flex-col items-center justify-center">
         <NuxtImg
            src="images/logos/nostr.webp"
            class="h-12 w-12"
            alt="Loading"
         />
         <p class="ml-4 text-lg font-bold text-pirate-700">
            {{ $t("fetching") }}
         </p>
      </div>
   </div>
</template>
