import { useCallback } from "react";
import { useTokens } from "src/api-query/queries";
import { Components } from "src/api-query/__generated__/AxiosClient";
import isDeposit from "src/helpers/checkTransaction";

type TransactionTokenResponse =
    | {
          status: "error" | "loading" | "idle";
      }
    | {
          status: "success";
          token: Components.Schemas.TokenMapItem | undefined;
      };

export function useGetTransactionToken() {
    const tokensQuery = useTokens();

    return useCallback(
        (transaction: Components.Schemas.WalletTx): TransactionTokenResponse => {
            if (tokensQuery.status !== "success") {
                return { status: tokensQuery.status };
            }

            const { token } =
                tokensQuery.data?.find(({ token: t }) => {
                    if (isDeposit(transaction)) {
                        return t.eth_address === transaction.Deposit.root_token;
                    } else {
                        return (
                            t.ccd_contract?.index === transaction.Withdraw.child_token?.index &&
                            t.ccd_contract?.subindex === transaction.Withdraw.child_token?.subindex
                        );
                    }
                }) ?? {};

            return { status: tokensQuery.status, token };
        },
        [tokensQuery]
    );
}
