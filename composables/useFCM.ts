import { initializeApp } from 'firebase/app';
import { getMessaging, getToken, onMessage, type Messaging } from 'firebase/messaging';
import { useNostrStore } from '~/stores/nostr';

let messaging: Messaging | null = null;

export const useFCM = () => {
    const runtimeConfig = useRuntimeConfig();
    const nostrStore = useNostrStore();

    const isIOSDevice = () => {
        return import.meta.client && /iPad|iPhone|iPod/.test(navigator.userAgent);
    };

    const initializeFCM = () => {
        // Only initialize FCM if we're on iOS
        if (!isIOSDevice()) {
            return false;
        }

        if (import.meta.client) {
            try {
                const firebaseConfig = {
                    apiKey: runtimeConfig.public.FIREBASE_API_KEY,
                    authDomain: runtimeConfig.public.FIREBASE_AUTH_DOMAIN,
                    projectId: runtimeConfig.public.FIREBASE_PROJECT_ID,
                    storageBucket: runtimeConfig.public.FIREBASE_STORAGE_BUCKET,
                    messagingSenderId: runtimeConfig.public.FIREBASE_MESSAGING_SENDER_ID,
                    appId: runtimeConfig.public.FIREBASE_APP_ID,
                };
                console.log('firebaseConfig', firebaseConfig);
                const app = initializeApp(firebaseConfig);
                messaging = getMessaging(app);

                // Handle incoming messages when app is in foreground
                onMessage(messaging, (payload) => {
                    console.log('Received foreground message:', payload);
                    const { title, body } = payload.notification || {};
                    if (title && body) {
                        nostrStore.showNotification(title, body);
                    }
                });

                return true;
            } catch (error) {
                console.error('Error initializing Firebase:', error);
                return false;
            }
        }
        return false;
    };

    const requestPermission = async () => {
        // Only proceed with FCM for iOS devices
        if (!isIOSDevice()) {
            return null;
        }

        try {
            if (!messaging) {
                const initialized = initializeFCM();
                if (!initialized) return null;
            }

            const permission = await Notification.requestPermission();
            if (permission === 'granted') {
                // Get FCM token
                const token = await getToken(messaging!, {
                    vapidKey: runtimeConfig.public.FIREBASE_VAPID_KEY
                });
                
                // Save the token to the nostr store
                await nostrStore.savePushToken(token);
                
                return token;
            }
            
            return null;
        } catch (error) {
            console.error('Error requesting permission:', error);
            return null;
        }
    };

    const getFCMToken = async () => {
        if (!isIOSDevice()) {
            return null;
        }

        if (!messaging) {
            const initialized = initializeFCM();
            if (!initialized) return null;
        }

        try {
            return await getToken(messaging!, {
                vapidKey: runtimeConfig.public.FIREBASE_VAPID_KEY
            });
        } catch (error) {
            console.error('Error getting FCM token:', error);
            return null;
        }
    };

    return {
        initializeFCM,
        requestPermission,
        getFCMToken,
        isIOSDevice,
    };
}; 