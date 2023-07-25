#![cfg_attr(not(feature = "std"), no_std)]
use core::ops::Deref;

use concordium_cis2::*;
use concordium_std::*;

/// Contract token amount type.
type ContractTokenAmount = TokenAmountU256;

/// The different errors the contract can produce.
#[derive(Serialize, Debug, PartialEq, Eq, Reject, SchemaType)]
enum CustomContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Failed logging: Log is full.
    LogFull,
    /// Failed logging: Log is malformed.
    LogMalformed,
    /// Contract is paused.
    ContractPaused,
    /// Failed to invoke a contract.
    InvokeContractError,
    /// Failed to invoke a transfer.
    InvokeTransferError,
    // Token not mapped
    TokenNotMapped,
    // Role not assigned
    RoleNotAssigned,
    // Withdraw fee is too low (smaller than required fee)
    WithdrawFeeTooLow,
    // Operation already processed
    OperationAlreadyProcessed,
    /// Upgrade failed because the new module does not exist.
    FailedUpgradeMissingModule,
    /// Upgrade failed because the new module does not contain a contract with a
    /// matching name.
    FailedUpgradeMissingContract,
    /// Upgrade failed because the smart contract version of the module is not
    /// supported.
    FailedUpgradeUnsupportedModuleVersion,
    /// We only support withdraws from accounts, not contracts
    OnlyAccountsCanWithdraw,
}

type ContractError = Cis2Error<CustomContractError>;

type ContractResult<A> = Result<A, ContractError>;

/// Mapping errors related to contract upgrades to CustomContractError.
impl From<UpgradeError> for CustomContractError {
    #[inline(always)]
    fn from(ue: UpgradeError) -> Self {
        match ue {
            UpgradeError::MissingModule => Self::FailedUpgradeMissingModule,
            UpgradeError::MissingContract => Self::FailedUpgradeMissingContract,
            UpgradeError::UnsupportedModuleVersion => Self::FailedUpgradeUnsupportedModuleVersion,
        }
    }
}

/// Mapping the logging errors to ContractError.
impl From<LogError> for CustomContractError {
    fn from(le: LogError) -> Self {
        match le {
            LogError::Full => Self::LogFull,
            LogError::Malformed => Self::LogMalformed,
        }
    }
}

/// Mapping errors related to contract invocations to CustomContractError.
impl<T> From<CallContractError<T>> for CustomContractError {
    fn from(_cce: CallContractError<T>) -> Self {
        Self::InvokeContractError
    }
}

/// Mapping errors related to contract invocations to CustomContractError.
impl From<TransferError> for CustomContractError {
    fn from(_te: TransferError) -> Self {
        Self::InvokeTransferError
    }
}

/// Mapping CustomContractError to ContractError
impl From<CustomContractError> for ContractError {
    fn from(c: CustomContractError) -> Self {
        Cis2Error::Custom(c)
    }
}

#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
struct AddressRoleState<S> {
    roles: StateSet<Roles, S>,
}

/// The contract state,
#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S> {
    /// Contract is paused if `paused = true` and unpaused if `paused = false`.
    paused: bool,
    roles: StateMap<Address, AddressRoleState<S>, S>,
    root_mapping: StateMap<EthAddress, ContractAddress, S>,
    child_mapping: StateMap<ContractAddress, EthAddress, S>,
    emit_event_index: u64,
    withdraw_fee: Amount,
    treasurer_address: AccountAddress,
    processed_operations: StateSet<u64, S>,
}

/// View function to check if an event index has been processed.
#[receive(
    contract = "bridge-manager",
    name = "isProcessed",
    parameter = "u64",
    return_value = "bool"
)]
fn contract_is_processed<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<bool> {
    let index = ctx.parameter_cursor().get()?;

    Ok(host.state().has_operation(index))
}

/// Part of the return parameter of the `viewRoles` function.
#[derive(Serialize, SchemaType, PartialEq)]
struct ViewRolesState {
    /// Vector of roles.
    roles: Vec<Roles>,
}

/// Return parameter of the `viewRoles` function.
#[derive(Serialize, SchemaType)]
struct ViewAllRolesState {
    /// Vector specifiying for each address a vector of its associated roles.
    all_roles: Vec<(Address, ViewRolesState)>,
}

/// View function that returns the entire `roles` content of the state. Meant
/// for monitoring.
#[receive(
    contract = "bridge-manager",
    name = "viewRoles",
    return_value = "ViewAllRolesState"
)]
fn contract_view_roles<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ViewAllRolesState> {
    let state = host.state();

    let mut all_roles = Vec::new();
    for (address, a_state) in state.roles.iter() {
        let roles: Vec<Roles> = a_state.roles.iter().map(|x| *x).collect();

        all_roles.push((*address, ViewRolesState { roles }));
    }

    Ok(ViewAllRolesState { all_roles })
}

/// Return parameter of the `viewTokenMappings` function.
#[derive(Serialize, SchemaType)]
struct ViewTokenMappings {
    /// Token mappings from ethereum address to concordium contract address.
    root_mappings: Vec<(EthAddress, ContractAddress)>,
    /// Token mappings from concordium contract address to ethereum address.
    child_mappings: Vec<(ContractAddress, EthAddress)>,
}

