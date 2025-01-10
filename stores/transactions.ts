import { defineStore } from "pinia";

interface Transaction {
   id: string;
   amount: number;
   due: Date;
   paid: Date;
   status: "pending" | "paid" | "overdue";
}

interface Agreement {
   id: string;
   title: string;
   signed: Date;
   downloadUrl: string;
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
}

export const useTransactionsStore = defineStore("transactions", {
   state: (): TransactionsState => ({
      transactions: [
         // {
         //    id: "1",
         //    amount: 2350,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "pending",
         // },
         // {
         //    id: "2",
         //    amount: 4780,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "paid",
         // },
         // {
         //    id: "3",
         //    amount: 1890,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "paid",
         // },
         // {
         //    id: "4",
         //    amount: 3420,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "pending",
         // },
         // {
         //    id: "5",
         //    amount: 5960,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "paid",
         // },
         // {
         //    id: "6",
         //    amount: 2890,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "pending",
         // },
         // {
         //    id: "7",
         //    amount: 4150,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "paid",
         // },
         // {
         //    id: "8",
         //    amount: 3780,
         //    due: new Date(),
         //    paid: new Date(),
         //    status: "pending",
         // },
      ],
      agreements: [
         {
            id: "1",
            title: "Agreement 1",
            signed: new Date(),
            downloadUrl: "/agreements/agreement-1.pdf",
         },
         {
            id: "2",
            title: "Agreement 2",
            signed: new Date(),
            downloadUrl: "/agreements/agreement-2.pdf",
         },
         {
            id: "3",
            title: "Agreement 3",
            signed: new Date(),
            downloadUrl: "/agreements/agreement-3.pdf",
         },
      ],
      equities: [
         {
            id: "1",
            payoff: 2350,
            total: 2350,
         },
         {
            id: "2",
            payoff: 4780,
            total: 4780,
         },
         {
            id: "3",
            payoff: 1890,
            total: 1890,
         },
         {
            id: "4",
            payoff: 3420,
            total: 3420,
         },
         {
            id: "5",
            payoff: 5960,
            total: 5960,
         },
         {
            id: "6",
            payoff: 2890,
            total: 2890,
         },
         {
            id: "7",
            payoff: 4150,
            total: 4150,
         },
         {
            id: "8",
            payoff: 3780,
            total: 3780,
         },
      ],
      ownerships: [
         {
            propertyId:
               "1a5b8c2e4d7f3a6b9c2e5d8f4a7b3c6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d",
            percentage: 25,
         },
         {
            propertyId:
               "2b6c9d3f5e8a4b7c3f6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d7f1a4b7c3e6",
            percentage: 50,
         },
         {
            propertyId:
               "3c7d0e4f6f9b5c8d4f7e0e3b6f9c2e5b8a4d7f1a4b7c3e6d9f2a5b8c1e4",
            percentage: 80,
         },
      ],
   }),

   getters: {
      getTransactions(): Transaction[] {
         return this.transactions;
      },
      getPayedOffPercentage(): number {
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
            (acc, transaction) => acc + transaction.amount,
            0,
         );
      },
      getPaidOffAmount(): number {
         return this.transactions
            .filter((transaction) => transaction.status === "paid")
            .reduce((acc, transaction) => acc + transaction.amount, 0);
      },
      getToBePaidOffAmount(): number {
         return this.transactions
            .filter((transaction) => transaction.status === "pending")
            .reduce((acc, transaction) => acc + transaction.amount, 0);
      },
      getAgreements(): Agreement[] {
         return this.agreements;
      },
   },

   actions: {
      getOwnerShipPercentage(propertyId: string): number {
         return (
            this.ownerships.find(
               (ownership) => ownership.propertyId === propertyId,
            )?.percentage ?? 0
         );
      },
   },
});
