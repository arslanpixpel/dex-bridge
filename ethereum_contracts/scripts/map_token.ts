import { ethers, run, web3, network } from "hardhat";

(async () => {
  const RootChainManagerFactory = await ethers.getContractFactory(
    "RootChainManager"
  );
  const rootManager = RootChainManagerFactory.attach(
    "0x8469d2a9910251d990bCdb73fF61C4fBfBdBc8Ce"
  );
  const provider = new ethers.providers.JsonRpcProvider(
    "https://evm.testnet.kava.io"
  );
  const signer = new ethers.Wallet(
    "55a592ab5a6eafd3d8f52b18534d62b38f2b7f42dc8b7a40375188d5e0c0fe44",
    provider
  );
  rootManager.connect(signer);
  //   const res = await (
  //     await rootManager.grantRole(
  //       "0x71e7a05257c6e1a8458eede759a6478098f83525a01219cdc0e74ef32e36a773",
  //       "0x801F0Ac608f9CE61a5A83DF032F6495598BcEBAf"
  //     )
  //   ).wait();
  const res = await (
    await rootManager.remapToken(
      "EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",
      4603,
      0,
      "0xa234e09165f88967a714e2a476288e4c6d88b4b69fe7c300a03190b858990bfc"
    )
  ).wait();
  console.log(res);
})();