/// View function that returns the entire `tokenMappings` content of the state.
/// Meant for monitoring.
#[receive(
    contract = "bridge-manager",
    name = "viewTokenMappings",
    return_value = "ViewTokenMappings"
)]
fn contract_view_token_mappings<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ViewTokenMappings> {
    let state = host.state();

    let mut root_mappings = Vec::new();
    for (eth_address, contract_address) in state.root_mapping.iter() {
        root_mappings.push((*eth_address, *contract_address));
    }

    let mut child_mappings = Vec::new();
    for (contract_address, eth_address) in state.child_mapping.iter() {
        child_mappings.push((*contract_address, *eth_address));
    }

    Ok(ViewTokenMappings {
        root_mappings,
        child_mappings,
    })
}

/// Return parameter of the `viewConfiguration` function.
#[derive(Serialize, SchemaType, PartialEq)]
struct ViewConfigurationState {
    /// Contract is paused if `paused = true` and unpaused if `paused = false`.
    paused: bool,
    /// The current event index that has been logged in the last withdraw event.
    emit_event_index: u64,
    /// The fee to be paid when initiating a withdrawal of tokens.
    withdraw_fee: Amount,
    /// The address of the treasury receiving the above fees.
    treasurer_address: AccountAddress,
}

/// View function that returns configuration values of the state. Meant for
/// monitoring.
#[receive(
    contract = "bridge-manager",
    name = "viewConfiguration",
    return_value = "ViewConfigurationState"
)]
fn contract_view_configuration<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ViewConfigurationState> {
    let state = host.state();

    Ok(ViewConfigurationState {
        paused: state.paused,
        emit_event_index: state.emit_event_index,
        withdraw_fee: state.withdraw_fee,
        treasurer_address: state.treasurer_address,
    })
}

#[derive(Serialize, PartialEq, Eq, Reject, SchemaType, Clone, Copy)]
pub enum Roles {
    Admin,
    Mapper,
    StateSyncer,
}

/// Manual implementation of the `EthAddressSchema` to display it in hex.
impl schema::SchemaType for EthAddress {
    fn get_type() -> schema::Type {
        schema::Type::ByteArray(20)
    }
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct EthAddress {
    pub eth_address: [u8; 20],
}

#[derive(Serialize, SchemaType)]
pub struct DepositOperation {
    pub id: u64,
    pub user: Address,
    pub root: EthAddress,
    pub amount: ContractTokenAmount,
    pub token_id: TokenIdU64,
}

#[derive(Serialize, SchemaType)]
pub struct TokenMapOperation {
    pub id: u64,
    pub root: EthAddress,
    pub child: ContractAddress,
}
#[derive(Serialize, SchemaType)]
pub enum StateUpdate {
    Deposit(DepositOperation),
    TokenMap(TokenMapOperation),
}

impl<S: HasStateApi> State<S> {
    /// Creates a new state with no one owning any tokens by default.
    fn new(state_builder: &mut StateBuilder<S>, treasurer: AccountAddress) -> Self {
        State {
            paused: false,
            roles: state_builder.new_map(),
            root_mapping: state_builder.new_map(),
            child_mapping: state_builder.new_map(),
            emit_event_index: 0u64,
            withdraw_fee: Amount::from_micro_ccd(0),
            treasurer_address: treasurer,
            processed_operations: state_builder.new_set(),
        }
    }

    fn has_role(&self, account: &Address, role: Roles) -> bool {
        return match self.roles.get(account) {
            None => false,
            Some(roles) => roles.roles.contains(&role),
        };
    }

    fn grant_role(&mut self, account: &Address, role: Roles, state_builder: &mut StateBuilder<S>) {
        self.roles
            .entry(*account)
            .or_insert_with(|| AddressRoleState {
                roles: state_builder.new_set(),
            });

        self.roles.entry(*account).and_modify(|entry| {
            entry.roles.insert(role);
        });
    }

    fn remove_role(&mut self, account: &Address, role: Roles) {
        self.roles.entry(*account).and_modify(|entry| {
            entry.roles.remove(&role);
        });
    }

    fn map_token(&mut self, root: &EthAddress, child: &ContractAddress) {
        let old_root = self.child_mapping.get(child);
        if let Some(old_root) = old_root {
            self.root_mapping.remove(old_root.deref());
        }

        let old_child = self.root_mapping.get(root);
        if let Some(old_child) = old_child {
            self.child_mapping.remove(old_child.deref())
        }

        self.root_mapping.remove(root);
        self.root_mapping.entry(*root).or_insert(*child);

        self.child_mapping.remove(child);
        self.child_mapping.entry(*child).or_insert(*root);
    }

    #[allow(dead_code)]
    fn clean_map_token(&mut self, root: &EthAddress, child: &ContractAddress) {
        self.root_mapping.remove(root);
        self.child_mapping.remove(child);
    }

    fn increment_emit_event_index(&mut self) -> &mut Self {
        self.emit_event_index += 1;
        self
    }

    fn set_withdraw_fee(&mut self, fee: Amount) {
        self.withdraw_fee = fee;
    }

    fn set_treasurer(&mut self, treasurer: AccountAddress) {
        self.treasurer_address = treasurer;
    }

    fn set_operation(&mut self, op: u64) {
        self.processed_operations.insert(op);
    }

