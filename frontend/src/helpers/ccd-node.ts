import { createConcordiumClient, Ratio, ConcordiumGRPCClient } from "@concordium/web-sdk";
import ccdNode from "@config/ccd-node";
import contractNames from "@config/contractNames";
import {
    deserializeTokenMetadataReturnValue,
    serializeMetadataParameter,
    getTokenMetadata,
    MetadataUrl,
} from "./token-helpers";

let client: ConcordiumGRPCClient;
const getClient = () => {
    if (!client) {
        client = createConcordiumClient(ccdNode.url, Number(ccdNode.port));
    }
    return client;
};

/**
 * Collapses the `Ratio` into a single number.
 * If the denominator does not divide the numerator, the function rounds up;
 */
export function collapseRatio({ numerator, denominator }: Ratio): bigint {
    const quotient = numerator / denominator;
    if (numerator % denominator === 0n) {
        return quotient;
    }
    return 1n + quotient;
}

/**
 * Gets energy to microCCD rate from the concordium node through the grpc v2 interface configured through environment variables
 */
export const getEnergyToMicroCcdRate = async (): Promise<bigint> => {
    const client = getClient();
    const { euroPerEnergy, microGTUPerEuro } = await client.getBlockChainParameters();

    const denominator = euroPerEnergy.denominator * microGTUPerEuro.denominator;
    const numerator = euroPerEnergy.numerator * microGTUPerEuro.numerator;

    return collapseRatio({ numerator, denominator });
};

const getTokenUrl = async (index: bigint, subindex: bigint): Promise<MetadataUrl> => {
    const client = getClient();

    const returnValue = await client.invokeContract({
        contract: { index, subindex },
        method: `${contractNames.cis2Bridgeable}.tokenMetadata`,
        parameter: serializeMetadataParameter([""]),
    });

    if (returnValue && returnValue.tag === "success" && returnValue.returnValue) {
        return deserializeTokenMetadataReturnValue(returnValue.returnValue)[0];
    } else {
        throw new Error(`Token does not exist in contract at <${index}, ${subindex}>`);
    }
};

export const tokenMetadataFor = async (index: bigint, subindex: bigint) => {
    const metadataUrl = await getTokenUrl(index, subindex);
    try {
        return getTokenMetadata(metadataUrl);
    } catch (e) {
        throw new Error(`Failed to get metadata for contract at <${index}, ${subindex}>`);
    }
};

export const waitForTransactionFinalization = (hash: string, timeout?: number) =>
    getClient().waitForTransactionFinalization(hash, timeout);
