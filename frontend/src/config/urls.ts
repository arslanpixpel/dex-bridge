import { ensureDefined } from "src/helpers/basic";

const ccdExplorer = ensureDefined(
    process.env.NEXT_PUBLIC_CCDSCAN_URL,
    "Expected NEXT_PUBLIC_CCDSCAN_URL to be provided as an environment variable"
);
const ethExplorer = ensureDefined(
    process.env.NEXT_PUBLIC_ETHEREUM_EXPLORER_URL,
    "Expected NEXT_PUBLIC_ETHEREUM_EXPLORER_URL to be provided as an environment variable"
);
const bridgeApi = ensureDefined(
    process.env.NEXT_PUBLIC_API_URL,
    "Expected NEXT_PUBLIC_API_URL to be provided as an environment variable"
);

const urls = {
    ccdExplorer,
    ethExplorer,
    bridgeApi,
};

export default urls;
