import { Components } from "src/api-query/__generated__/AxiosClient";
import create from "zustand";

type TransactionFlowStore = {
    amount?: bigint;
    token?: Components.Schemas.TokenMapItem;
    transactionHash?: string;
    setAmount(amount: bigint): void;
    setToken(token: Components.Schemas.TokenMapItem): void;
    setTransactionHash(txHash: string): void;
    clear(): void;
};

/**
 * Value store to be used for deposit/withdraw flows.
 */
export const useTransactionFlowStore = create<TransactionFlowStore>((set) => ({
    setAmount: (amount) => set({ amount, transactionHash: undefined }),
    setToken: (token) => set({ token, transactionHash: undefined }),
    setTransactionHash: (transactionHash) => set({ transactionHash }),
    clear: () => set({ amount: undefined, token: undefined, transactionHash: undefined }),
}));
