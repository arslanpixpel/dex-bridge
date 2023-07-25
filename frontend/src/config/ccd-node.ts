import { ensureDefined } from "src/helpers/basic";

const url = ensureDefined(
    process.env.NEXT_PUBLIC_CCD_NODE_URL,
    "Expected NEXT_PUBLIC_CCD_NODE_URL to be provided as an environment variable"
);
const port = ensureDefined(
    process.env.NEXT_PUBLIC_CCD_NODE_PORT,
    "Expected NEXT_PUBLIC_CCD_NODE_PORT to be provided as an environment variable"
);

const ccdNode = {
    url,
    port,
};

export default ccdNode;
