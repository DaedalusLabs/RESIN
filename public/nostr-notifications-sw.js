importScripts("/ndk-bundle.min.js");

const ndk = new self.NDK({
   explicitRelayUrls: [
      "wss://nostr1.daedaluslabs.io",
      "wss://nostr2.daedaluslabs.io",
   ],
});

self.addEventListener("install", (event) => {
   self.skipWaiting();
});

self.addEventListener("activate", (event) => {
   event.waitUntil(clients.claim());
});

// Handle messages from the main thread
self.addEventListener("message", async (event) => {
   if (event.data.type === "INIT_NOSTR_SUB" && event.data.pubkey) {
      await initNostrSubscription(event.data.pubkey);
   }
});

async function initNostrSubscription(pubkey) {
   try {
      await ndk.connect();
      // Subscribe to kind 1059 events for the user's pubkey
      const sub = ndk.subscribe(
         {
            "kinds": [1059],
            "#p": [pubkey],
            "limit": 1,
         },
         { closeOnEose: false },
      );

      sub.on("event", async (event) => {
         // Get all window clients
         const clients = await self.clients.matchAll({
            type: "window",
            includeUncontrolled: true,
         });

         // If there's at least one window client active, send the event there
         if (clients.length > 0) {
            // Serialize the event data
            const serializedEvent = {
               id: event.id,
               pubkey: event.pubkey,
               kind: event.kind,
               tags: event.tags,
               content: event.content,
               created_at: event.created_at,
               sig: event.sig,
            };

            clients.forEach((client) => {
               client.postMessage({
                  type: "NOSTR_EVENT",
                  event: serializedEvent,
               });
            });
         } else {
            // If no window is active, show a notification
            const title = "RESIN - New Message";
            const options = {
               body: "You have a new encrypted message",
               icon: "/images/logos/Resin_Hexagon_Orange_Fill.svg",
               badge: "/images/logos/Resin_Hexagon_Orange_Fill.svg",
               tag: "resin-chat",
               data: { eventId: event.id },
            };

            await self.registration.showNotification(title, options);
         }
      });
   } catch (error) {
      console.error("Failed to initialize Nostr subscription:", error);
   }
}

// Handle notification clicks
self.addEventListener("notificationclick", (event) => {
   event.notification.close();

   event.waitUntil(
      clients.matchAll({ type: "window" }).then((clientList) => {
         // If there's already a window open, focus it
         for (const client of clientList) {
            if (client.url.includes("/messages") && "focus" in client) {
               return client.focus();
            }
         }
         // If no window is open, open a new one
         if (clients.openWindow) {
            return clients.openWindow("/chat");
         }
      }),
   );
});
