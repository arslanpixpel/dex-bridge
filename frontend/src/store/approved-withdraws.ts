import create from "zustand";
import { persist } from "zustand/middleware";

type ApprovedWithdrawalsStore = {
    transactions: Record<string, string>;
    addApproved(ccdTx: string, ethTx: string): void;
    remove(ccdTx: string): void;
};

export const useApprovedWithdrawalsStore = create(
    persist<ApprovedWithdrawalsStore>(
        (set, get) => ({
            transactions: {},
            addApproved: (ccdTx: string, ethTx: string) =>
                set({ transactions: { ...get().transactions, [ccdTx]: ethTx } }),
            remove: (ccdTx: string) => {
                if (get().transactions[ccdTx] === undefined) {
                    return;
                }

                const ts = { ...get().transactions };
                delete ts[ccdTx];
                set({ transactions: ts });
            },
        }),
        { name: "eth-ccd-bridge.approved-withdrawals" }
    )
);
