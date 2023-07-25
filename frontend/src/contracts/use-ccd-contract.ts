import {
    AccountAddress,
    AccountTransactionType,
    CcdAmount,
    ContractAddress,
    UpdateContractPayload,
} from "@concordium/common-sdk";
import { serializeUpdateContractParameters } from "@concordium/web-sdk";
import addresses from "@config/addresses";
import contractNames from "@config/contractNames";
import { bridgeManager, cis2Bridgeable } from "@config/schemas";
import leb from "leb128";
import { Buffer } from "buffer/index";
import { Components } from "src/api-query/__generated__/AxiosClient";
import decodeOperatorOf from "src/helpers/decodeOperatorOf";
import { detectConcordiumProvider } from "@concordium/browser-wallet-api-helpers";
import { collapseRatio } from "src/helpers/ccd-node";

type EstimateResponse = {
    /** exact amount of energy used for contract invocation. */
    exact: bigint;
    /** conservative amount of energy used for contract invocation. Good for passing from estimate invocation to actual invocation. */
    conservative: bigint;
};

/** Adds 10% to estimate given to function. */
const getConservativeEstimate = (estimate: bigint) => collapseRatio({ numerator: estimate * 110n, denominator: 100n });

/** Strips the hex string identifier from the hex string, returning just the hex value. */
const stripHexId = (hexString: string) => hexString.replace("0x", "");