    fn has_operation(&self, op: u64) -> bool {
        self.processed_operations.contains(&op)
    }
}
// Contract functions

/// Initialize contract instance with no initial tokens.
#[init(contract = "bridge-manager", enable_logger, event = "BridgeEvent")]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
    logger: &mut impl HasLogger,
) -> InitResult<State<S>> {
    // Construct the initial contract state.
    let mut state = State::new(state_builder, ctx.init_origin());
    // Get the instantiater of this contract instance.
    let invoker = Address::Account(ctx.init_origin());

    state.grant_role(&invoker, Roles::Admin, state_builder);
    logger.log(&BridgeEvent::GrantRole(GrantRoleEvent {
        address: invoker,
        role: Roles::Admin,
    }))?;
    state.set_withdraw_fee(Amount::from_micro_ccd(0u64));

    Ok(state)
}

/// The parameter type for the contract function `hasRole`.
// Note: the order of the fields cannot be changed.
#[derive(Serialize, SchemaType)]
pub struct HasRoleQueryParamaters {
    pub address: Address,
    pub role: Roles,
}

/// The response which is sent back when calling the contract function
/// `hasRole`.
#[derive(Serialize, SchemaType)]
pub struct HasRoleQueryResponse(pub bool);
impl From<bool> for HasRoleQueryResponse {
    fn from(ok: bool) -> Self {
        HasRoleQueryResponse(ok)
    }
}

/// Tagged event to be serialized for the event log.
#[derive(Serialize, SchemaType)]
pub enum BridgeEvent {
    // TokenMapEvent
    TokenMap(TokenMapEvent),
    Deposit(DepositEvent),
    Withdraw(WithdrawEvent),
    GrantRole(GrantRoleEvent),
    RevokeRole(RevokeRoleEvent),
}

#[derive(Serialize, SchemaType)]
pub struct TokenMapEvent {
    pub id: u64,
    pub root: EthAddress,
    pub child: ContractAddress,
}

#[derive(Serialize, SchemaType)]
pub struct DepositEvent {
    pub id: u64,
    pub contract: ContractAddress,
    pub amount: ContractTokenAmount,
    pub token_id: TokenIdU64,
}

#[derive(Serialize, SchemaType)]
pub struct WithdrawEvent {
    pub id: u64,
    pub contract: ContractAddress,
    pub amount: ContractTokenAmount,
    pub ccd_address: Address,
    pub eth_address: EthAddress,
    pub token_id: TokenIdU64,
}

// A GrantRoleEvent introduced by this smart contract.
#[derive(Serialize, SchemaType)]
pub struct GrantRoleEvent {
    /// Address that has been given the role
    address: Address,
    role: Roles,
}
// A RevokeRoleEvent introduced by this smart contract.
#[derive(Serialize, SchemaType)]
pub struct RevokeRoleEvent {
    /// Address that has been revoked the role
    address: Address,
    role: Roles,
}
/// Check if an address has a role.
/// TODO Should this be batched like the rest of the functions ?
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "bridge-manager",
    name = "hasRole",
    parameter = "HasRoleQueryParamaters",
    return_value = "HasRoleQueryResponse"
)]
fn contract_has_role<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<HasRoleQueryResponse> {
    let mut cursor = ctx.parameter_cursor();
    let query: HasRoleQueryParamaters = cursor.get()?;
    let address = query.address;
    let role = query.role;
    let has_role = _host.state().has_role(&address, role);
    Ok(HasRoleQueryResponse::from(has_role))
}

/// The parameter type for the contract function `grantRole`.
#[derive(Serialize, SchemaType)]
pub struct GrantRoleParams {
    pub address: Address,
    pub role: Roles,
}

/// Grant Permission to an address
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The sender does not have the required permission
#[receive(
    contract = "bridge-manager",
    name = "grantRole",
    parameter = "GrantRoleParams",
    enable_logger,
    mutable
)]
fn contract_grant_role<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
    // Parse the parameter.
    let params: GrantRoleParams = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, state_builder) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );

    state.grant_role(&params.address, params.role, state_builder);
    logger.log(&BridgeEvent::GrantRole(GrantRoleEvent {
        address: params.address,
        role: params.role,
    }))?;
    Ok(())
}

/// The parameter type for the contract function `removeRole`.
#[derive(Serialize, SchemaType)]
pub struct RemoveRoleParams {
    pub address: Address,
    pub role: Roles,
}

/// Remove Permission to an address
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The sender does not have the required permission
/// - the address does not have the role
#[receive(
    contract = "bridge-manager",
    name = "removeRole",
    parameter = "RemoveRoleParams",
    enable_logger,
    mutable
)]
fn contract_remove_role<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
    // Parse the parameter.
    let params: RemoveRoleParams = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );

    ensure!(
        state.has_role(&params.address, params.role),
        ContractError::Custom(CustomContractError::RoleNotAssigned)
    );

    state.remove_role(&params.address, params.role);
    logger.log(&BridgeEvent::RevokeRole(RevokeRoleEvent {
        address: params.address,
        role: params.role,
    }))?;
    Ok(())
}

/// The parameter type for the contract function `setWithdrawFee`.
#[derive(Serialize, SchemaType)]
pub struct SetWithdrawFeeParams {
    pub amount: Amount,
}

/// Set withdraw fee required for withdrawing tokens.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The sender does not have the required permission
#[receive(
    contract = "bridge-manager",
    name = "setWithdrawFee",
    parameter = "SetWithdrawFeeParams",
    mutable
)]
fn contract_set_withdraw_fee<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    // Parse the parameter.
    let params: SetWithdrawFeeParams = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );

    state.set_withdraw_fee(params.amount);

    Ok(())
}

