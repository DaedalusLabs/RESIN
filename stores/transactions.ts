import { defineStore } from "pinia";
import type { NostrAPI } from "~/lib/nostr_api";
import { commands } from "~/lib/nostr_api";
import { useRuntimeConfig } from "#app";

interface Property {
   id: string;
   name: string;
   ownershipPercentages: Record<string, number>;
}

interface Payment {
   date: string;
   amount: string;
   currency: string;
}

export interface Transaction {
   id: string;
   amount: string;
   currency: string;
   status: "pending" | "paid" | "overdue";
   dueDate: string;
   property: Property;
   payment: Payment | null;
}

export interface Agreement {
   id: string;
   title: string;
   sha256: string;
   url: string;
   isSigned: boolean;
   signedDate: string;
   property: Property;
}

interface Equity {
   id: string;
   payoff: number;
   total: number;
}

interface Ownership {
   propertyId: string;
   percentage: number;
}

interface TransactionsState {
   transactions: Transaction[];
   agreements: Agreement[];
   equities: Equity[];
   ownerships: Ownership[];
   isLoading: boolean;
   error: string | null;
}

export const useTransactionsStore = defineStore("transactions", {
   state: (): TransactionsState => ({
      transactions: [],
      agreements: [],
      equities: [],
      ownerships: [],
      isLoading: false,
      error: null
   }),

   getters: {
      getTransactions(): Transaction[] {
         return this.transactions;
      },
      getPayedOffPercentage(): number {
         if (this.transactions.length === 0) return 0;
         return (
            (this.transactions.filter(
               (transaction) => transaction.status === "paid",
            ).length /
               this.transactions.length) *
            100
         );
      },
      getTotalAmount(): number {
         return this.transactions.reduce(
            (acc, transaction) => acc + parseFloat(transaction.amount),
            0,
         );
      },
      getPaidOffAmount(): number {
         return this.transactions
            .filter((transaction) => transaction.status === "paid")
            .reduce((acc, transaction) => acc + parseFloat(transaction.amount), 0);
      },
      getToBePaidOffAmount(): number {
         return this.transactions
            .filter((transaction) => transaction.status === "pending")
            .reduce((acc, transaction) => acc + parseFloat(transaction.amount), 0);
      },
      getAgreements(): Agreement[] {
         return this.agreements;
      },
   },

   actions: {
      getOwnerShipPercentage(propertyId: string): number {
         const transaction = this.transactions.find(t => t.property.id === propertyId);
         if (!transaction) return 0;
         
         // Get the first ownership percentage value (assuming one owner for now)
         const percentages = Object.values(transaction.property.ownershipPercentages);
         return percentages.length > 0 ? percentages[0] : 0;
      },

      async fetchTransactions(nostrApi: NostrAPI) {
         try {
            this.isLoading = true;
            this.error = null;
            const config = useRuntimeConfig();
            const targetPubkey = config.public.MESSAGES_PUBKEY;

            // Fetch transactions
            const transactionsData = await nostrApi.request<Transaction[]>(
               targetPubkey,
               commands.get_transactions
            );
            this.transactions = transactionsData;

            // Fetch agreements
            const agreementsData = await nostrApi.request<Agreement[]>(
               targetPubkey,
               commands.get_agreements
            );
            this.agreements = agreementsData;

            // Calculate equities based on transactions
            this.equities = this.transactions.map(transaction => ({
               id: transaction.id,
               payoff: transaction.status === "paid" ? parseFloat(transaction.amount) : 0,
               total: parseFloat(transaction.amount)
            }));

            // Calculate ownerships from transaction properties
            const ownershipMap = new Map<string, number>();
            this.transactions.forEach(transaction => {
               const percentages = Object.values(transaction.property.ownershipPercentages);
               if (percentages.length > 0) {
                  ownershipMap.set(transaction.property.id, percentages[0]);
               }
            });
            
            this.ownerships = Array.from(ownershipMap.entries()).map(([propertyId, percentage]) => ({
               propertyId,
               percentage
            }));

         } catch (error) {
            console.error('Error fetching transactions:', error);
            this.error = error instanceof Error ? error.message : 'Failed to fetch transactions';
         } finally {
            this.isLoading = false;
         }
      },
   },
});
