import { CCDWalletStore } from "src/types/store/ccd-wallet";
import create from "zustand";

const useCCDWalletStore = create<CCDWalletStore>((set) => ({
    account: undefined,
    isActive: false,
    setWallet: (address: string) => {
        set({ account: address, isActive: true });
        localStorage["CCP_CCD_connected"] = true;
    },
    deleteWallet: () => {
        set({
            account: undefined,
            isActive: false,
        });
        delete localStorage["CCP_CCD_connected"];
    },
}));

export default useCCDWalletStore;
