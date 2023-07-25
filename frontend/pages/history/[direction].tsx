import { NextPage } from "next";

import History from "@components/templates/history";
import { useRouter } from "next/router";
import { QueryRouter } from "src/types/config";
import { BridgeDirection, routes } from "src/constants/routes";
import { useEffect } from "react";

const TransferHistory: NextPage = () => {
    const { query, prefetch, isReady } = useRouter() as QueryRouter<{ direction: BridgeDirection }>;

    useEffect(() => {
        if (isReady) {
            prefetch(
                routes.history(
                    query.direction === BridgeDirection.Deposit ? BridgeDirection.Withdraw : BridgeDirection.Deposit
                )
            );
        }
    }, [isReady, prefetch, query.direction]);

    return <History depositSelected={query.direction === BridgeDirection.Deposit} />;
};

export default TransferHistory;
