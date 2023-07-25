import { Components } from "src/api-query/__generated__/AxiosClient";

const isDeposit = (
    transaction: { Deposit: Components.Schemas.WalletDepositTx } | { Withdraw: Components.Schemas.WalletWithdrawTx }
): transaction is { Deposit: Components.Schemas.WalletDepositTx } => {
    return (transaction as { Deposit: Components.Schemas.WalletDepositTx }).Deposit?.tx_hash !== undefined;
};

export default isDeposit;
