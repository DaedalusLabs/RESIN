<template>
   <div
      class="flex h-screen flex-col bg-pirate-950 bg-hex-dark bg-cover bg-center"
   >
      <main class="flex h-full flex-1">
         <NuxtImg
            class="hidden h-full w-3/5 object-cover lg:block"
            src="/images/introduction-desktop.png"
            alt="background"
         />
         <div class="h-full flex-1">
            <slot />
         </div>
      </main>
   </div>
</template>
