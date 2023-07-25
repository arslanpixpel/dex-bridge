interface CCDWalletState {
    account: string | undefined;
    isActive: boolean;
}

interface CCDWalletActions {
    setWallet: (ccdWallet: string) => void;
    deleteWallet: () => void;
}

export type CCDWalletStore = CCDWalletState & CCDWalletActions;
