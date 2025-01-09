<template>
   <div class="flex h-screen flex-col items-center justify-center bg-resin-500">
      <IntroLogo class="z-0 scale-50" />
      <div
         class="fixed top-1/2 z-10 h-1/2 w-full translate-y-32 bg-resin-500 p-10"
      />
   </div>
</template>
<script>
export default {
   beforeRouteEnter(to, from, next) {
    const isAuthenticated = localStorage.getItem('nostr-store') ? JSON.parse(localStorage.getItem('nostr-store')).authenticated : false;
    console.log('isAuthenticated', isAuthenticated);
    if (!isAuthenticated) {
      next(); // Redirect to the intro logo if not authenticated
    } else {
      next('/home'); // Proceed to the route
    }
  }
}
</script>
<script setup>
definePageMeta({
   layout: "startup",
});
</script>
