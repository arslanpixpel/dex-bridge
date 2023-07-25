import urls from "@config/urls";
import { formatString } from "./string";

export const ccdTransactionUrl = (transactionHash: string) => formatString(urls.ccdExplorer, transactionHash);
export const ethTransactionUrl = (transactionHash: string) => formatString(urls.ethExplorer, `0x${transactionHash}`);
