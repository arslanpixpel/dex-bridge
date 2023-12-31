/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import {
  BaseContract,
  BigNumber,
  BigNumberish,
  BytesLike,
  CallOverrides,
  ContractTransaction,
  Overrides,
  PayableOverrides,
  PopulatedTransaction,
  Signer,
  utils,
} from "ethers";
import { FunctionFragment, Result, EventFragment } from "@ethersproject/abi";
import { Listener, Provider } from "@ethersproject/providers";
import { TypedEventFilter, TypedEvent, TypedListener, OnEvent } from "./common";

export declare namespace IRootChainManager {
  export type WithdrawParamsStruct = {
    ccdIndex: BigNumberish;
    ccdSubIndex: BigNumberish;
    amount: BigNumberish;
    userWallet: string;
    ccdTxHash: BytesLike;
    ccdEventIndex: BigNumberish;
    tokenId: BigNumberish;
  };

  export type WithdrawParamsStructOutput = [
    BigNumber,
    BigNumber,
    BigNumber,
    string,
    string,
    BigNumber,
    BigNumber
  ] & {
    ccdIndex: BigNumber;
    ccdSubIndex: BigNumber;
    amount: BigNumber;
    userWallet: string;
    ccdTxHash: string;
    ccdEventIndex: BigNumber;
    tokenId: BigNumber;
  };
}

export interface RootChainManagerInterface extends utils.Interface {
  contractName: "RootChainManager";
  functions: {
    "DEFAULT_ADMIN_ROLE()": FunctionFragment;
    "ETHER_ADDRESS()": FunctionFragment;
    "MAPPER_ROLE()": FunctionFragment;
    "MERKLE_UPDATER()": FunctionFragment;
    "childToRootToken(bytes32)": FunctionFragment;
    "cleanMapToken(address,uint64,uint64)": FunctionFragment;
    "depositEtherFor(address,bytes32)": FunctionFragment;
    "depositFee()": FunctionFragment;
    "depositFor(address,bytes32,address,bytes)": FunctionFragment;
    "getMerkleRoot()": FunctionFragment;
    "getRoleAdmin(bytes32)": FunctionFragment;
    "grantRole(bytes32,address)": FunctionFragment;
    "hasRole(bytes32,address)": FunctionFragment;
    "hashChild(uint64,uint64)": FunctionFragment;
    "initialize(address)": FunctionFragment;
    "mapToken(address,uint64,uint64,bytes32)": FunctionFragment;
    "merkleRoot()": FunctionFragment;
    "paused()": FunctionFragment;
    "previousMerkleRoot()": FunctionFragment;
    "processedExits(bytes32)": FunctionFragment;
    "registerVault(bytes32,address)": FunctionFragment;
    "remapToken(address,uint64,uint64,bytes32)": FunctionFragment;
    "renounceRole(bytes32,address)": FunctionFragment;
    "revokeRole(bytes32,address)": FunctionFragment;
    "rootToChildToken(address)": FunctionFragment;
    "setDepositFee(uint256)": FunctionFragment;
    "setMerkleRoot(bytes32)": FunctionFragment;
    "setPaused(bool)": FunctionFragment;
    "setStateSender(address)": FunctionFragment;
    "setTreasurer(address)": FunctionFragment;
    "setWithdrawFee(uint256)": FunctionFragment;
    "stateSenderAddress()": FunctionFragment;
    "supportsInterface(bytes4)": FunctionFragment;
    "tokenToType(address)": FunctionFragment;
    "treasurer()": FunctionFragment;
    "typeToVault(bytes32)": FunctionFragment;
    "withdraw((uint64,uint64,uint256,address,bytes32,uint64,uint64),bytes32[])": FunctionFragment;
    "withdrawFee()": FunctionFragment;
  };