const useCCDContract = (ccdAccount: string | undefined, enabled: boolean) => {
    const bridgeManagerContract = {
        index: BigInt(addresses.bridgeManager.index),
        subindex: BigInt(addresses.bridgeManager.subindex),
    } as ContractAddress;

    const approve = async (token: Components.Schemas.TokenMapItem, maxContractExecutionEnergy?: bigint) => {
        if (!ccdAccount || !enabled) {
            throw new Error("No account available");
        }
        if (maxContractExecutionEnergy === undefined) {
            throw new Error("Energy is undefined");
        }
        if (token.ccd_contract?.index === undefined || token.ccd_contract.subindex === undefined) {
            throw new Error("Contract address undefined");
        }

        const contractAddress = {
            index: BigInt(token.ccd_contract?.index),
            subindex: BigInt(token.ccd_contract?.subindex),
        } as ContractAddress;

        const receiveName = `${contractNames.cis2Bridgeable}.updateOperator`;
        const rawSchema = cis2Bridgeable;
        const provider = await detectConcordiumProvider();
        const userInput = [
            {
                update: {
                    Add: [],
                },
                operator: {
                    Contract: [
                        {
                            index: +addresses.bridgeManager.index,
                            subindex: +addresses.bridgeManager.subindex,
                        },
                    ],
                },
            },
        ];

        const txHash = await provider.sendTransaction(
            ccdAccount,
            AccountTransactionType.Update,
            {
                amount: new CcdAmount(BigInt(0)),
                address: contractAddress,
                receiveName: receiveName,
                maxContractExecutionEnergy,
            } as UpdateContractPayload,
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            userInput as any,
            rawSchema,
            2
        );

        return txHash;
    };

    const withdraw = async function (
        amount: bigint,
        token?: Components.Schemas.TokenMapItem,
        ethAddress?: string,
        maxContractExecutionEnergy?: bigint
    ): Promise<string> {
        if (!ccdAccount || !enabled) {
            throw new Error("No account available");
        }

        if (token?.ccd_contract?.index === undefined || token?.ccd_contract?.subindex === undefined) {
            throw new Error("ccdToken is undefined");
        }
        if (maxContractExecutionEnergy === undefined) {
            throw new Error("Energy is undefined");
        }
        if (!ethAddress) {
            throw new Error("ETH address is undefined");
        }

        const receiveName = `${contractNames.bridgeManager}.withdraw`;
        const rawSchema = bridgeManager;
        const provider = await detectConcordiumProvider();

        const txHash = await provider.sendTransaction(
            ccdAccount,
            AccountTransactionType.Update,
            {
                amount: new CcdAmount(BigInt(0)),
                address: bridgeManagerContract,
                receiveName: receiveName,
                maxContractExecutionEnergy: maxContractExecutionEnergy,
            } as UpdateContractPayload,
            {
                eth_address: stripHexId(ethAddress),
                amount: amount.toString(),
                token: {
                    index: token.ccd_contract.index,
                    subindex: token.ccd_contract.subindex,
                },
                token_id: "0000000000000000",
            },
            rawSchema,
            2
        );

        return txHash;
    };

    const getTransactionStatus = async (hash: string) => {
        const provider = detectConcordiumProvider();
        return (await provider).getJsonRpcClient().getTransactionStatus(hash);
    };

    const balanceOf = async function (token: Components.Schemas.TokenMapItem): Promise<bigint> {
        if (token?.ccd_contract?.index === undefined || token?.ccd_contract.subindex === undefined) {
            throw new Error("ccdToken is undefined");
        }

        const param = serializeUpdateContractParameters(
            contractNames.cis2Bridgeable,
            "balanceOf",
            [
                {
                    address: {
                        Account: [ccdAccount],
                    },
                    token_id: "",
                },
            ],
            Buffer.from(cis2Bridgeable, "base64")
        );

        const provider = await detectConcordiumProvider();
        const res = await provider.getJsonRpcClient().invokeContract({
            method: `${contractNames.cis2Bridgeable}.balanceOf`,
            contract: {
                index: BigInt(token.ccd_contract.index),
                subindex: BigInt(token.ccd_contract.subindex),
            },
            parameter: param,
        });
        if (!res || res.tag === "failure" || !res.returnValue) {
            throw new Error(
                `RPC call 'invokeContract' on method '${contractNames.cis2Bridgeable}.balanceOf' of contract '${token.ccd_contract.index}' failed`
            );
        }

        // The return value is an array. The value stored in the array starts at position 4 of the return value.
        return BigInt(leb.unsigned.decode(Buffer.from(res.returnValue.slice(4), "hex")));
    };

    const hasApprove = async (ccdTokenAddress?: { index?: number; subindex?: number }) => {
        if (!enabled || !ccdAccount) return;

        if (!ccdTokenAddress?.index || (!ccdTokenAddress?.subindex && ccdTokenAddress?.subindex !== 0)) {
            throw new Error("ccdTokenAddress is undefined");
        }

        const provider = await detectConcordiumProvider();
        const userInput = [
            {
                owner: {
                    Account: [ccdAccount],
                },
                address: {
                    Contract: [
                        {
                            index: +addresses.bridgeManager.index,
                            subindex: +addresses.bridgeManager.subindex,
                        },
                    ],
                },
            },
        ];

        // calculateEnergyCost
        // https://github.dev/Concordium/concordium-browser-wallet/blob/main/packages/browser-wallet/src/popup/pages/SendTransaction/SendTransaction.tsx#L83

        const moduleFileBuffer = Buffer.from(cis2Bridgeable, "base64");

        const params = serializeUpdateContractParameters(
            contractNames.cis2Bridgeable,
            "operatorOf",
            userInput,
            moduleFileBuffer
        );

        const res = await provider.getJsonRpcClient().invokeContract({
            invoker: new AccountAddress(ccdAccount),
            contract: {
                index: BigInt(ccdTokenAddress.index),
                subindex: BigInt(ccdTokenAddress.subindex),
            },
            amount: new CcdAmount(BigInt(0)),
            method: `${contractNames.cis2Bridgeable}.operatorOf`,
            parameter: params,
            energy: BigInt(30000),
        });

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const isApproved = decodeOperatorOf((res as any).returnValue);
        return isApproved;
    };

    const getLatestFinalizedBlock = async function () {
        const provider = await detectConcordiumProvider();
        const res = await provider.getJsonRpcClient().getConsensusStatus();
        return res.lastFinalizedBlock;
    };

    const estimateWithdraw = async (
        amount: bigint,
        token?: Components.Schemas.TokenMapItem,
        ethAddress?: string
    ): Promise<EstimateResponse | undefined> => {
        if (!enabled || !ccdAccount) return;

        if (token?.ccd_contract?.index === undefined || token?.ccd_contract.subindex === undefined) {
            throw new Error("ccdToken is undefined");
        }
        if (!ethAddress) {
            throw new Error("ETH address is undefined");
        }

        const provider = await detectConcordiumProvider();
        const userInput = {
            eth_address: stripHexId(ethAddress),
            amount: amount.toString(),
            token: {
                index: token.ccd_contract.index,
                subindex: token.ccd_contract.subindex,
            },
            token_id: "0000000000000000",
        };

        const moduleFileBuffer = Buffer.from(bridgeManager, "base64");
        const params = serializeUpdateContractParameters(
            contractNames.bridgeManager,
            "withdraw",
            userInput,
            moduleFileBuffer
        );

        const res = await provider.getJsonRpcClient().invokeContract({
            invoker: new AccountAddress(ccdAccount),
            contract: bridgeManagerContract,
            method: `${contractNames.bridgeManager}.withdraw`,
            amount: undefined,
            parameter: params,
            energy: BigInt(30000),
        });

        if (!res || res.tag === "failure") {
            throw new Error(
                `RPC call 'invokeContract' on method '${contractNames.bridgeManager}.withdraw' of contract '${
                    bridgeManagerContract.index
                    // eslint-disable-next-line @typescript-eslint/no-explicit-any
                }' failed with rejectReason ${(res as any)?.reason?.rejectReason}`
            );
        }

        if (res === undefined) {
            return undefined;
        }

        return { exact: res.usedEnergy, conservative: getConservativeEstimate(res.usedEnergy) };
    };

    const estimateApprove = async (token?: Components.Schemas.TokenMapItem): Promise<EstimateResponse | undefined> => {
        if (!enabled || !ccdAccount) return;

        if (token?.ccd_contract?.index === undefined || token?.ccd_contract?.subindex === undefined) {
            throw new Error("ccdToken is undefined");
        }

        const provider = await detectConcordiumProvider();
        const contractAddress = {
            index: BigInt(token.ccd_contract.index),
            subindex: BigInt(token.ccd_contract.subindex),
        };
        const moduleFileBuffer = Buffer.from(cis2Bridgeable, "base64");
        const userInput = [
            {
                update: {
                    Add: {},
                },
                operator: {
                    Contract: [
                        {
                            index: token?.ccd_contract.index,
                            subindex: token?.ccd_contract.subindex,
                        },
                    ],
                },
            },
        ];

        const params = serializeUpdateContractParameters(
            `${contractNames.cis2Bridgeable}`,
            "updateOperator",
            userInput,
            moduleFileBuffer
        );

        const res = await provider.getJsonRpcClient().invokeContract({
            invoker: new AccountAddress(ccdAccount),
            contract: contractAddress,
            method: `${contractNames.cis2Bridgeable}.updateOperator`,
            amount: undefined,
            parameter: params,
            energy: BigInt(30000),
        });

        if (res === undefined) {
            return undefined;
        }

        return { exact: res.usedEnergy, conservative: getConservativeEstimate(res.usedEnergy) };
    };

    return {
        approve,
        withdraw,
        getTransactionStatus,
        balanceOf,
        hasApprove,
        getLatestFinalizedBlock,
        estimateWithdraw,
        estimateApprove,
    };
};

export default useCCDContract;
