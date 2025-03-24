<img src="/resin-front/public/wioso_logo.png" alt="Logo" width="300px" style="margin-bottom: 1rem">

# 🏡 Wioso – Buy a home without a loan

**Wioso** is a modern real estate web application built with **Rust**, **Yew**, and **Nostr**. It allows users to explore properties, view details, save favorites, and securely contact agents.  

## ✨ Features  

- 📌 **Property Listings** – Browse available properties with detailed information.  
- 📌 **Interactive Map** – View property locations on a map.  
- 📌 **Secure Messaging** – Contact real estate agents via **Nostr**.  
- 📌 **Favorites** – Save properties for later review.  
- 📌 **Profile Management** – Manage your account using **nostr keys**.
- 📌 **Lucide Yew** - Flexible icon family for interfaces [(Lucide Yew Docs)](https://docs.rs/lucide-yew/latest/lucide_yew/)

---

## 🛠 Tech Stack  

This application leverages modern technologies for performance and decentralization:  

- **🦀 Rust** – A high-performance and memory-safe programming language. [(Rust Docs)](https://www.rust-lang.org/)  
- **🌿 Yew** – A Rust-based framework for building web applications. [(Yew Docs)](https://yew.rs/docs/)  
- **🔑 Nostr** – A decentralized protocol for secure communication. [(Nostr Docs)](https://nostr.com/)

---

## 🏡 How Wioso Uses Nostr

Wioso leverages **Nostr** as its primary communication protocol for a trustless and decentralized experience. 

### 🔗 Communication with Relays
- All messages, transactions, and interactions are sent via **Nostr relays**, removing the need for centralized servers.
- Each app (consumer, business, admin, etc.) subscribes to specific events in the network, ensuring seamless interaction.

### 🔑 Authentication
- Users authenticate using **Nostr public/private keys** instead of traditional logins.
- Private keys are securely stored on the client-side, never exposing them to third parties.

### 📦 Data Storage
- No centralized database is used. Instead, **IndexedDB** is leveraged for local storage.
- Users retain control over their data, which can be synchronized across devices via Nostr relays.

Using **Nostr** ensures Fuente remains **censorship-resistant**, **fault-tolerant**, and **decentralized**, making it an ideal solution for borderless commerce.

---

## 📂 Project Structure  

The project is organized as follows:

```sh
/wioso
│── /components        # Reusable components
│── /listing-test      # Listing test module
│── /resin-core        # Core logic and data models
│── /resin-front       # Frontend built with Yew
│── /resin-server      # Backend server in Rust
│── Cargo.toml         # Rust dependencies and configuration
│── README.md          # Project documentation
```

---

## 📦 Installation & Running  

### 1️⃣ Clone the repository  

```sh
git clone https://gitlab.illuminodes.com/illuminodes/daedaluslabs/resin-rust.git
cd wioso
```

### 2️⃣ Start the project

```sh
trunk serve --config resin-front/Trunk.toml
```

---

## 🔗 Documentation & Resources  

- 🦀 **Rust** – [Rust Docs](https://www.rust-lang.org/)  
- 🌿 **Yew** – [Yew Docs](https://yew.rs/docs/)  
- 🔑 **Nostr** – [Nostr Docs](https://nostr.com/)