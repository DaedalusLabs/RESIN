import { defineStore } from "pinia";
import { useNostrStore } from "./nostr";

interface SettingsState {
  // Theme settings
  theme: 'light' | 'dark' | 'system';
  
  // Display preferences
  display: {
    language: string;
    currency: string;
  };
  // Map preferences
  map: {
    defaultZoom: number;
    defaultCenter: {
      lat: number;
      lng: number;
    };
    showLabels: boolean;
  };
  propertyTypes: string[];
}

export const useSettingsStore = defineStore("settings", {
  state: (): SettingsState => ({
    theme: 'system',
    
    display: {
      language: 'en',
      currency: 'USD'
    },
    
    map: {
      defaultZoom: 12,
      defaultCenter: {
        lat: 52.3676,
        lng: 4.9041,
      },
      showLabels: true,
    },
    propertyTypes: [],
  }),

  persist: {
    key: 'settings-store',
    storage: localStorage,
  },

  getters: {
    isDarkMode(): boolean {
      if (this.theme === 'system') {
        return window?.matchMedia('(prefers-color-scheme: dark)').matches;
      }
      return this.theme === 'dark';
    },
    notifications(): boolean {
      const nostrStore = useNostrStore();
      return nostrStore.notificationsEnabled;
    }
  },

  actions: {
    setTheme(theme: 'light' | 'dark' | 'system'): void {
      this.theme = theme;
    },

    setPropertyTypes(types: string[]): void {
      this.propertyTypes = types;
    },

    updateDisplaySettings(settings: Partial<SettingsState['display']>): void {
      this.display = {
        ...this.display,
        ...settings,
      };
    },

    updateMapSettings(settings: Partial<SettingsState['map']>): void {
      this.map = {
        ...this.map,
        ...settings,
      };
    },

    resetSettings(): void {
      this.$reset();
    },
  },
}); 