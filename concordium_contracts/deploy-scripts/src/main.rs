pub mod contracts;
pub mod deployer;

use crate::contracts::{BridgeRoles, CIS2BridgeableRoles};
use anyhow::Context;
use clap::Parser;
use concordium_rust_sdk::{
    endpoints::{self, RPCError},
    smart_contracts::common::{Address, NewContractNameError, NewReceiveNameError},
    types::{
        hashes::TransactionHash,
        smart_contracts::{ExceedsParameterSize, ModuleRef, WasmModule},
        ContractAddress,
    },
    v2,
};
use deployer::{Deployer, ModuleDeployed};
use hex::FromHexError;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Deserialize, Clone, Debug)]
pub struct WrappedToken {
    pub name: String,
    pub token_metadata_url: String,
    pub token_metadata_hash: Option<TransactionHash>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub bridge_manager: ContractAddress,
    pub tokens: Vec<OutputToken>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputToken {
    pub name: String,
    pub token_url: String,
    pub contract: ContractAddress,
}

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("concordium error: {0}")]
    RPCError(#[from] RPCError),
    #[error("transport error: {0}")]
    TransportError(#[from] v2::Error),
    #[error("query error: {0}")]
    QueryError(#[from] endpoints::QueryError),
    #[error("anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("There are unfinalized transactions. Transaction nonce is not reliable enough.")]
    NonceNotFinal,
    #[error("Transaction rejected: {0}")]
    TransactionRejected(RPCError),
    #[error("Transaction rejected: {0:?}")]
    TransactionRejectedR(String),
    #[error("Invalid block item: {0}")]
    InvalidBlockItem(String),
    #[error("Invalid contract name: {0}")]
    InvalidContractName(String),
    #[error("hex decoding error: {0}")]
    HexDecodingError(#[from] FromHexError),
    #[error("failed to parse receive name: {0}")]
    FailedToParseReceiveName(String),
    #[error("Json error: {0}")]
    JSONError(#[from] serde_json::Error),
    #[error("Parameter size error: {0}")]
    ParameterSizeError(#[from] ExceedsParameterSize),
    #[error("Receive name error: {0}")]
    ReceiveNameError(#[from] NewReceiveNameError),
    #[error("Contract name error: {0}")]
    ContractNameError(#[from] NewContractNameError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid metadata hash: {0}")]
    InvalidHash(String),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Invoke contract failed: {0}")]
    InvokeContractFailed(String),
}

#[allow(dead_code)]
fn module_deployed(module_ref: &str) -> Result<ModuleDeployed, DeployError> {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(module_ref, &mut bytes)?;

    let module_deployed = ModuleDeployed {
        module_ref: ModuleRef::from(bytes),
    };

    Ok(module_deployed)
}

fn get_wasm_module(file: &Path) -> Result<WasmModule, DeployError> {
    let wasm_module =
        std::fs::read(file).context("Could not read the cis2_bridgeable.wasm.v1 file")?;
    let mut cursor = Cursor::new(wasm_module);
    let wasm_module: WasmModule = concordium_rust_sdk::common::from_bytes(&mut cursor)?;
    Ok(wasm_module)
}

async fn deploy_token(
    deployer: &Deployer,
    token: &WrappedToken,
    cis2_bridgeable_module_ref: ModuleRef,
    bridge_manager: ContractAddress,
) -> Result<OutputToken, DeployError> {
    println!("Initializing cis2-bridgeable {}....", token.name);

    let mock_token = deployer
        .init_token_contract(
            token.name.clone(),
            token.token_metadata_url.clone(),
            // token.token_metadata_hash.as_ref().try_into().unwrap(),
            cis2_bridgeable_module_ref,
        )
        .await?;
    println!(
        "Initialized cis2-bridgeable {} at address: ({}, {})",
        token.name, mock_token.index, mock_token.subindex
    );
    println!(
        "Granting bridge-manager Manager role on {} token....",
        token.name
    );
    deployer
        .token_grant_role(
            mock_token,
            Address::Contract(bridge_manager),
            CIS2BridgeableRoles::Manager,
        )
        .await?;
    println!(
        "Granted bridge-manager Manager role on {} token",
        token.name.clone()
    );

    let token = OutputToken {
        name: token.name.clone(),
        token_url: token.token_metadata_url.clone(),
        contract: mock_token,
    };

    Ok(token)
}

async fn init_contracts(
    deployer: &Deployer,
    tokens: &[WrappedToken],
    bridge_manager_module_ref: ModuleRef,
    cis2_bridgeable_module_ref: ModuleRef,
) -> Result<(), DeployError> {
    println!("Initializing bridge-manager....");
    let bridge_manager = deployer
        .init_bridge_contract(bridge_manager_module_ref)
        .await?;
    println!(
        "Initialized bridge-manager, address: ({}, {})",
        bridge_manager.index, bridge_manager.subindex
    );
    println!("Granting Manager address Manager role on bridge-manager....");
    deployer
        .bridge_grant_role(bridge_manager, BridgeRoles::StateSyncer)
        .await?;
    println!("Granted Manager address Manager role on bridge-manager");

    println!();

    let mut output = Output {
        bridge_manager,
        tokens: vec![],
    };

    for token in tokens {
        let output_token =
            deploy_token(deployer, token, cis2_bridgeable_module_ref, bridge_manager).await?;
        output.tokens.push(output_token);

        println!();
    }

    println!();

    let json = serde_json::to_string_pretty(&output)?;

    println!("{json}");

    let file = File::create("../latest.json")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(json.as_bytes())?;

    Ok(())
}

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
struct DeployScripts {
    #[clap(
        long = "node",
        default_value = "http://localhost:20001",
        help = "V2 API of the concordium node."
    )]
    concordium_url: v2::Endpoint,
    #[clap(long = "wallet", help = "Location of the Concordium wallet.")]
    concordium_wallet: PathBuf,
    #[clap(long = "tokens", help = "JSON file with a list of tokens.")]
    tokens: PathBuf,
    #[clap(
        long = "manager-source",
        help = "Location of the compiled BridgeManager contract."
    )]
    manager_source: PathBuf,
    #[clap(long = "cis2-bridgeable", help = "Source of the CIS2 token contract.")]
    cis2_source: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), DeployError> {
    let app: DeployScripts = DeployScripts::parse();

    let concordium_client = v2::Client::new(app.concordium_url).await?;

    let deployer = Deployer::new(concordium_client, &app.concordium_wallet)?;

    let tokens: Vec<WrappedToken> = serde_json::from_slice(&std::fs::read(app.tokens)?)?;

    // make sure we didn't mess up the hashes:
    // for token in tokens.iter() {
    //     let _ =
    //         get_and_compare_metadata_hash(&token.token_metadata_url, &token.token_metadata_hash)
    //             .await?;
    // }

    let wasm_module = get_wasm_module(app.cis2_source.as_path())?;
    println!("Deploying cis2-bridgeable....");
    let cis2_bridgeable_module_ref = deployer.deploy_wasm_module(wasm_module).await?;
    println!(
        "Deployed cis2-bridgeable, module_ref: {}",
        cis2_bridgeable_module_ref.module_ref
    );

    println!();

    let wasm_module = get_wasm_module(app.manager_source.as_path())?;
    println!("Deploying bridge-manager....");
    let bridge_manager_module_ref = deployer.deploy_wasm_module(wasm_module).await?;
    println!(
        "Deployed bridge-manager, module_ref: {}",
        bridge_manager_module_ref.module_ref
    );

    println!();

    init_contracts(
        &deployer,
        &tokens,
        bridge_manager_module_ref.module_ref,
        cis2_bridgeable_module_ref.module_ref,
    )
    .await?;

    Ok(())
}
