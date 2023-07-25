export enum BridgeDirection {
    Deposit = "deposit",
    Withdraw = "withdraw",
}

export const routes = {
    deposit: {
        path: "/",
        overview: "/deposit/overview",
        tx: (ethTxHash: string) => `/deposit/${ethTxHash}`,
    },
    withdraw: {
        path: "/withdraw",
        overview: "/withdraw/overview",
        tx: (ccdTxHash: string) => `/withdraw/${ccdTxHash}`,
    },
    history: (direction = BridgeDirection.Deposit) => `/history/${direction}`,
};
