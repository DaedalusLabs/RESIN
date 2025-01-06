// composables/useNostr.ts
import { NDKNip07Signer, NDKPrivateKeySigner } from '@nostr-dev-kit/ndk';
import { ref } from 'vue';
import { useNostrStore } from '~/stores/nostr';

export function useNostr() {
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const store = useNostrStore();

  const loginWithExtension = async () => {
    isLoading.value = true;
    error.value = null;
    
    try {
      const pubkey = await store.login();
      setSigner(new NDKNip07Signer());
      store.pubkey = pubkey;

      return pubkey;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to login';
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const hasExtension = () => 'nostr' in window

  const hasNip44 = () => 'nostr' in window && 'nip44' in window.nostr

  const generateKeyPair = () => {
 //   authType = 'generated'
    return store.generateKeyPair()
  }

  const loginWithMnemonic = async (mnemonic: string) => {
//    authType = 'generated'
    const { privateKey, _, pubkey } = await store.generateKeyPair(mnemonic);
    setSigner(new NDKPrivateKeySigner(privateKey));
    store.pubkey = pubkey;
    return privateKey;
  }

  const checkAuthenticated = async() => {
    return await store.checkAuthenticated();
  }



  return {
    isLoading,
    error,
    loginWithExtension,
    loginWithMnemonic,
    hasExtension,
    hasNip44,
    generateKeyPair,
    checkAuthenticated,
    isAuthenticated: computed(() => store.authenticated),
    pubkey: computed(() => store.pubkey)
  };
}