import network from "@config/network";
import { ethers } from "ethers";
import { useCallback, useEffect } from "react";
import { useWeb3Context } from "web3-react";
import detectEthereumProvider from "@metamask/detect-provider";
import { useAsyncMemo } from "./utils";
import { noOp } from "src/helpers/basic";
import { useAccount, useBalance, useConnect, useDisconnect, useNetwork, useSigner } from "wagmi";
import { InjectedConnector } from "wagmi/connectors/injected";
const CHAIN_ID = Number(2221);

// let hasInitialised = false;
const useEthWallet = () => {
    // const context = useWeb3Context();
    const provider = useAsyncMemo(detectEthereumProvider, noOp, []);
    const { connect: _connect } = useConnect({ connector: new InjectedConnector() });
    const { disconnect: _disconnect } = useDisconnect();
    const { data: balance } = useBalance();
    const { address, connector } = useAccount();
    const { chain } = useNetwork();
    const { data: signer } = useSigner();
    const hasWallet = address != undefined; // null/undefined
    // context.
    const context = {
        active: hasWallet,
        account: address,
        connector,
        connectorName: connector?.name,
        networkId: chain?.id,
        library: {
            getSigner() {
                return signer;
            },
        },
    };

    const connect = useCallback(async () => {
        _connect();
        // if (context.networkId !== CHAIN_ID) {
        //     await changeChain(`0x${CHAIN_ID.toString(16)}`);
        // }

        // if (!context.active) {
        //     try {
        //         console.log("setConnector", context);
        //         await context.setConnector("MetaMask", { suppressAndThrowErrors: true });
        //     } catch (e) {
        //         context.unsetConnector();
        //     }
        // }
    }, [context]);

    const disconnect = useCallback(async () => {
        _disconnect();
        // context.unsetConnector();
        delete localStorage["CCP_ETH_connected"];
    }, [context]);

    const getNativeBalance = async () => {
        // if (!context.account) throw new Error("You must be signed in with wallet");

        // const balance = await context.library?.getBalance(context.account);
        // if (!balance) return;

        // return ethers.utils.formatEther(balance);
        console.log(balance?.formatted);
        return balance?.formatted;
    };

    const changeChain = async (chainId: string) => {
        await window?.ethereum?.request?.({
            method: "wallet_switchEthereumChain",
            params: [{ chainId: chainId }], // chainId must be in hexadecimal numbers
        });
    };

    const init = useCallback(async () => {
        // if (context.active) {
        //     return;
        // }
        // const accounts = await window.ethereum?.request?.({ method: "eth_accounts" });
        // if (accounts?.length) {
        //     console.log(accounts);
        //     await context.setConnector("MetaMask");
        // }
    }, [context]);

    useEffect(() => {
        // if (provider !== undefined && !hasInitialised) {
        //     init();
        //     hasInitialised = true;
        // }
    }, [provider, init]);

    useEffect(() => {
        if (context.active) {
            localStorage["CCP_ETH_connected"] = true;
        }
    }, [context]);

    return {
        context,
        connect,
        disconnect,
        getNativeBalance,
        hasWallet,
    };
};

export default useEthWallet;
