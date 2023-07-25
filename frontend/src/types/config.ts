import type { WalletApi } from "@concordium/browser-wallet-api-helpers";
import type { NextRouter } from "next/router";
import type { UrlObject } from "url";
import type { providers } from "ethers";

export type QueryRouter<T extends UrlObject["query"]> = NextRouter & { query: Partial<T> };

declare global {
    interface Window {
        concordium: WalletApi | undefined;
        ethereum: providers.ExternalProvider | undefined;
    }
}
