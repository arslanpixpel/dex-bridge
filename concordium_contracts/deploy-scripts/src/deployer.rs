use concordium_rust_sdk::{
    common::types::{Amount, TransactionTime},
    id::types::AccountAddress,
    smart_contracts::common::{
        self as contracts_common, Address, ModuleReference, OwnedContractName, OwnedReceiveName,
    },
    types::{
        queries::AccountNonceResponse,
        smart_contracts::{
            ContractContext, InvokeContractResult, ModuleRef, Parameter, WasmModule,
        },
        transactions::{
            self,
            send::{deploy_module, init_contract, GivenEnergy},
            InitContractPayload, UpdateContractPayload,
        },
        AccountTransactionEffects, BlockItemSummary, BlockItemSummaryDetails, ContractAddress,
        Energy, RejectReason, TransactionType, WalletAccount,
    },
    v2,
};
use std::{path::Path, str::FromStr};

use crate::{
    contracts::{
        BridgeGrantRoleParams, BridgeRoles, CIS2BridgeableGrantRoleParams,
        CIS2BridgeableInitParams, CIS2BridgeableRoles, UpgradeParams,
    },
    DeployError,
};

const BRIDGE_GRANT_ROLE_METHOD: &str = "bridge-manager.grantRole";
const BRIDGE_INIT_METHOD: &str = "init_bridge-manager";
pub const BRIDGE_UPGRADE_METHOD: &str = "bridge-manager.upgrade";
pub const CIS2_UPGRADE_METHOD: &str = "cis2-bridgeable.upgrade";
const CIS2_GRANT_ROLE_METHOD: &str = "cis2-bridgeable.grantRole";
const CIS2_INIT_METHOD: &str = "init_cis2-bridgeable";

#[derive(Clone, Debug)]
pub struct ModuleDeployed {
    pub module_ref: ModuleRef,
}

#[derive(Clone, Debug)]
pub struct ContractInitialized {
    pub contract: ContractAddress,
}

#[derive(Debug)]
pub struct Deployer {
    pub client: v2::Client,
    pub manager_key: WalletAccount,
}

impl Deployer {
    pub fn new(client: v2::Client, wallet_account_file: &Path) -> Result<Deployer, DeployError> {
        let key_data = WalletAccount::from_json_file(wallet_account_file)?;

        Ok(Deployer {
            client,
            manager_key: key_data,
        })
    }

    pub async fn module_exists(&self, wasm_module: WasmModule) -> Result<bool, DeployError> {
        let consensus_info = self.client.clone().get_consensus_info().await?;

        let latest_block = consensus_info.last_finalized_block;

        let module_ref = wasm_module.get_module_ref();

        let module_ref = self
            .client
            .clone()
            .get_module_source(&module_ref, &latest_block)
            .await;
        match module_ref {
            Ok(_) => Ok(true),
            Err(e) if e.is_not_found() => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn deploy_wasm_module(
        &self,
        wasm_module: WasmModule,
    ) -> Result<ModuleDeployed, DeployError> {
        let exists = self.module_exists(wasm_module.clone()).await?;
        if exists {
            println!(
                "Module with reference {} already exists.",
                wasm_module.get_module_ref()
            );
            return Ok(ModuleDeployed {
                module_ref: wasm_module.get_module_ref(),
            });
        }

        let nonce = self.get_nonce(self.manager_key.address).await?;

        if !nonce.all_final {
            return Err(DeployError::NonceNotFinal);
        }

        let expiry = TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 300) as u64);
        let tx = deploy_module(
            &self.manager_key,
            self.manager_key.address,
            nonce.nonce,
            expiry,
            wasm_module,
        );
        let bi = transactions::BlockItem::AccountTransaction(tx);

        let tx_hash = self
            .client
            .clone()
            .send_block_item(&bi)
            .await
            .map_err(DeployError::TransactionRejected)?;

        let (_, block_item) = self.client.clone().wait_until_finalized(&tx_hash).await?;

        let module_deployed = self.parse_deploy_module_event(block_item)?;

        println!(
            "Transaction finalized, tx_hash={} module_ref={}",
            tx_hash, module_deployed.module_ref,
        );

        Ok(module_deployed)
    }

    pub async fn init_token_contract(
        &self,
        _name: String,
        metadata: String,
        // metadata_hash: [u8; 32],
        module_ref: ModuleRef,
    ) -> Result<ContractAddress, DeployError> {
        let parameters = CIS2BridgeableInitParams {
            url: metadata,
            // hash: Some(metadata_hash),
            hash: None,
        };
        let bytes = contracts_common::to_bytes(&parameters);
        let param: Parameter = bytes.try_into()?;

        let payload = InitContractPayload {
            init_name: OwnedContractName::new(CIS2_INIT_METHOD.into())?,
            amount: Amount::from_micro_ccd(0),
            mod_ref: module_ref,
            param,
        };

        let contract = self.init_contract(payload).await?;

        Ok(contract)
    }