/// The parameter type for the contract function `setTreasurer`.
#[derive(Serialize, SchemaType)]
pub struct SetTreasurer {
    pub account: AccountAddress,
}

/// Set treasurer account where fees for withdrawing are collected
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The sender does not have the required permission
#[receive(
    contract = "bridge-manager",
    name = "setTreasurer",
    parameter = "SetTreasurer",
    mutable
)]
fn contract_set_treasurer<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    // Parse the parameter.
    let params: SetTreasurer = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );

    state.set_treasurer(params.account);

    Ok(())
}

/// The parameter type for the contract function `upgrade`.
/// Takes the new module and optionally an entrypoint to call in the new module
/// after triggering the upgrade. The upgrade is reverted if the entrypoint
/// fails. This is useful for doing migration in the same transaction triggering
/// the upgrade.
#[derive(Serialize, SchemaType)]
struct UpgradeParams {
    /// The new module reference.
    module: ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

#[receive(
    contract = "bridge-manager",
    name = "upgrade",
    parameter = "UpgradeParams",
    error = "ContractError",
    mutable
)]
fn contract_upgrade<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();

    // Check that only the admin is authorized to set implementors.
    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );
    // Parse the parameter.
    let params: UpgradeParams = ctx.parameter_cursor().get()?;
    // Trigger the upgrade.
    host.upgrade(params.module)?;
    // Call the migration function if provided.
    if let Some((func, parameters)) = params.migrate {
        host.invoke_contract_raw(
            &ctx.self_address(),
            parameters.as_parameter(),
            func.as_entrypoint_name(),
            Amount::zero(),
        )?;
    }
    Ok(())
}

/// The parameter type for the contract function `setPaused`.
#[derive(Serialize, SchemaType)]
#[repr(transparent)]
struct SetPausedParams {
    /// Contract is paused if `paused = true` and unpaused if `paused = false`.
    paused: bool,
}
/// Pause/Unpause this smart contract instance by the admin. All non-admin
/// state-mutative functions (wrap, unwrap, transfer, updateOperator) cannot be
/// executed when the contract is paused.
///
/// It rejects if:
/// - Sender is not the admin of the contract instance.
/// - It fails to parse the parameter.
#[receive(
    contract = "bridge-manager",
    name = "setPaused",
    parameter = "SetPausedParams",
    error = "ContractError",
    mutable
)]
fn contract_set_paused<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<()> {
    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();

    // Check that only the admin is authorized to set implementors.
    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );

    // Parse the parameter.
    let params: SetPausedParams = ctx.parameter_cursor().get()?;

    // Update the paused variable.
    host.state_mut().paused = params.paused;

    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct DepositParams {
    pub address: Address,
    pub amount: ContractTokenAmount,
    pub token_id: TokenIdU64,
}
#[receive(
    contract = "bridge-manager",
    name = "receiveStateUpdate",
    parameter = "StateUpdate",
    enable_logger,
    mutable
)]
fn contract_receive_state_update<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
    // Check that contract is not paused.
    ensure!(
        !host.state().paused,
        ContractError::Custom(CustomContractError::ContractPaused)
    );
    // Parse the parameter.
    let state_update: StateUpdate = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::StateSyncer),
        ContractError::Unauthorized
    );

    match state_update {
        StateUpdate::TokenMap(op) => {
            ensure!(
                !state.has_operation(op.id),
                ContractError::Custom(CustomContractError::OperationAlreadyProcessed)
            );
            state.set_operation(op.id);
            state.map_token(&op.root, &op.child);
            logger.log(&BridgeEvent::TokenMap(TokenMapEvent {
                id: op.id,
                root: op.root,
                child: op.child,
            }))?;
        }
        StateUpdate::Deposit(op) => {
            ensure!(
                !state.has_operation(op.id),
                ContractError::Custom(CustomContractError::OperationAlreadyProcessed)
            );
            state.set_operation(op.id);
            let deposit_params = DepositParams {
                address: op.user,
                amount: op.amount,
                token_id: op.token_id,
            };

            let child_token = match state.root_mapping.get(&op.root) {
                None => return Err(ContractError::Custom(CustomContractError::TokenNotMapped)),
                Some(child) => *child.deref(),
            };
            host.invoke_contract(
                &child_token,
                &deposit_params,
                EntrypointName::new("deposit").unwrap(),
                Amount { micro_ccd: 0 },
            )?;
            logger.log(&BridgeEvent::Deposit(DepositEvent {
                id: op.id,
                contract: child_token,
                amount: deposit_params.amount,
                token_id: deposit_params.token_id,
            }))?;
        }
    }
    Ok(())
}

#[derive(Serialize, SchemaType)]
pub struct WithdrawParams {
    pub eth_address: EthAddress,
    pub amount: ContractTokenAmount,
    pub token: ContractAddress,
    pub token_id: TokenIdU64,
}

