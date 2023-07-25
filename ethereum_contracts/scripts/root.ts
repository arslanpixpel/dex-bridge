import { ethers, run, web3, network } from "hardhat";

(async () => {
  const RootChainManagerFactory = await ethers.getContractFactory(
    "RootChainManager"
  );
  const rootManager = RootChainManagerFactory.attach(
    "0x8469d2a9910251d990bCdb73fF61C4fBfBdBc8Ce"
  );
  await run("verify:verify", {
    contract: "contracts/root/RootChainManagerProxy.sol:RootChainManagerProxy",
    address: "0x8469d2a9910251d990bCdb73fF61C4fBfBdBc8Ce",
    constructorArguments: [
      "0x841cb0aba4e3d6cf2e106c84b9d8742b0fe9a97b",
      "0x801f0ac608f9ce61a5a83df032f6495598bcebaf",
      rootManager.interface.encodeFunctionData("initialize", [
        "0x801f0ac608f9ce61a5a83df032f6495598bcebaf",
      ]),
    ],
  });
})();