    pub async fn init_bridge_contract(
        &self,
        module_ref: ModuleRef,
    ) -> Result<ContractAddress, DeployError> {
        let param: Parameter = Parameter::default();

        let payload = InitContractPayload {
            init_name: OwnedContractName::new(BRIDGE_INIT_METHOD.into())?,
            amount: Amount::from_micro_ccd(0),
            mod_ref: module_ref,
            param,
        };

        let contract = self.init_contract(payload).await?;

        Ok(contract)
    }

    pub async fn upgrade_contract(
        &self,
        new_module_ref: ModuleRef,
        contract: ContractAddress,
        method: &str,
    ) -> Result<(), DeployError> {
        let consensus_info = self.client.clone().get_consensus_info().await?;

        let latest_block = consensus_info.last_finalized_block;

        let state = self
            .client
            .clone()
            .get_instance_info(contract, &latest_block)
            .await?;

        let current_module_ref = state.response.source_module();

        if current_module_ref == new_module_ref {
            println!("Contract already uses the new module.");
            return Ok(());
        }

        let nonce = self.get_nonce(self.manager_key.address).await?;

        if !nonce.all_final {
            return Err(DeployError::NonceNotFinal);
        }

        let slice: &[u8] = new_module_ref.as_ref();
        let slice: [u8; 32] = slice.try_into().unwrap();
        let modref: ModuleReference = ModuleReference::from(slice);

        let params = UpgradeParams {
            module: modref,
            migrate: None,
        };
        let bytes = contracts_common::to_bytes(&params);

        let update_payload = transactions::UpdateContractPayload {
            amount: Amount::from_ccd(0),
            address: contract,
            receive_name: OwnedReceiveName::from_str(method)?,
            message: bytes.try_into()?,
        };

        let energy = self.estimate_energy(update_payload.clone()).await?;

        self.update_contract(update_payload, GivenEnergy::Add(energy))
            .await?;

        Ok(())
    }