// The parameter type for the contract function `withdraw`.
#[derive(Serialize, SchemaType)]
pub struct Cis2WithdrawParams {
    pub address: Address,
    pub amount: ContractTokenAmount,
    pub token_id: TokenIdU64,
}
#[receive(
    contract = "bridge-manager",
    name = "withdraw",
    parameter = "WithdrawParams",
    enable_logger,
    mutable,
    payable
)]
fn contract_withdraw<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    amount: Amount,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
    // Check that contract is not paused.
    ensure!(
        !host.state().paused,
        ContractError::Custom(CustomContractError::ContractPaused)
    );
    ensure!(
        ctx.sender().is_account(),
        ContractError::Custom(CustomContractError::OnlyAccountsCanWithdraw)
    );
    // Parse the parameter.
    let withdraw_params: WithdrawParams = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    // Transfer fee to treasury.
    let fee = host.state().withdraw_fee;
    ensure!(
        amount >= fee,
        ContractError::Custom(CustomContractError::WithdrawFeeTooLow)
    );
    let treasurer = host.state().treasurer_address;
    host.invoke_transfer(&treasurer, amount)?;

    let params = Cis2WithdrawParams {
        amount: withdraw_params.amount,
        address: sender,
        token_id: withdraw_params.token_id,
    };
    host.state_mut().increment_emit_event_index();
    let event_index = host.state().emit_event_index;

    host.invoke_contract(
        &withdraw_params.token,
        &params,
        EntrypointName::new("withdraw").unwrap(),
        Amount { micro_ccd: 0 },
    )?;

    logger.log(&BridgeEvent::Withdraw(WithdrawEvent {
        id: event_index,
        contract: withdraw_params.token,
        amount: withdraw_params.amount,
        ccd_address: sender,
        eth_address: withdraw_params.eth_address,
        token_id: params.token_id,
    }))?;

    Ok(())
}

