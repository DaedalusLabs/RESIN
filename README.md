<img src="./public/images/logos/resin-white.svg" alt="logo" width="300px" style="margin-bottom: 1rem">

Resin is an open-source real estate platform that helps users buy homes without relying on traditional bank loans. With a sleek, map-based interface, users can explore properties, view detailed listings, and discover alternative financing options — all in one place!

## 🚀 Technologies Used

This project is built using cutting-edge technologies to ensure scalability, maintainability, and ease of development:

### Core Framework

- **[Nuxt 3](https://nuxt.com/)** - The framework powering the entire app, built on Vue 3 and designed for both server-side and static site generation.
   - [GitHub Repository](https://github.com/nuxt/nuxt)
   - [Vue 3 Documentation](https://vuejs.org/)
   - [Vue 3 GitHub](https://github.com/vuejs/core)
- **[Pinia](https://pinia.vuejs.org/)** - Vue's official state management library.
   - [GitHub Repository](https://github.com/vuejs/pinia)
   - [Documentation](https://pinia.vuejs.org/)

### Nostr Integration

- **[NDK (Nostr Development Kit)](https://ndk.fiatjaf.com/)** - Powerful Nostr development toolkit.
   - [GitHub Repository](https://github.com/nostr-dev-kit/ndk)
   - [Documentation](https://ndk.fiatjaf.com/docs/getting-started)
- **[nostr-tools](https://github.com/nbd-wtf/nostr-tools)** - Core Nostr protocol implementation.
   - [GitHub Repository](https://github.com/nbd-wtf/nostr-tools)
   - [Documentation](https://github.com/nbd-wtf/nostr-tools#documentation)

### Search & Discovery

- **[Vue InstantSearch](https://www.algolia.com/doc/guides/building-search-ui/what-is-instantsearch/vue/)** - Search UI components for Vue.
   - [GitHub Repository](https://github.com/algolia/instantsearch.js)
   - [Documentation](https://www.algolia.com/doc/guides/building-search-ui/what-is-instantsearch/vue/)
- **[Typesense InstantSearch](https://github.com/typesense/typesense-instantsearch-adapter)** - Typesense adapter for InstantSearch.
   - [GitHub Repository](https://github.com/typesense/typesense-instantsearch-adapter)
   - [Documentation](https://typesense.org/docs/guide/introduction.html)

### Icons & UI Elements

- **[Phosphor Icons](https://phosphoricons.com/)** - Flexible icon family for interfaces.
   - [GitHub Repository](https://github.com/phosphor-icons/vue)
   - [Icon Browser](https://phosphoricons.com/)

### Nuxt Plugins & Modules

- **[@nuxtjs/i18n](https://i18n.nuxtjs.org/)** - Internationalization for Nuxt.
   - [GitHub Repository](https://github.com/nuxt-modules/i18n)
   - [Documentation](https://i18n.nuxtjs.org/)
- **[@nuxtjs/algolia](https://algolia.nuxtjs.org/)** - Algolia integration for Nuxt.
   - [GitHub Repository](https://github.com/nuxt-modules/algolia)
   - [Documentation](https://algolia.nuxtjs.org/)
- **[@nuxtjs/robots](https://github.com/nuxt-modules/robots)** - Robots.txt module for Nuxt.
   - [GitHub Repository](https://github.com/nuxt-modules/robots)
- **[@nuxtjs/sitemap](https://sitemap.nuxtjs.org/)** - Sitemap generation for Nuxt.
   - [GitHub Repository](https://github.com/nuxt-modules/sitemap)
   - [Documentation](https://sitemap.nuxtjs.org/)
- **[@vite-pwa/nuxt](https://vite-pwa-org.netlify.app/frameworks/nuxt)** - PWA plugin for Nuxt.
   - [GitHub Repository](https://github.com/vite-pwa/nuxt)
   - [Documentation](https://vite-pwa-org.netlify.app/frameworks/nuxt)

### Styling & UI Components

- **[TailwindCSS](https://tailwindcss.com/)** - Utility-first CSS framework for fast and responsive styling.
   - [GitHub Repository](https://github.com/tailwindlabs/tailwindcss)
   - [Documentation](https://tailwindcss.com/docs)
- **[Flowbite](https://flowbite.com/)** - Pre-built UI components for TailwindCSS to quickly build out user interfaces.
   - [GitHub Repository](https://github.com/themesberg/flowbite)
   - [Documentation](https://flowbite.com/docs/getting-started/introduction/)

### Maps & Geospatial

- **[MapLibre](https://maplibre.org/)** - Open-source map library for rendering maps and enhancing location-based searches.
   - [GitHub Repository](https://github.com/maplibre/maplibre-gl-js)
   - [Documentation](https://maplibre.org/maplibre-gl-js/docs/)

### Testing

- **[Playwright](https://playwright.dev/)** - End-to-end testing framework.
   - [GitHub Repository](https://github.com/microsoft/playwright)
   - [Documentation](https://playwright.dev/docs/intro)

### State Management

The application uses Pinia stores (located in `@stores`) to manage application state:

- **`nostr.ts`** - Handles Nostr protocol integration:

   - User authentication (Extension/Mnemonic) using [NDK](https://ndk.fiatjaf.com/)
   - Key pair management with [@scure/bip39](https://github.com/paulmillr/scure-bip39)
   - Direct messaging with encryption
   - Push notifications
   - User preferences persistence
   - Message signing and gift wrapping

- **`chat.ts`** - Real-time chat functionality:

   - Message caching using [Dexie](https://dexie.org/) (IndexedDB wrapper)
   - Message queue management
   - Chat history and user profiles
   - Unread message tracking
   - Encrypted message handling with [NDK](https://ndk.fiatjaf.com/)
   - Profile management

- **`properties.ts`** - Real estate property management:

   - Property listing data
   - Property details and metadata
   - Favorite properties tracking
   - Property search and filtering with [Typesense](https://typesense.org/)
   - Property status management

- **`transactions.ts`** - Payment and transaction handling:

   - Transaction status tracking
   - Payment processing with [BTCPayServer](https://btcpayserver.org/)
   - Transaction history
   - Payment method management

- **`filters.ts`** - Search filter management:

   - Property search criteria
   - Filter persistence with [pinia-plugin-persistedstate](https://prazdevs.github.io/pinia-plugin-persistedstate/)
   - Search preferences
   - Filter combinations

- **`settings.ts`** - Application preferences:

   - User interface settings
   - Language preferences with [@nuxtjs/i18n](https://i18n.nuxtjs.org/)
   - Theme configuration
   - Notification settings

- **`search.ts`** - Search functionality:
   - Search query management with [Vue InstantSearch](https://www.algolia.com/doc/guides/building-search-ui/what-is-instantsearch/vue/)
   - Search history
   - Results caching

Each store is modular and follows [Pinia](https://pinia.vuejs.org/)'s composition-based approach with:

- State management for its domain
- Actions for async operations
- Getters for computed state
- Persistence where needed (using [pinia-plugin-persistedstate](https://prazdevs.github.io/pinia-plugin-persistedstate/))

The stores are integrated with Nuxt's auto-import feature and can be accessed using the `use[StoreName]Store()` composable pattern throughout the application.

---

## ⚡ Nostr Service

RESIN uses [Nostr](https://nostr.com/) (Notes and Other Stuff Transmitted by Relays) as its decentralized communication protocol. This integration provides:

### Authentication

- Key-based authentication using [NDK](https://ndk.fiatjaf.com/)
- Support for browser extensions (nos2x, Alby)
- BIP-39 mnemonic phrase backup
- Hierarchical Deterministic (HD) key derivation

### Messaging

- End-to-end encrypted direct messages
- Real estate agent communication
- Property inquiries and negotiations
- Message persistence through relays
- Local caching with IndexedDB

### Profile Management

- Decentralized identity management
- Profile metadata storage (NIP-01)
- Avatar and banner images
- Contact lists and follows

### Technical Implementation

- Uses [NDK](https://ndk.fiatjaf.com/) for protocol interaction
- Implements NIPs (Nostr Implementation Possibilities):
   - NIP-01: Basic protocol flow description
   - NIP-04: Encrypted Direct Messages
   - NIP-05: Mapping Nostr keys to DNS-based internet identifiers
   - NIP-07: `window.nostr` capability for web browsers
- Relay management and connection pooling
- Automatic relay discovery
- Message queue for offline support

### Push Notifications

- Service worker implementation in `public/nostr-notifications-sw.js`
- Background message monitoring using NDK
- Custom relay pool configuration
- Notification handling for:
   - Active windows (in-app notifications)
   - Background state (system notifications)
- Click handling and navigation

To build or update the service worker bundle:

```bash
# Build the NDK bundle for service worker
pnpm build:sw

# This runs the script in scripts/build-sw.js which:
# 1. Bundles NDK and its dependencies
# 2. Minifies the output
# 3. Places it in public/ndk-bundle.min.js
```

### Data Privacy

- All messages are end-to-end encrypted
- Private keys never leave the user's device
- Optional key backup with encrypted mnemonic phrases
- Zero-knowledge profile verification

---

## 🔄 Planned Integrations

### Payment Processing

- **[BTCPayServer](https://btcpayserver.org/)** - Self-hosted, open-source cryptocurrency payment processor.
   - [GitHub Repository](https://github.com/btcpayserver/btcpayserver)
   - [Documentation](https://docs.btcpayserver.org/)
   - **Status**: Implementation in progress for handling property deposits and payments using Bitcoin and Lightning Network.

### Identity Verification

- **[SumSub](https://sumsub.com/)** - Identity verification and KYC/AML compliance platform.
   - [Documentation](https://developers.sumsub.com/)
   - [SDK Reference](https://developers.sumsub.com/api-reference/#introduction)
   - **Status**: Integration implemented for user verification and compliance checks, pending activation with production credentials.

---

## 🛠️ How to Run the Project Locally

Follow these simple steps to get the project up and running on your local machine:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/DaedalusLabs/RESIN
   cd RESIN
   ```

2. **Install dependencies:**

   Make sure you have [Node.js](https://nodejs.org/) installed. Then, run the following command:

   ```bash
   pnpm install
   ```

3. **Start the development server:**

   To start the local development server, run:

   ```bash
   pnpm dev
   ```

4. **Open the app in your browser:**

   Navigate to `http://localhost:3000` to view the app running locally.

## 📝 License

This project is open-sourced under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🧪 Testing

E2E tests are run with [Playwright](https://playwright.dev/) and cover the following scenarios:

### Authentication Tests (`login.spec.ts`)

- NOSTR account login flow using recovery phrase
- Verification of correct public key generation
- Authentication state persistence
- Visual regression testing with screenshots

### Navigation Tests (`navigation.spec.ts`)

- Complete new user onboarding flow:
   - Account creation
   - Property type selection
   - Location selection on map
   - Property listing view
   - Property details view
- Settings menu navigation:
   - Profile access
   - Messages view
   - NOSTR keys management
   - Settings configuration
   - Help section
   - Terms & conditions
- Logout functionality
- Visual regression testing with screenshots

### Settings Pages Tests (`settings-pages.spec.ts`)

- Profile information management
- NOSTR key management
- Application settings configuration
- User preferences persistence

To run the tests, use the following command:

```bash
pnpm test:e2e
```

For interactive test debugging with UI:

```bash
pnpm test:e2e:ui
```