    pub async fn init_contract(
        &self,
        payload: InitContractPayload,
    ) -> Result<ContractAddress, DeployError> {
        let nonce = self.get_nonce(self.manager_key.address).await?;

        if !nonce.all_final {
            return Err(DeployError::NonceNotFinal);
        }

        let expiry = TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 300) as u64);
        let energy = Energy { energy: 5000 };

        let tx = init_contract(
            &self.manager_key,
            self.manager_key.address,
            nonce.nonce,
            expiry,
            payload,
            energy,
        );

        let bi = transactions::BlockItem::AccountTransaction(tx);

        let tx_hash = self
            .client
            .clone()
            .send_block_item(&bi)
            .await
            .map_err(DeployError::TransactionRejected)?;
        println!("Sent tx: {tx_hash}");

        let (_, block_item) = self.client.clone().wait_until_finalized(&tx_hash).await?;

        let contract_init = self.parse_contract_init_event(block_item)?;

        println!(
            "Transaction finalized, tx_hash={} contract=({}, {})",
            tx_hash, contract_init.contract.index, contract_init.contract.subindex,
        );

        Ok(contract_init.contract)
    }

    pub async fn bridge_grant_role(
        &self,
        contract: ContractAddress,
        role: BridgeRoles,
    ) -> Result<(), DeployError> {
        let address = Address::Account(self.manager_key.address);

        let params = BridgeGrantRoleParams { address, role };
        let bytes = contracts_common::to_bytes(&params);

        let update_payload = transactions::UpdateContractPayload {
            amount: Amount::from_ccd(0),
            address: contract,
            receive_name: OwnedReceiveName::from_str(BRIDGE_GRANT_ROLE_METHOD)?,
            message: bytes.try_into()?,
        };

        let energy = self.estimate_energy(update_payload.clone()).await?;

        self.update_contract(update_payload, GivenEnergy::Add(energy))
            .await?;

        Ok(())
    }

    pub async fn token_grant_role(
        &self,
        contract: ContractAddress,
        address: Address,
        role: CIS2BridgeableRoles,
    ) -> Result<(), DeployError> {
        let params = CIS2BridgeableGrantRoleParams { address, role };
        let bytes = contracts_common::to_bytes(&params);

        let update_payload = transactions::UpdateContractPayload {
            amount: Amount::from_ccd(0),
            address: contract,
            receive_name: OwnedReceiveName::from_str(CIS2_GRANT_ROLE_METHOD)?,
            message: bytes.try_into()?,
        };

        let energy = self.estimate_energy(update_payload.clone()).await?;

        self.update_contract(update_payload, GivenEnergy::Add(energy))
            .await?;

        Ok(())
    }

    pub async fn update_contract(
        &self,
        update_payload: UpdateContractPayload,
        energy: GivenEnergy,
    ) -> Result<(), DeployError> {
        let nonce = self.get_nonce(self.manager_key.address).await?;

        if !nonce.all_final {
            return Err(DeployError::NonceNotFinal);
        }

        let payload = transactions::Payload::Update {
            payload: update_payload,
        };

        let expiry = TransactionTime::from_seconds((chrono::Utc::now().timestamp() + 300) as u64);

        let tx = transactions::send::make_and_sign_transaction(
            &self.manager_key,
            self.manager_key.address,
            nonce.nonce,
            expiry,
            energy,
            payload,
        );
        let bi = transactions::BlockItem::AccountTransaction(tx);

        let tx_hash = self
            .client
            .clone()
            .send_block_item(&bi)
            .await
            .map_err(DeployError::TransactionRejected)?;
        println!("Sent tx: {tx_hash}");

        let (_, block_item) = self.client.clone().wait_until_finalized(&tx_hash).await?;

        self.parse_contract_update_event(block_item)?;

        Ok(())
    }

    async fn estimate_energy(&self, payload: UpdateContractPayload) -> Result<Energy, DeployError> {
        let consensus_info = self.client.clone().get_consensus_info().await?;

        let context = ContractContext {
            invoker: Some(Address::Account(self.manager_key.address)),
            contract: payload.address,
            amount: payload.amount,
            method: payload.receive_name,
            parameter: payload.message,
            energy: 100000.into(),
        };

        let result = self
            .client
            .clone()
            .invoke_instance(&consensus_info.best_block, &context)
            .await?;

        match result.response {
            InvokeContractResult::Failure {
                return_value,
                reason,
                used_energy,
            } => Err(DeployError::InvokeContractFailed(format!(
                "contract invoke failed: {reason:?}, used_energy={used_energy}, return \
                 value={return_value:?}"
            ))),
            InvokeContractResult::Success {
                return_value: _,
                events: _,
                used_energy,
            } => {
                let e = used_energy.energy;
                println!("Estimated energy: {e}");
                Ok(Energy { energy: e + 100 })
            }
        }
    }

    pub async fn get_nonce(
        &self,
        address: AccountAddress,
    ) -> Result<AccountNonceResponse, DeployError> {
        let nonce = self
            .client
            .clone()
            .get_next_account_sequence_number(&address)
            .await?;
        Ok(nonce)
    }

    fn parse_deploy_module_event(
        &self,
        block_item: BlockItemSummary,
    ) -> Result<ModuleDeployed, DeployError> {
        match block_item.details {
            BlockItemSummaryDetails::AccountTransaction(a) => match a.effects {
                AccountTransactionEffects::None {
                    transaction_type,
                    reject_reason,
                } => {
                    if transaction_type != Some(TransactionType::DeployModule) {
                        return Err(DeployError::InvalidBlockItem(
                            "Expected transaction type to be DeployModule if rejected".into(),
                        ));
                    }

                    match reject_reason {
                        RejectReason::ModuleHashAlreadyExists { contents } => Ok(ModuleDeployed {
                            module_ref: contents,
                        }),
                        _ => Err(DeployError::TransactionRejectedR(format!(
                            "module deploy rejected with reason: {reject_reason:?}"
                        ))),
                    }
                }
                AccountTransactionEffects::ModuleDeployed { module_ref } => {
                    Ok(ModuleDeployed { module_ref })
                }
                _ => Err(DeployError::InvalidBlockItem(
                    "invalid transaction effects".into(),
                )),
            },
            _ => Err(DeployError::InvalidBlockItem(
                "Expected Account transaction".into(),
            )),
        }
    }

    fn parse_contract_init_event(
        &self,
        block_item: BlockItemSummary,
    ) -> Result<ContractInitialized, DeployError> {
        match block_item.details {
            BlockItemSummaryDetails::AccountTransaction(a) => match a.effects {
                AccountTransactionEffects::None {
                    transaction_type,
                    reject_reason,
                } => {
                    if transaction_type != Some(TransactionType::InitContract) {
                        return Err(DeployError::InvalidBlockItem(
                            "Expected transaction type to be InitContract if rejected".into(),
                        ));
                    }

                    return Err(DeployError::TransactionRejectedR(format!(
                        "contract init rejected with reason: {reject_reason:?}"
                    )));
                }
                AccountTransactionEffects::ContractInitialized { data } => {
                    Ok(ContractInitialized {
                        contract: data.address,
                    })
                }
                _ => Err(DeployError::InvalidBlockItem(
                    "invalid transaction effects".into(),
                )),
            },
            _ => Err(DeployError::InvalidBlockItem(
                "Expected Account transaction".into(),
            )),
        }
    }

    fn parse_contract_update_event(&self, block_item: BlockItemSummary) -> Result<(), DeployError> {
        match block_item.details {
            BlockItemSummaryDetails::AccountTransaction(a) => match a.effects {
                AccountTransactionEffects::None {
                    transaction_type,
                    reject_reason,
                } => {
                    if transaction_type != Some(TransactionType::Update) {
                        return Err(DeployError::InvalidBlockItem(
                            "Expected transaction type to be Update if rejected".into(),
                        ));
                    }

                    return Err(DeployError::TransactionRejectedR(format!(
                        "contract update rejected with reason: {reject_reason:?}"
                    )));
                }
                AccountTransactionEffects::ContractUpdateIssued { effects: _ } => Ok(()),
                _ => Err(DeployError::InvalidBlockItem(
                    "invalid transaction effects".into(),
                )),
            },
            _ => Err(DeployError::InvalidBlockItem(
                "Expected Account transaction".into(),
            )),
        }
    }
}
