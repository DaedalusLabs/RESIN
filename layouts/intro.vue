<template>
   <div
      class="flex h-screen flex-col bg-pirate-950 bg-hex-dark bg-cover bg-center"
   >
      <main class="h-full flex-1 lg:grid lg:grid-cols-2 lg:grid-rows-1">
         <NuxtImg
            class="hidden h-full w-full object-cover lg:block"
            src="/images/introduction-desktop.png"
            alt="background"
         />
         <div class="h-full">
            <slot />
         </div>
      </main>
   </div>
</template>