  encodeFunctionData(
    functionFragment: "DEFAULT_ADMIN_ROLE",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "ETHER_ADDRESS",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "MAPPER_ROLE",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "MERKLE_UPDATER",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "childToRootToken",
    values: [BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "cleanMapToken",
    values: [string, BigNumberish, BigNumberish]
  ): string;
  encodeFunctionData(
    functionFragment: "depositEtherFor",
    values: [string, BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "depositFee",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "depositFor",
    values: [string, BytesLike, string, BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "getMerkleRoot",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "getRoleAdmin",
    values: [BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "grantRole",
    values: [BytesLike, string]
  ): string;
  encodeFunctionData(
    functionFragment: "hasRole",
    values: [BytesLike, string]
  ): string;
  encodeFunctionData(
    functionFragment: "hashChild",
    values: [BigNumberish, BigNumberish]
  ): string;
  encodeFunctionData(functionFragment: "initialize", values: [string]): string;
  encodeFunctionData(
    functionFragment: "mapToken",
    values: [string, BigNumberish, BigNumberish, BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "merkleRoot",
    values?: undefined
  ): string;
  encodeFunctionData(functionFragment: "paused", values?: undefined): string;
  encodeFunctionData(
    functionFragment: "previousMerkleRoot",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "processedExits",
    values: [BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "registerVault",
    values: [BytesLike, string]
  ): string;
  encodeFunctionData(
    functionFragment: "remapToken",
    values: [string, BigNumberish, BigNumberish, BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "renounceRole",
    values: [BytesLike, string]
  ): string;
  encodeFunctionData(
    functionFragment: "revokeRole",
    values: [BytesLike, string]
  ): string;
  encodeFunctionData(
    functionFragment: "rootToChildToken",
    values: [string]
  ): string;
  encodeFunctionData(
    functionFragment: "setDepositFee",
    values: [BigNumberish]
  ): string;
  encodeFunctionData(
    functionFragment: "setMerkleRoot",
    values: [BytesLike]
  ): string;
  encodeFunctionData(functionFragment: "setPaused", values: [boolean]): string;
  encodeFunctionData(
    functionFragment: "setStateSender",
    values: [string]
  ): string;
  encodeFunctionData(
    functionFragment: "setTreasurer",
    values: [string]
  ): string;
  encodeFunctionData(
    functionFragment: "setWithdrawFee",
    values: [BigNumberish]
  ): string;
  encodeFunctionData(
    functionFragment: "stateSenderAddress",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "supportsInterface",
    values: [BytesLike]
  ): string;
  encodeFunctionData(functionFragment: "tokenToType", values: [string]): string;
  encodeFunctionData(functionFragment: "treasurer", values?: undefined): string;
  encodeFunctionData(
    functionFragment: "typeToVault",
    values: [BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "withdraw",
    values: [IRootChainManager.WithdrawParamsStruct, BytesLike[]]
  ): string;
  encodeFunctionData(
    functionFragment: "withdrawFee",
    values?: undefined
  ): string;

  decodeFunctionResult(
    functionFragment: "DEFAULT_ADMIN_ROLE",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "ETHER_ADDRESS",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "MAPPER_ROLE",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "MERKLE_UPDATER",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "childToRootToken",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "cleanMapToken",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "depositEtherFor",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "depositFee", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "depositFor", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "getMerkleRoot",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "getRoleAdmin",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "grantRole", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "hasRole", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "hashChild", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "initialize", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "mapToken", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "merkleRoot", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "paused", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "previousMerkleRoot",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "processedExits",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "registerVault",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "remapToken", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "renounceRole",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "revokeRole", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "rootToChildToken",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "setDepositFee",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "setMerkleRoot",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "setPaused", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "setStateSender",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "setTreasurer",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "setWithdrawFee",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "stateSenderAddress",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "supportsInterface",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "tokenToType",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "treasurer", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "typeToVault",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "withdraw", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "withdrawFee",
    data: BytesLike
  ): Result;

  events: {
    "Initialized(uint8)": EventFragment;
    "RoleAdminChanged(bytes32,bytes32,bytes32)": EventFragment;
    "RoleGranted(bytes32,address,address)": EventFragment;
    "RoleRevoked(bytes32,address,address)": EventFragment;
  };

  getEvent(nameOrSignatureOrTopic: "Initialized"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "RoleAdminChanged"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "RoleGranted"): EventFragment;
  getEvent(nameOrSignatureOrTopic: "RoleRevoked"): EventFragment;
}

export type InitializedEvent = TypedEvent<[number], { version: number }>;

export type InitializedEventFilter = TypedEventFilter<InitializedEvent>;

export type RoleAdminChangedEvent = TypedEvent<
  [string, string, string],
  { role: string; previousAdminRole: string; newAdminRole: string }
>;

export type RoleAdminChangedEventFilter =
  TypedEventFilter<RoleAdminChangedEvent>;

export type RoleGrantedEvent = TypedEvent<
  [string, string, string],
  { role: string; account: string; sender: string }
>;

export type RoleGrantedEventFilter = TypedEventFilter<RoleGrantedEvent>;

export type RoleRevokedEvent = TypedEvent<
  [string, string, string],
  { role: string; account: string; sender: string }
>;

export type RoleRevokedEventFilter = TypedEventFilter<RoleRevokedEvent>;

export interface RootChainManager extends BaseContract {
  contractName: "RootChainManager";
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  interface: RootChainManagerInterface;

  queryFilter<TEvent extends TypedEvent>(
    event: TypedEventFilter<TEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TEvent>>;

  listeners<TEvent extends TypedEvent>(
    eventFilter?: TypedEventFilter<TEvent>
  ): Array<TypedListener<TEvent>>;
  listeners(eventName?: string): Array<Listener>;
  removeAllListeners<TEvent extends TypedEvent>(
    eventFilter: TypedEventFilter<TEvent>
  ): this;
  removeAllListeners(eventName?: string): this;
  off: OnEvent<this>;
  on: OnEvent<this>;
  once: OnEvent<this>;
  removeListener: OnEvent<this>;

  functions: {
    DEFAULT_ADMIN_ROLE(overrides?: CallOverrides): Promise<[string]>;

    ETHER_ADDRESS(overrides?: CallOverrides): Promise<[string]>;

    MAPPER_ROLE(overrides?: CallOverrides): Promise<[string]>;

    MERKLE_UPDATER(overrides?: CallOverrides): Promise<[string]>;

    childToRootToken(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<[string]>;

    cleanMapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    depositEtherFor(
      user: string,
      ccdUser: BytesLike,
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    depositFee(overrides?: CallOverrides): Promise<[BigNumber]>;

    depositFor(
      user: string,
      ccdUser: BytesLike,
      rootToken: string,
      depositData: BytesLike,
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    getMerkleRoot(overrides?: CallOverrides): Promise<[string]>;

    getRoleAdmin(role: BytesLike, overrides?: CallOverrides): Promise<[string]>;

    grantRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    hasRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<[boolean]>;

    hashChild(
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string]>;

    initialize(
      _owner: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    mapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    merkleRoot(overrides?: CallOverrides): Promise<[string]>;

    paused(overrides?: CallOverrides): Promise<[boolean]>;

    previousMerkleRoot(overrides?: CallOverrides): Promise<[string]>;

    processedExits(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<[boolean]>;

    registerVault(
      tokenType: BytesLike,
      vaultAddress: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    remapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    renounceRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    revokeRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    rootToChildToken(
      arg0: string,
      overrides?: CallOverrides
    ): Promise<
      [BigNumber, BigNumber] & { index: BigNumber; subindex: BigNumber }
    >;

    setDepositFee(
      newDepositFee: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    setMerkleRoot(
      _merkleRoot: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    setPaused(
      _paused: boolean,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    setStateSender(
      newStateSender: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    setTreasurer(
      newTreasurer: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    setWithdrawFee(
      newWithdrawFee: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    stateSenderAddress(overrides?: CallOverrides): Promise<[string]>;

    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides
    ): Promise<[boolean]>;

    tokenToType(arg0: string, overrides?: CallOverrides): Promise<[string]>;

    treasurer(overrides?: CallOverrides): Promise<[string]>;

    typeToVault(arg0: BytesLike, overrides?: CallOverrides): Promise<[string]>;

    withdraw(
      withdrawParam: IRootChainManager.WithdrawParamsStruct,
      proof: BytesLike[],
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    withdrawFee(overrides?: CallOverrides): Promise<[BigNumber]>;
  };

  DEFAULT_ADMIN_ROLE(overrides?: CallOverrides): Promise<string>;

  ETHER_ADDRESS(overrides?: CallOverrides): Promise<string>;

  MAPPER_ROLE(overrides?: CallOverrides): Promise<string>;

  MERKLE_UPDATER(overrides?: CallOverrides): Promise<string>;

  childToRootToken(arg0: BytesLike, overrides?: CallOverrides): Promise<string>;

  cleanMapToken(
    rootToken: string,
    childTokenIndex: BigNumberish,
    childTokenSubIndex: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  depositEtherFor(
    user: string,
    ccdUser: BytesLike,
    overrides?: PayableOverrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  depositFee(overrides?: CallOverrides): Promise<BigNumber>;

  depositFor(
    user: string,
    ccdUser: BytesLike,
    rootToken: string,
    depositData: BytesLike,
    overrides?: PayableOverrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  getMerkleRoot(overrides?: CallOverrides): Promise<string>;

  getRoleAdmin(role: BytesLike, overrides?: CallOverrides): Promise<string>;

  grantRole(
    role: BytesLike,
    account: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  hasRole(
    role: BytesLike,
    account: string,
    overrides?: CallOverrides
  ): Promise<boolean>;

  hashChild(
    childTokenIndex: BigNumberish,
    childTokenSubIndex: BigNumberish,
    overrides?: CallOverrides
  ): Promise<string>;

  initialize(
    _owner: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  mapToken(
    rootToken: string,
    childTokenIndex: BigNumberish,
    childTokenSubIndex: BigNumberish,
    tokenType: BytesLike,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  merkleRoot(overrides?: CallOverrides): Promise<string>;

  paused(overrides?: CallOverrides): Promise<boolean>;

  previousMerkleRoot(overrides?: CallOverrides): Promise<string>;

  processedExits(arg0: BytesLike, overrides?: CallOverrides): Promise<boolean>;

  registerVault(
    tokenType: BytesLike,
    vaultAddress: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  remapToken(
    rootToken: string,
    childTokenIndex: BigNumberish,
    childTokenSubIndex: BigNumberish,
    tokenType: BytesLike,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  renounceRole(
    role: BytesLike,
    account: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  revokeRole(
    role: BytesLike,
    account: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  rootToChildToken(
    arg0: string,
    overrides?: CallOverrides
  ): Promise<
    [BigNumber, BigNumber] & { index: BigNumber; subindex: BigNumber }
  >;

  setDepositFee(
    newDepositFee: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  setMerkleRoot(
    _merkleRoot: BytesLike,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  setPaused(
    _paused: boolean,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  setStateSender(
    newStateSender: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  setTreasurer(
    newTreasurer: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  setWithdrawFee(
    newWithdrawFee: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  stateSenderAddress(overrides?: CallOverrides): Promise<string>;

  supportsInterface(
    interfaceId: BytesLike,
    overrides?: CallOverrides
  ): Promise<boolean>;

  tokenToType(arg0: string, overrides?: CallOverrides): Promise<string>;

  treasurer(overrides?: CallOverrides): Promise<string>;

  typeToVault(arg0: BytesLike, overrides?: CallOverrides): Promise<string>;

  withdraw(
    withdrawParam: IRootChainManager.WithdrawParamsStruct,
    proof: BytesLike[],
    overrides?: PayableOverrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  withdrawFee(overrides?: CallOverrides): Promise<BigNumber>;

  callStatic: {
    DEFAULT_ADMIN_ROLE(overrides?: CallOverrides): Promise<string>;

    ETHER_ADDRESS(overrides?: CallOverrides): Promise<string>;

    MAPPER_ROLE(overrides?: CallOverrides): Promise<string>;

    MERKLE_UPDATER(overrides?: CallOverrides): Promise<string>;

    childToRootToken(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<string>;

    cleanMapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: CallOverrides
    ): Promise<void>;

    depositEtherFor(
      user: string,
      ccdUser: BytesLike,
      overrides?: CallOverrides
    ): Promise<void>;

    depositFee(overrides?: CallOverrides): Promise<BigNumber>;

    depositFor(
      user: string,
      ccdUser: BytesLike,
      rootToken: string,
      depositData: BytesLike,
      overrides?: CallOverrides
    ): Promise<void>;

    getMerkleRoot(overrides?: CallOverrides): Promise<string>;

    getRoleAdmin(role: BytesLike, overrides?: CallOverrides): Promise<string>;

    grantRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<void>;

    hasRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<boolean>;

    hashChild(
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: CallOverrides
    ): Promise<string>;

    initialize(_owner: string, overrides?: CallOverrides): Promise<void>;

    mapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: CallOverrides
    ): Promise<void>;

    merkleRoot(overrides?: CallOverrides): Promise<string>;

    paused(overrides?: CallOverrides): Promise<boolean>;

    previousMerkleRoot(overrides?: CallOverrides): Promise<string>;

    processedExits(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<boolean>;

    registerVault(
      tokenType: BytesLike,
      vaultAddress: string,
      overrides?: CallOverrides
    ): Promise<void>;

    remapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: CallOverrides
    ): Promise<void>;

    renounceRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<void>;

    revokeRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<void>;

    rootToChildToken(
      arg0: string,
      overrides?: CallOverrides
    ): Promise<
      [BigNumber, BigNumber] & { index: BigNumber; subindex: BigNumber }
    >;

    setDepositFee(
      newDepositFee: BigNumberish,
      overrides?: CallOverrides
    ): Promise<void>;

    setMerkleRoot(
      _merkleRoot: BytesLike,
      overrides?: CallOverrides
    ): Promise<void>;

    setPaused(_paused: boolean, overrides?: CallOverrides): Promise<void>;

    setStateSender(
      newStateSender: string,
      overrides?: CallOverrides
    ): Promise<void>;

    setTreasurer(
      newTreasurer: string,
      overrides?: CallOverrides
    ): Promise<void>;

    setWithdrawFee(
      newWithdrawFee: BigNumberish,
      overrides?: CallOverrides
    ): Promise<void>;

    stateSenderAddress(overrides?: CallOverrides): Promise<string>;

    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides
    ): Promise<boolean>;

    tokenToType(arg0: string, overrides?: CallOverrides): Promise<string>;

    treasurer(overrides?: CallOverrides): Promise<string>;

    typeToVault(arg0: BytesLike, overrides?: CallOverrides): Promise<string>;

    withdraw(
      withdrawParam: IRootChainManager.WithdrawParamsStruct,
      proof: BytesLike[],
      overrides?: CallOverrides
    ): Promise<void>;

    withdrawFee(overrides?: CallOverrides): Promise<BigNumber>;
  };

  filters: {
    "Initialized(uint8)"(version?: null): InitializedEventFilter;
    Initialized(version?: null): InitializedEventFilter;

    "RoleAdminChanged(bytes32,bytes32,bytes32)"(
      role?: BytesLike | null,
      previousAdminRole?: BytesLike | null,
      newAdminRole?: BytesLike | null
    ): RoleAdminChangedEventFilter;
    RoleAdminChanged(
      role?: BytesLike | null,
      previousAdminRole?: BytesLike | null,
      newAdminRole?: BytesLike | null
    ): RoleAdminChangedEventFilter;

    "RoleGranted(bytes32,address,address)"(
      role?: BytesLike | null,
      account?: string | null,
      sender?: string | null
    ): RoleGrantedEventFilter;
    RoleGranted(
      role?: BytesLike | null,
      account?: string | null,
      sender?: string | null
    ): RoleGrantedEventFilter;

    "RoleRevoked(bytes32,address,address)"(
      role?: BytesLike | null,
      account?: string | null,
      sender?: string | null
    ): RoleRevokedEventFilter;
    RoleRevoked(
      role?: BytesLike | null,
      account?: string | null,
      sender?: string | null
    ): RoleRevokedEventFilter;
  };

  estimateGas: {
    DEFAULT_ADMIN_ROLE(overrides?: CallOverrides): Promise<BigNumber>;

    ETHER_ADDRESS(overrides?: CallOverrides): Promise<BigNumber>;

    MAPPER_ROLE(overrides?: CallOverrides): Promise<BigNumber>;

    MERKLE_UPDATER(overrides?: CallOverrides): Promise<BigNumber>;

    childToRootToken(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    cleanMapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    depositEtherFor(
      user: string,
      ccdUser: BytesLike,
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    depositFee(overrides?: CallOverrides): Promise<BigNumber>;

    depositFor(
      user: string,
      ccdUser: BytesLike,
      rootToken: string,
      depositData: BytesLike,
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    getMerkleRoot(overrides?: CallOverrides): Promise<BigNumber>;

    getRoleAdmin(
      role: BytesLike,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    grantRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    hasRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    hashChild(
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    initialize(
      _owner: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    mapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    merkleRoot(overrides?: CallOverrides): Promise<BigNumber>;

    paused(overrides?: CallOverrides): Promise<BigNumber>;

    previousMerkleRoot(overrides?: CallOverrides): Promise<BigNumber>;

    processedExits(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    registerVault(
      tokenType: BytesLike,
      vaultAddress: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    remapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    renounceRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    revokeRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    rootToChildToken(
      arg0: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    setDepositFee(
      newDepositFee: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    setMerkleRoot(
      _merkleRoot: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    setPaused(
      _paused: boolean,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    setStateSender(
      newStateSender: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    setTreasurer(
      newTreasurer: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    setWithdrawFee(
      newWithdrawFee: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    stateSenderAddress(overrides?: CallOverrides): Promise<BigNumber>;

    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    tokenToType(arg0: string, overrides?: CallOverrides): Promise<BigNumber>;

    treasurer(overrides?: CallOverrides): Promise<BigNumber>;

    typeToVault(arg0: BytesLike, overrides?: CallOverrides): Promise<BigNumber>;

    withdraw(
      withdrawParam: IRootChainManager.WithdrawParamsStruct,
      proof: BytesLike[],
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    withdrawFee(overrides?: CallOverrides): Promise<BigNumber>;
  };

  populateTransaction: {
    DEFAULT_ADMIN_ROLE(
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    ETHER_ADDRESS(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    MAPPER_ROLE(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    MERKLE_UPDATER(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    childToRootToken(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    cleanMapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    depositEtherFor(
      user: string,
      ccdUser: BytesLike,
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    depositFee(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    depositFor(
      user: string,
      ccdUser: BytesLike,
      rootToken: string,
      depositData: BytesLike,
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    getMerkleRoot(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    getRoleAdmin(
      role: BytesLike,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    grantRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    hasRole(
      role: BytesLike,
      account: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    hashChild(
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    initialize(
      _owner: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    mapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    merkleRoot(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    paused(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    previousMerkleRoot(
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    processedExits(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    registerVault(
      tokenType: BytesLike,
      vaultAddress: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    remapToken(
      rootToken: string,
      childTokenIndex: BigNumberish,
      childTokenSubIndex: BigNumberish,
      tokenType: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    renounceRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    revokeRole(
      role: BytesLike,
      account: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    rootToChildToken(
      arg0: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    setDepositFee(
      newDepositFee: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    setMerkleRoot(
      _merkleRoot: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    setPaused(
      _paused: boolean,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    setStateSender(
      newStateSender: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    setTreasurer(
      newTreasurer: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    setWithdrawFee(
      newWithdrawFee: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    stateSenderAddress(
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    supportsInterface(
      interfaceId: BytesLike,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    tokenToType(
      arg0: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    treasurer(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    typeToVault(
      arg0: BytesLike,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    withdraw(
      withdrawParam: IRootChainManager.WithdrawParamsStruct,
      proof: BytesLike[],
      overrides?: PayableOverrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    withdrawFee(overrides?: CallOverrides): Promise<PopulatedTransaction>;
  };
}
