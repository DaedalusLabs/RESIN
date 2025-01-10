// This file bundles NDK for use in service workers
import NDK from '@nostr-dev-kit/ndk';

// Initialize WebSocket for the worker scope
if (!self.WebSocket) {
  self.WebSocket = WebSocket;
}

// Expose NDK to the global scope
self.NDK = NDK; 