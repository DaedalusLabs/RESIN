<template>
   <div class="flex h-screen flex-col items-center justify-center bg-resin-500">
      <IntroLogo class="z-0 scale-50" />
      <div
         class="fixed top-1/2 z-10 h-1/2 w-full translate-y-32 bg-resin-500 p-10"
      />
   </div>
</template>
<script setup>
definePageMeta({
   layout: "startup",
   middleware: [
      async function (to, from, next) {
         const store = useNostrStore();

         const isAuthenticated = await store.checkAuthenticated();

         if (isAuthenticated) {
            return navigateTo('/home');
         } else {
            return navigateTo('/introduction');
         }
         console.log('to', to);
         console.log('from', from);
         next();
      }
   ]
});
</script>
