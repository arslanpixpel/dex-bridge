import type { NextPage } from "next";
import { useEffect } from "react";
import { useRouter } from "next/router";

import Transfer from "@components/templates/transfer";
import { routes } from "src/constants/routes";

const Withdraw: NextPage = () => {
    const { prefetch } = useRouter();

    useEffect(() => {
        prefetch(routes.withdraw.overview);
        prefetch(routes.deposit.path);
    }, [prefetch]);

    return <Transfer />;
};

export default Withdraw;
