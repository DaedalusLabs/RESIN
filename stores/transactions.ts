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

interface TransactionsState {
   transactions: Transaction[];
   agreements: Agreement[];
}

export const useTransactionsStore = defineStore("transactions", {
   state: (): TransactionsState => ({
      transactions: [
         {
            id: "1",
            amount: 2350,
            due: new Date(),
            paid: new Date(),
            status: "pending",
         },
         {
            id: "2",
            amount: 4780,
            due: new Date(),
            paid: new Date(),
            status: "paid",
         },
         {
            id: "3",
            amount: 1890,
            due: new Date(),
            paid: new Date(),
            status: "paid",
         },
         {
            id: "4",
            amount: 3420,
            due: new Date(),
            paid: new Date(),
            status: "pending",
         },
         {
            id: "5",
            amount: 5960,
            due: new Date(),
            paid: new Date(),
            status: "paid",
         },
         {
            id: "6",
            amount: 2890,
            due: new Date(),
            paid: new Date(),
            status: "pending",
         },
         {
            id: "7",
            amount: 4150,
            due: new Date(),
            paid: new Date(),
            status: "paid",
         },
         {
            id: "8",
            amount: 3780,
            due: new Date(),
            paid: new Date(),
            status: "pending",
         },
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

   actions: {},
});