// Tests
#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    const ACCOUNT_0: AccountAddress = AccountAddress([0u8; 32]);
    const ADDRESS_0: Address = Address::Account(ACCOUNT_0);
    const ACCOUNT_1: AccountAddress = AccountAddress([1u8; 32]);
    const ADDRESS_1: Address = Address::Account(ACCOUNT_1);
    const ACCOUNT_2: AccountAddress = AccountAddress([2u8; 32]);
    const ADDRESS_2: Address = Address::Account(ACCOUNT_2);
    const TREASURY_ACCOUNT: AccountAddress = AccountAddress([3u8; 32]);

    const ETH_ADDRESS: EthAddress = EthAddress {
        eth_address: [1u8; 20],
    };
    const ETH_WALLET_ADDRESS: EthAddress = EthAddress {
        eth_address: [2u8; 20],
    };

    const CIS2_ADDRESS: ContractAddress = ContractAddress {
        index: 42,
        subindex: 0,
    };

    fn token_amount(amount: u64) -> ContractTokenAmount {
        TokenAmountU256(amount.into())
    }

    /// Test helper function which creates a contract state with admin granted
    /// to ACCOUNT_0
    fn initial_state<S: HasStateApi>(state_builder: &mut StateBuilder<S>) -> State<S> {
        let mut state = State::new(state_builder, ACCOUNT_2);
        state.grant_role(&ADDRESS_0, Roles::Admin, state_builder);
        state
    }
    /// Test initialization succeeds and the tokens are owned by the contract
    /// instantiater and the appropriate events are logged.
    #[concordium_test]
    fn test_init() {
        // Setup the context
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);

        let mut logger = TestLogger::init();
        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        claim_eq!(
            state.has_role(&ADDRESS_0, Roles::Admin),
            true,
            "Initiator does not have admin"
        );
        // Check the logs
        claim_eq!(logger.logs.len(), 1, "Exactly 1 events should be logged");

        claim!(
            logger
                .logs
                .contains(&to_bytes(&BridgeEvent::GrantRole(GrantRoleEvent {
                    address: ADDRESS_0,
                    role: Roles::Admin,
                }))),
            "Missing event for the new admin"
        );
    }

    /// Test the `viewConfiguration` function.
    #[concordium_test]
    fn test_view_configuration() {
        let emit_event_index = 9;
        let withdraw_fee = Amount::from_micro_ccd(4);

        let builder = TestStateBuilder::new();
        let mut state_builder = TestStateBuilder::new();

        let state = State {
            paused: true,
            roles: state_builder.new_map(),
            root_mapping: state_builder.new_map(),
            child_mapping: state_builder.new_map(),
            emit_event_index,
            withdraw_fee,
            treasurer_address: TREASURY_ACCOUNT,
            processed_operations: state_builder.new_set(),
        };

        let host = TestHost::new(state, builder);

        let ctx = TestReceiveContext::empty();

        // Check state configuration
        let configuration_result = contract_view_configuration(&ctx, &host);

        claim_eq!(
            configuration_result,
            Ok(ViewConfigurationState {
                paused: true,
                emit_event_index,
                withdraw_fee,
                treasurer_address: TREASURY_ACCOUNT,
            }),
            "Configuration state should be correct"
        );
    }

    /// Test adding an operator succeeds and the appropriate event is logged.
    #[concordium_test]
    fn test_roles() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);
        claim!(
            logger
                .logs
                .contains(&to_bytes(&BridgeEvent::GrantRole(GrantRoleEvent {
                    address: ADDRESS_0,
                    role: Roles::Admin,
                }))),
            "Missing event for the new admin"
        );
        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role: Roles::Mapper,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&parameter_bytes);
        let mut result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_err(), "ADDRESS_1 not allowed to grant role");

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        result = contract_grant_role(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_0  is allowed to grant role");
        claim!(
            logger
                .logs
                .contains(&to_bytes(&BridgeEvent::GrantRole(GrantRoleEvent {
                    address: ADDRESS_1,
                    role: Roles::Mapper,
                }))),
            "Missing event for grant role"
        );

        let query = HasRoleQueryParamaters {
            address: ADDRESS_1,
            role: Roles::Mapper,
        };
        let query_bytes = to_bytes(&query);

        ctx.set_parameter(&query_bytes);

        let has_role_result = contract_has_role(&ctx, &host);
        claim!(has_role_result.is_ok(), "has role error");
        claim!(has_role_result.unwrap().0, "ADDRESS1 has manager role");

        // Remove role
        let parameter = RemoveRoleParams {
            address: ADDRESS_1,
            role: Roles::Mapper,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        result = contract_remove_role(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_0 is allowed to remove role");
        claim!(
            logger
                .logs
                .contains(&to_bytes(&BridgeEvent::RevokeRole(RevokeRoleEvent {
                    address: ADDRESS_1,
                    role: Roles::Mapper,
                }))),
            "Missing event for revoke role"
        );

        // Check has role again
        let query = HasRoleQueryParamaters {
            address: ADDRESS_1,
            role: Roles::Mapper,
        };
        let query_bytes = to_bytes(&query);

        ctx.set_parameter(&query_bytes);

        let has_role_result = contract_has_role(&ctx, &host);
        claim!(has_role_result.is_ok(), "has role error");
        claim!(
            !has_role_result.unwrap().0,
            "ADDRESS1 should not have manager role"
        );

        // Remove role for not existing address
        let parameter = RemoveRoleParams {
            address: ADDRESS_2,
            role: Roles::Mapper,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        result = contract_remove_role(&ctx, &mut host, &mut logger);
        claim!(result.is_err(), "ADDRESS_2 does not have role");
    }

    /// Test `view_roles` function displays the `roles` content of the state.
    /// Add the ADMIN and MAPPER role to ACCOUNT_0 and the MAPPER role to
    /// ACCOUNT_1.
    #[concordium_test]
    fn test_view_roles() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role: Roles::Mapper,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let grant_role_result = contract_grant_role(&ctx, &mut host, &mut logger);
        claim!(
            grant_role_result.is_ok(),
            "ADDRESS_0  is allowed to grant role"
        );

        let parameter = GrantRoleParams {
            address: ADDRESS_0,
            role: Roles::Mapper,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let grant_role_result2 = contract_grant_role(&ctx, &mut host, &mut logger);
        claim!(
            grant_role_result2.is_ok(),
            "ADDRESS_0  is allowed to grant role"
        );

        // Testing the `viewRoles` function
        let roles_result = contract_view_roles(&ctx, &host);

        let roles = roles_result.expect_report("Calling contract_view_roles expected to succeed.");

        // Check the roles_result
        claim_eq!(
            roles.all_roles.len(),
            2,
            "Exactly 2 accounts should have roles"
        );
        claim_eq!(
            roles.all_roles[0],
            (
                concordium_std::Address::Account(ACCOUNT_0),
                ViewRolesState {
                    roles: vec![Roles::Admin, Roles::Mapper],
                }
            ),
            "ACCOUNT_0 should have the roles Admin and Mapper"
        );
        claim_eq!(
            roles.all_roles[1],
            (
                concordium_std::Address::Account(ACCOUNT_1),
                ViewRolesState {
                    roles: vec![Roles::Mapper],
                }
            ),
            "ACCOUNT_1 should have the role Mapper"
        );
    }

    /// Test if token mappings can be viewed in the state.
    #[concordium_test]
    fn test_token_mappings_view_function() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_2,
            role: Roles::StateSyncer,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0 is allowed to grant role");

        let parameter = StateUpdate::TokenMap(TokenMapOperation {
            id: 1u64,
            root: ETH_ADDRESS,
            child: CIS2_ADDRESS,
        });

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_2);
        ctx.set_parameter(&parameter_bytes);
        let result = contract_receive_state_update(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_2  is allowed to state update");

        claim!(
            *host.state().root_mapping.get(&ETH_ADDRESS).unwrap().deref() == CIS2_ADDRESS,
            "Mapping must be succesfull"
        );

        // Check `viewTokenMappings` function
        let token_mappings_result = contract_view_token_mappings(&ctx, &host);

        let token_mappings = token_mappings_result
            .expect_report("Calling contract_view_token_mappings expected to succeed.");

        claim_eq!(
            token_mappings.root_mappings,
            vec![(ETH_ADDRESS, CIS2_ADDRESS)],
            "Initiator does not have admin"
        );
        claim_eq!(
            token_mappings.child_mappings,
            vec![(CIS2_ADDRESS, ETH_ADDRESS)],
            "Initiator does not have admin"
        );
    }

    /// Test deposit flow. Add tokens to token mappings and deposit a token.
    #[concordium_test]
    fn test_deposit_flow() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_2,
            role: Roles::StateSyncer,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0 is allowed to grant role");

        let parameter = StateUpdate::TokenMap(TokenMapOperation {
            id: 1u64,
            root: ETH_ADDRESS,
            child: CIS2_ADDRESS,
        });

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&parameter_bytes);
        let mut result: ContractResult<()> =
            contract_receive_state_update(&ctx, &mut host, &mut logger);

        claim!(result.is_err(), "ADDRESS_1 not allowed to state update");

        ctx.set_sender(ADDRESS_2);
        ctx.set_parameter(&parameter_bytes);
        result = contract_receive_state_update(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_2  is allowed to state update");

        claim!(
            *host.state().root_mapping.get(&ETH_ADDRESS).unwrap().deref() == CIS2_ADDRESS,
            "Mapping must be succesfull"
        );

        let entrypoint_deposit = OwnedEntrypointName::new_unchecked("deposit".into());
        // We are simulating reentrancy with this mock because we mutate the state.
        host.setup_mock_entrypoint(
            CIS2_ADDRESS,
            entrypoint_deposit,
            MockFn::new_v1(
                |_parameter, _amount, _balance, _state: &mut State<TestStateApi>| {
                    let deposit = from_bytes::<DepositParams>(_parameter.0).unwrap();

                    claim!(
                        deposit.amount == token_amount(42),
                        "Deposit amount is correct"
                    );
                    claim!(
                        deposit.token_id == TokenIdU64(0),
                        "Token id should be correct"
                    );
                    claim!(deposit.address == ADDRESS_1, "Address should be correct");
                    Ok((true, ()))
                },
            ),
        );
        let parameter = StateUpdate::Deposit(DepositOperation {
            id: 2u64,
            user: ADDRESS_1,
            root: ETH_ADDRESS,
            amount: token_amount(42),
            token_id: TokenIdU64(0),
        });

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_2);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> =
            contract_receive_state_update(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_2  is allowed to state update");

        let index = 1u64;
        let parameter_bytes = to_bytes(&index);

        ctx.set_parameter(&parameter_bytes);
        // Check `isProcessed` function
        let is_processed_result = contract_is_processed(&ctx, &host);

        claim_eq!(
            is_processed_result,
            Ok(true),
            "Event index should be processed"
        );
    }

    #[concordium_test]
    fn test_withdraw_flow_disallow_contract_calls() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_2,
            role: Roles::StateSyncer,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0 is allowed to grant role");

        let parameter = StateUpdate::TokenMap(TokenMapOperation {
            id: 1u64,
            root: ETH_ADDRESS,
            child: CIS2_ADDRESS,
        });

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&parameter_bytes);
        let mut result: ContractResult<()> =
            contract_receive_state_update(&ctx, &mut host, &mut logger);

        claim!(result.is_err(), "ADDRESS_1 not allowed to state update");

        ctx.set_sender(ADDRESS_2);
        ctx.set_parameter(&parameter_bytes);
        result = contract_receive_state_update(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_2  is allowed to state update");

        claim!(
            *host.state().root_mapping.get(&ETH_ADDRESS).unwrap().deref() == CIS2_ADDRESS,
            "Mapping must be succesfull"
        );

        let parameter = WithdrawParams {
            eth_address: ETH_WALLET_ADDRESS,
            amount: token_amount(42),
            token_id: TokenIdU64(0),
            token: CIS2_ADDRESS,
        };

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(Address::Contract(ContractAddress {
            index: 1,
            subindex: 2,
        }));
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> =
            contract_withdraw(&ctx, &mut host, Amount { micro_ccd: 0 }, &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(
                CustomContractError::OnlyAccountsCanWithdraw
            )),
            "Pause should fail because not the current admin tries to invoke it"
        );
    }
    /// Test adding an operator succeeds and the appropriate event is logged.
    #[concordium_test]
    fn test_withdraw_flow() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_2,
            role: Roles::StateSyncer,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0 is allowed to grant role");

        let parameter = StateUpdate::TokenMap(TokenMapOperation {
            id: 1u64,
            root: ETH_ADDRESS,
            child: CIS2_ADDRESS,
        });

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&parameter_bytes);
        let mut result: ContractResult<()> =
            contract_receive_state_update(&ctx, &mut host, &mut logger);

        claim!(result.is_err(), "ADDRESS_1 not allowed to state update");

        ctx.set_sender(ADDRESS_2);
        ctx.set_parameter(&parameter_bytes);
        result = contract_receive_state_update(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_2  is allowed to state update");

        claim!(
            *host.state().root_mapping.get(&ETH_ADDRESS).unwrap().deref() == CIS2_ADDRESS,
            "Mapping must be succesfull"
        );
        let entrypoint_withdraw = OwnedEntrypointName::new_unchecked("withdraw".into());
        // pub struct WithdrawParams {
        // pub eth_address: EthAddress,
        // pub amount: ContractTokenAmount,
        // pub token: ContractAddress,
        // pub token_id: TokenIdU64,
        // }
        // We are simulating reentrancy with this mock because we mutate the state.
        host.setup_mock_entrypoint(
            CIS2_ADDRESS,
            entrypoint_withdraw,
            MockFn::new_v1(
                |_parameter, _amount, _balance, _state: &mut State<TestStateApi>| {
                    let params = from_bytes::<Cis2WithdrawParams>(_parameter.0).unwrap();

                    claim!(
                        params.amount == token_amount(42),
                        "Deposit amount is correct"
                    );
                    claim!(
                        params.token_id == TokenIdU64(0),
                        "Token id should be correct"
                    );
                    claim!(params.address == ADDRESS_1, "Address should be correct");
                    Ok((true, ()))
                },
            ),
        );
        let parameter = WithdrawParams {
            eth_address: ETH_WALLET_ADDRESS,
            amount: token_amount(42),
            token_id: TokenIdU64(0),
            token: CIS2_ADDRESS,
        };

        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> =
            contract_withdraw(&ctx, &mut host, Amount { micro_ccd: 0 }, &mut logger);

        claim!(result.is_ok(), "ADDRESS_1  is allowed to withdraw");
    }
    /// Test pausing the contract.
    #[concordium_test]
    fn test_pause() {
        // Set up the context.
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);

        // Set up the parameter to pause the contract.
        let parameter_bytes = to_bytes(&true);
        ctx.set_parameter(&parameter_bytes);

        // Set up the state and host.
        let mut builder = TestStateBuilder::new();
        let state = initial_state(&mut builder);
        let mut host = TestHost::new(state, builder);
        // Call the contract function.
        let result: ContractResult<()> = contract_set_paused(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check contract is paused.
        claim_eq!(host.state().paused, true, "Smart contract should be paused");
    }

    /// Test unpausing the contract.
    #[concordium_test]
    fn test_unpause() {
        // Set up the context.
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);

        // Set up the parameter to pause the contract.
        let parameter_bytes = to_bytes(&true);
        ctx.set_parameter(&parameter_bytes);

        // Set up the state and host.
        let mut builder = TestStateBuilder::new();
        let state = initial_state(&mut builder);
        let mut host = TestHost::new(state, builder);
        // Call the contract function.
        let result: ContractResult<()> = contract_set_paused(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check contract is paused.
        claim_eq!(host.state().paused, true, "Smart contract should be paused");

        // Set up the parameter to unpause the contract.
        let parameter_bytes = to_bytes(&false);
        ctx.set_parameter(&parameter_bytes);

        // Call the contract function.
        let result: ContractResult<()> = contract_set_paused(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check contract is unpaused.
        claim_eq!(
            host.state().paused,
            false,
            "Smart contract should be unpaused"
        );
    }

    /// Test that only the current admin can pause/unpause the contract.
    #[concordium_test]
    fn test_pause_not_authorized() {
        // Set up the context.
        let mut ctx = TestReceiveContext::empty();
        // NEW_ADMIN is not the current admin but tries to pause/unpause the contract.
        ctx.set_sender(ADDRESS_1);

        // Set up the parameter to pause the contract.
        let parameter_bytes = to_bytes(&true);
        ctx.set_parameter(&parameter_bytes);

        // Set up the state and host.
        let mut builder = TestStateBuilder::new();
        let state = initial_state(&mut builder);
        let mut host = TestHost::new(state, builder);
        // Call the contract function.
        let result: ContractResult<()> = contract_set_paused(&ctx, &mut host);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Unauthorized),
            "Pause should fail because not the current admin tries to invoke it"
        );
    }

    /// Test that one can NOT call non-admin state-mutative functions (wrap,
    /// unwrap, transfer, updateOperator) when the contract is paused.
    #[concordium_test]
    fn test_no_execution_of_state_mutative_functions_when_paused() {
        // Set up the context.
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);

        // Set up the parameter to pause the contract.
        let parameter_bytes = to_bytes(&true);
        ctx.set_parameter(&parameter_bytes);

        // Set up the state and host.
        let mut builder = TestStateBuilder::new();
        let state = initial_state(&mut builder);
        let mut host = TestHost::new(state, builder);
        // Call the contract function.
        let result: ContractResult<()> = contract_set_paused(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check contract is paused.
        claim_eq!(host.state().paused, true, "Smart contract should be paused");

        let mut logger = TestLogger::init();

        // Call the `transfer` function.
        let result: ContractResult<()> =
            contract_withdraw(&ctx, &mut host, Amount::from_ccd(100), &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(CustomContractError::ContractPaused)),
            "Transfer should fail because contract is paused"
        );

        // Call the `updateOperator` function.
        let result: ContractResult<()> =
            contract_receive_state_update(&ctx, &mut host, &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(CustomContractError::ContractPaused)),
            "Update operator should fail because contract is paused"
        );
    }
    #[concordium_test]
    fn test_upgradability() {
        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);
        ctx.set_owner(ACCOUNT_0);

        let self_address = ContractAddress::new(0, 0);
        ctx.set_self_address(self_address);

        let new_module_ref = ModuleReference::from([0u8; 32]);
        let migration_entrypoint = OwnedEntrypointName::new_unchecked("migration".into());

        // and parameter.
        let parameter = UpgradeParams {
            module: new_module_ref,
            migrate: Some((migration_entrypoint.clone(), OwnedParameter(Vec::new()))),
        };
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut builder = TestStateBuilder::new();
        let state = initial_state(&mut builder);
        let mut host = TestHost::new(state, builder);
        host.setup_mock_upgrade(new_module_ref, Ok(()));
        host.setup_mock_entrypoint(self_address, migration_entrypoint, MockFn::returning_ok(()));

        let result: ContractResult<()> = contract_upgrade(&ctx, &mut host);

        claim_eq!(result, Ok(()));
    }

    #[concordium_test]
    fn test_upgradability_rejects() {
        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);
        ctx.set_owner(ACCOUNT_0);

        let new_module_ref = ModuleReference::from([0u8; 32]);

        // and parameter.
        let parameter = UpgradeParams {
            module: new_module_ref,
            migrate: None,
        };
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut builder = TestStateBuilder::new();
        let state = initial_state(&mut builder);
        let mut host = TestHost::new(state, builder);
        host.setup_mock_upgrade(new_module_ref, Err(UpgradeError::MissingModule));

        let result: ContractResult<()> = contract_upgrade(&ctx, &mut host);

        claim_eq!(
            result,
            Err(ContractError::Custom(
                CustomContractError::FailedUpgradeMissingModule
            ))
        );
    }
}
