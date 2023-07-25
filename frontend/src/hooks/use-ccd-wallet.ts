import { useCallback, useEffect, useMemo } from "react";
import { detectConcordiumProvider } from "@concordium/browser-wallet-api-helpers";
import useCCDWalletStore from "src/store/ccd-wallet/ccdWalletStore";
import network from "@config/network";
import { useAsyncMemo } from "./utils";
import { noOp } from "src/helpers/basic";

/**
 * Returns undefined if `provider.getSelectedChain returns undefined`.
 * Throws if API is unavailable
 */
const isNetworkMatchNew = async () => {
    const provider = await detectConcordiumProvider();

    // TODO: remove any cast when concordium browser wallet version 1 has been released.
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    if ((provider as any).getSelectedChain === undefined) {
        throw new Error("New API not available");
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const selectedChain = await (provider as any).getSelectedChain();

    if (selectedChain === undefined) {
        return undefined;
    }

    return selectedChain === network.ccd.genesisHash;
};

const isNetworkMatchOld = async () => {
    const provider = await detectConcordiumProvider();
    const client = provider.getJsonRpcClient();

    try {
        const result = await client.getCryptographicParameters(network.ccd.genesisHash);

        if (result === undefined || result?.value === null) {
            throw new Error("Genesis block not found");
        }

        return true;
    } catch {
        return false;
    }
};

let hasInitialised = false;
const useCCDWallet = () => {
    const provider = useAsyncMemo(detectConcordiumProvider, noOp, []);
    const hasWallet = provider !== undefined;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const hasNewApi = (provider as any)?.getSelectedChain !== undefined;
    const { deleteWallet, setWallet, account, isActive } = useCCDWalletStore();
    const ccdContext = useMemo(() => ({ account, isActive }), [account, isActive]);

    const matchesExpectedNetwork = useCallback(async () => {
        if (hasNewApi) {
            return (await isNetworkMatchNew()) ?? false;
        }

        return await isNetworkMatchOld();
    }, [hasNewApi]);

    const refreshMostRecentlySelectedAccount = useCallback(async () => {
        const account = await provider?.getMostRecentlySelectedAccount();

        if (account) {
            setWallet(account);
        }
    }, [setWallet, provider]);

    const init = useCallback(async () => {
        if (ccdContext.isActive) {
            return;
        }

        const networkMatch = await matchesExpectedNetwork();
        if (networkMatch) {
            refreshMostRecentlySelectedAccount();
        } else {
            deleteWallet();
        }
    }, [ccdContext.isActive, refreshMostRecentlySelectedAccount, matchesExpectedNetwork, deleteWallet]);

    /**
     * Throws if API not available
     */
    const connectCCD = useCallback(async () => {
        let networkMatch: boolean | undefined = undefined;
        if (hasNewApi) {
            networkMatch = await isNetworkMatchNew();
        }

        if (networkMatch === false) {
            // New API found, wrong network in wallet
            throw new Error("Wrong network in concordium wallet");
        }

        let account: string | undefined;
        try {
            account = await provider?.connect();
        } catch {
            // Connection request rejected in wallet
            deleteWallet();
            return;
        }

        // New API not found or couldn't be used, use fallback network match
        if (networkMatch === undefined && !(await isNetworkMatchOld())) {
            throw new Error("Genesis block for expected network not found");
        }

        if (account) {
            setWallet(account);
        }
    }, [deleteWallet, setWallet, hasNewApi, provider]);

    useEffect(() => {
        if (provider !== undefined && !hasInitialised) {
            init();
            hasInitialised = true;
        }
    }, [init, provider]);

    return {
        ccdContext,
        /**
         * Throws if API not available
         */
        connectCCD,
        disconnectCCD: deleteWallet,
        refreshMostRecentlySelectedAccount,
        hasWallet,
    };
};

export default useCCDWallet;
