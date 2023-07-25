import { Connectors } from "web3-react";
const { InjectedConnector } = Connectors;

const MetaMask = new InjectedConnector({ supportedNetworks: [1, 4, 5] });

// const Concordium = new NetworkOnlyConnector({
//   providerURL: "https://mainnet.infura.io/v3/...",
// });

const connectors = { MetaMask };

export default connectors;
