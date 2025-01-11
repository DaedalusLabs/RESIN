export default defineNuxtRouteMiddleware(async (to) => {
   const store = useNostrStore();
   const localePath = useLocalePath();

   if (import.meta.server) return;
   await store.checkAuthenticated();

   if (!store.authenticated && to.path !== "/login") {
      return navigateTo(localePath("introduction"), { replace: true });
   }
});
