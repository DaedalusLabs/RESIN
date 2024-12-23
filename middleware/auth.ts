export default defineNuxtRouteMiddleware(async(to) => {

    const store = useNostrStore();

    if (import.meta.server) return
    await store.checkAuthenticated();
    
    if (!store.authenticated && to.path !== '/login') {
      return navigateTo('/en-US/introduction', { replace: true });
    }
  });