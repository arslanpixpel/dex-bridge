import { sha256 } from "@concordium/web-sdk";
import { Buffer } from "buffer/";

export interface MetadataUrl {
    url: string;
    hash?: string;
}
export interface MetadataAttribute {
    type: string;
    name: string;
    value: string;
}
export interface TokenMetadata {
    name?: string;
    symbol?: string;
    decimals?: number;
    description?: string;
    thumbnail?: MetadataUrl;
    display?: MetadataUrl;
    artifact?: MetadataUrl;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    assets?: any;
    attributes?: MetadataAttribute[];
    unique?: boolean;
    localization?: Record<string, MetadataUrl>;
}

/**
 * Returns the url for the token metadata.
 * returnValue is assumed to be a HEX-encoded string.
 */
export function deserializeTokenMetadataReturnValue(returnValue: string): MetadataUrl[] {
    const buf = Buffer.from(returnValue, "hex");
    const n = buf.readUInt16LE(0);
    let cursor = 2; // First 2 bytes hold number of token amounts included in response.
    const urls: MetadataUrl[] = [];

    // eslint-disable-next-line no-plusplus
    for (let i = 0; i < n; i++) {
        const length = buf.readUInt16LE(cursor);
        const urlStart = cursor + 2;
        const urlEnd = urlStart + length;

        const url = Buffer.from(buf.subarray(urlStart, urlEnd)).toString("utf8");

        cursor = urlEnd;

        const hasChecksum = buf.readUInt8(cursor);
        cursor += 1;

        if (hasChecksum === 1) {
            const hash = Buffer.from(buf.subarray(cursor, cursor + 32)).toString("hex");
            cursor += 32;
            urls.push({ url, hash });
        } else if (hasChecksum === 0) {
            urls.push({ url });
        } else {
            throw new Error("Deserialization failed: boolean value had an unexpected value");
        }
    }

    return urls;
}
/**
 * Returns a buffer containing the parameters used for the CIS-2 view function .tokenMetadata, for the given token id.
 */
export function serializeMetadataParameter(ids: string[]): Buffer {
    const queries = Buffer.alloc(2);
    queries.writeInt16LE(ids.length, 0);

    const idBufs = ids.map((id) => {
        const idBuf = Buffer.from(id, "hex");
        const length = Buffer.alloc(1);
        length.writeInt8(idBuf.length, 0);

        return Buffer.concat([length, idBuf]);
    });

    return Buffer.concat([queries, ...idBufs]);
}

/**
 * Fetches token metadata from the given url
 */
export async function getTokenMetadata({ url, hash: checksumHash }: MetadataUrl): Promise<TokenMetadata> {
    const resp = await fetch(url);
    if (!resp.ok) {
        throw new Error(`Error when trying to fetch metadata, status: ${resp.status}`);
    }

    const body = Buffer.from(await resp.arrayBuffer());
    if (checksumHash && sha256([body]).toString("hex") !== checksumHash) {
        throw new Error("Metadata does not match checksum provided with url");
    }

    return JSON.parse(body.toString());
}
