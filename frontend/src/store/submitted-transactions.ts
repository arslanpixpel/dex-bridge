import { BigNumberish } from "ethers";
import { Components } from "src/api-query/__generated__/AxiosClient";
import create, { StateCreator } from "zustand";
import { persist } from "zustand/middleware";

export type SubmittedTransaction = {
    hash: string;
    /** Integer */
    amount: string;
    token: Components.Schemas.TokenMapItem;
    /** In seconds */
    timestamp: number;
    /** ETH account involved in the transaction */
    ethAccount: string;
};

type SubmittedTransactions = Record<string, SubmittedTransaction[]>;

type SubmittedTransactionsStore = {
    transactions: SubmittedTransactions;
    add(
        ethAccount: string,
        transactionHash: string,
        amount: BigNumberish,
        token: Components.Schemas.TokenMapItem
    ): void;
    get(ethAccount: string): SubmittedTransaction[];
    remove(transactionHash: string): void;
};

const storeCreator: StateCreator<SubmittedTransactionsStore> = (set, get) => ({
    transactions: {},
    get: (ethAccount) => get().transactions[ethAccount] ?? [],
    add: (ethAccount, hash, amount, token) =>
        set({
            transactions: {
                ...get().transactions,
                [ethAccount]: [
                    ...get().get(ethAccount),
                    {
                        hash,
                        amount: amount.toString(),
                        token,
                        timestamp: Math.floor(Date.now() / 1000),
                    } as SubmittedTransaction,
                ],
            },
        }),
    remove: (hash) => {
        const [account, transactions] =
            Object.entries(get().transactions).find(([, txs]) => txs.some((tx) => tx.hash === hash)) ?? [];

        if (!account || !transactions?.length) {
            return;
        }

        set({
            transactions: {
                ...get().transactions,
                [account]: transactions.filter((tx) => tx.hash !== hash),
            },
        });
    },
});

export const useSubmittedDepositsStore = create(
    persist<SubmittedTransactionsStore>(storeCreator, {
        name: "eth-ccd-bridge.submitted-deposits",
    })
);

export const useSubmittedWithdrawalsStore = create(
    persist<SubmittedTransactionsStore>(storeCreator, {
        name: "eth-ccd-bridge.submitted-withdrawals",
    })
);
