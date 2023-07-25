//! An example implementation of CIS2 for a single fungible token.
//!
//! # Description
//! Note: The word 'address' refers to either an account address or a
//! contract address.
//!
//! As follows from the CIS2 specification, the contract has a `transfer`
//! function for transferring an amount of a specific token type from one
//! address to another address. An address can enable and disable one or more
//! addresses as operators. An operator of some token owner address is allowed
//! to transfer any tokens of the owner.
//!
//! Besides the contract functions required for the CIS2 standard, this contract
//! implements a `deposit` function that allows the `bridge-manager` smart
//! contract to mint tokens and a `withdraw` function that allows the
//! `bridge-manager` smart contract to burn tokens.

#![cfg_attr(not(feature = "std"), no_std)]
use concordium_cis2::{Cis2Event, *};
use concordium_std::{collections::BTreeMap, *};
use primitive_types::U256;

/// The id of the token in this contract.
const TOKEN_ID: ContractTokenId = TokenIdUnit();

/// Tag for the GrantRole event.
pub const GRANT_ROLE_EVENT_TAG: u8 = 0;
/// Tag for the RevokeRole event.
pub const REVOKE_ROLE_EVENT_TAG: u8 = 1;

/// List of supported standards by this contract address.
const SUPPORTS_STANDARDS: [StandardIdentifier<'static>; 2] =
    [CIS0_STANDARD_IDENTIFIER, CIS2_STANDARD_IDENTIFIER];
// Types

/// Contract token ID type.
/// Since this contract will only ever contain this one token type, we use the
/// smallest possible token ID.
type ContractTokenId = TokenIdUnit;

/// Contract token amount type.
/// Since this contract is wrapping the CCD and the CCD can be represented as a
/// u64, we can specialize the token amount to u64 reducing module size and cost
/// of arithmetics.
type ContractTokenAmount = TokenAmountU256;
const TOKEN_AMOUNT_ZERO: TokenAmountU256 = TokenAmountU256(U256([0; 4]));

/// The state tracked for each address.
#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
struct AddressState<S> {
    /// The number of tokens owned by this address.
    balance:   ContractTokenAmount,
    /// The address which are currently enabled as operators for this token and
    /// this address.
    operators: StateSet<Address, S>,
}
#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
struct AddressRoleState<S> {
    roles: StateSet<Roles, S>,
}

/// The contract state,
#[derive(Serial, DeserialWithState, StateClone)]
#[concordium(state_parameter = "S")]
struct State<S: HasStateApi> {
    /// Contract is paused if `paused = true` and unpaused if `paused = false`.
    paused:       bool,
    /// Map specifying the `AddressState` (balance and operators) for every
    /// address.
    token:        StateMap<Address, AddressState<S>, S>,
    roles:        StateMap<Address, AddressRoleState<S>, S>,
    /// The MetadataUrl of the token.
    /// `StateBox` allows for lazy loading data. This is helpful
    /// in the situations when one wants to do a partial update not touching
    /// this field, which can be large.
    metadata_url: StateBox<concordium_cis2::MetadataUrl, S>,
    implementors: StateMap<StandardIdentifierOwned, Vec<ContractAddress>, S>,
}

/// The return type for the contract function `view`.
#[derive(Serialize, SchemaType)]
struct ReturnBasicState {
    /// The metadata URL of the token.
    metadata_url: concordium_cis2::MetadataUrl,
    /// Contract is paused if `paused = true` and unpaused if `paused = false`.
    paused:       bool,
}

/// Part of the return type of the `viewRoles` function.
#[derive(Serialize, SchemaType, PartialEq)]
struct ViewRolesState {
    /// Vector of roles.
    roles: Vec<Roles>,
}

/// The return type of the `viewRoles` function.
#[derive(Serialize, SchemaType)]
struct ViewAllRolesState {
    /// Vector specifiying for each address a vector of its associated roles.
    all_roles: Vec<(Address, ViewRolesState)>,
}

/// The return type of the `viewTokenOwners` function.
#[derive(Serialize, SchemaType)]
struct ViewTokenOwners {
    token_owners: Vec<Address>,
}

/// The parameter type for the contract function `setPaused`.
#[derive(Serialize, SchemaType)]
#[repr(transparent)]
struct SetPausedParams {
    /// Contract is paused if `paused = true` and unpaused if `paused = false`.
    paused: bool,
}

// A GrantRoleEvent introduced by this smart contract.
#[derive(Serial, SchemaType)]
struct GrantRoleEvent {
    /// Address that has been given the role
    address: Address,
    role:    Roles,
}
// A RevokeRoleEvent introduced by this smart contract.
#[derive(Serial, SchemaType)]
struct RevokeRoleEvent {
    /// Address that has been revoked the role
    address: Address,
    role:    Roles,
}
/// Tagged events to be serialized for the event log.
enum BridgeableEvent {
    GrantRole(GrantRoleEvent),
    RevokeRole(RevokeRoleEvent),
    Cis2Event(Cis2Event<ContractTokenId, ContractTokenAmount>),
}

impl Serial for BridgeableEvent {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        match self {
            BridgeableEvent::GrantRole(event) => {
                out.write_u8(GRANT_ROLE_EVENT_TAG)?;
                event.serial(out)
            }
            BridgeableEvent::RevokeRole(event) => {
                out.write_u8(REVOKE_ROLE_EVENT_TAG)?;
                event.serial(out)
            }
            BridgeableEvent::Cis2Event(event) => event.serial(out),
        }
    }
}
/// Manual implementation of the `BridgeableEventSchema` which includes both the
/// events specified in this contract and the events specified in the CIS-2
/// library. The events are tagged to distinguish them on-chain.
impl schema::SchemaType for BridgeableEvent {
    fn get_type() -> schema::Type {
        let mut event_map = BTreeMap::new();
        event_map.insert(
            GRANT_ROLE_EVENT_TAG,
            (
                "GrantRole".to_string(),
                schema::Fields::Named(vec![
                    (String::from("address"), Address::get_type()),
                    (String::from("role"), Roles::get_type()),
                ]),
            ),
        );
        event_map.insert(
            REVOKE_ROLE_EVENT_TAG,
            (
                "RevokeRole".to_string(),
                schema::Fields::Named(vec![
                    (String::from("address"), Address::get_type()),
                    (String::from("role"), Roles::get_type()),
                ]),
            ),
        );
        event_map.insert(
            TRANSFER_EVENT_TAG,
            (
                "Transfer".to_string(),
                schema::Fields::Named(vec![
                    (String::from("token_id"), ContractTokenId::get_type()),
                    (String::from("amount"), ContractTokenAmount::get_type()),
                    (String::from("from"), Address::get_type()),
                    (String::from("to"), Address::get_type()),
                ]),
            ),
        );
        event_map.insert(
            MINT_EVENT_TAG,
            (
                "Mint".to_string(),
                schema::Fields::Named(vec![
                    (String::from("token_id"), ContractTokenId::get_type()),
                    (String::from("amount"), ContractTokenAmount::get_type()),
                    (String::from("owner"), Address::get_type()),
                ]),
            ),
        );
        event_map.insert(
            BURN_EVENT_TAG,
            (
                "Burn".to_string(),
                schema::Fields::Named(vec![
                    (String::from("token_id"), ContractTokenId::get_type()),
                    (String::from("amount"), ContractTokenAmount::get_type()),
                    (String::from("owner"), Address::get_type()),
                ]),
            ),
        );
        event_map.insert(
            UPDATE_OPERATOR_EVENT_TAG,
            (
                "UpdateOperator".to_string(),
                schema::Fields::Named(vec![
                    (String::from("update"), OperatorUpdate::get_type()),
                    (String::from("owner"), Address::get_type()),
                    (String::from("operator"), Address::get_type()),
                ]),
            ),
        );
        event_map.insert(
            TOKEN_METADATA_EVENT_TAG,
            (
                "TokenMetadata".to_string(),
                schema::Fields::Named(vec![
                    (String::from("token_id"), ContractTokenId::get_type()),
                    (String::from("metadata_url"), MetadataUrl::get_type()),
                ]),
            ),
        );
        schema::Type::TaggedEnum(event_map)
    }
}
/// The parameter type for the contract function `setImplementors`.
/// Takes a standard identifier and list of contract addresses providing
/// implementations of this standard.
#[derive(Serialize, SchemaType)]
struct SetImplementorsParams {
    /// The identifier for the standard.
    id:           StandardIdentifierOwned,
    /// The addresses of the implementors of the standard.
    implementors: Vec<ContractAddress>,
}

/// The parameter type for the contract function `upgrade`.
/// Takes the new module and optionally an entrypoint to call in the new module
/// after triggering the upgrade. The upgrade is reverted if the entrypoint
/// fails. This is useful for doing migration in the same transaction triggering
/// the upgrade.
#[derive(Serialize, SchemaType)]
struct UpgradeParams {
    /// The new module reference.
    module:  ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

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
    // Role is not assigned
    RoleNotAssigned,
    /// Upgrade failed because the new module does not exist.
    FailedUpgradeMissingModule,
    /// Upgrade failed because the new module does not contain a contract with a
    /// matching name.
    FailedUpgradeMissingContract,
    /// Upgrade failed because the smart contract version of the module is not
    /// supported.
    FailedUpgradeUnsupportedModuleVersion,
}

type ContractError = Cis2Error<CustomContractError>;

type ContractResult<A> = Result<A, ContractError>;

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
    fn from(_cce: CallContractError<T>) -> Self { Self::InvokeContractError }
}

/// Mapping errors related to contract invocations to CustomContractError.
impl From<TransferError> for CustomContractError {
    fn from(_te: TransferError) -> Self { Self::InvokeTransferError }
}

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

/// Mapping CustomContractError to ContractError
impl From<CustomContractError> for ContractError {
    fn from(c: CustomContractError) -> Self { Cis2Error::Custom(c) }
}

#[derive(Serialize, PartialEq, Eq, Reject, Clone, Copy)]
pub enum Roles {
    Admin,
    Manager,
}

/// Manual implementation of the `Roles` schema.
impl schema::SchemaType for Roles {
    fn get_type() -> schema::Type {
        schema::Type::Enum(vec![
            ("Admin".to_string(), schema::Fields::None),
            ("Manager".to_string(), schema::Fields::None),
        ])
    }
}

impl<S: HasStateApi> State<S> {
    /// Creates a new state with no one owning any tokens by default.
    fn new(
        state_builder: &mut StateBuilder<S>,
        metadata_url: concordium_cis2::MetadataUrl,
    ) -> Self {
        State {
            paused:       false,
            token:        state_builder.new_map(),
            roles:        state_builder.new_map(),
            metadata_url: state_builder.new_box(metadata_url),
            implementors: state_builder.new_map(),
        }
    }

    /// Get the current balance of a given token id for a given address.
    /// Results in an error if the token id does not exist in the state.
    fn balance(
        &self,
        token_id: &ContractTokenId,
        address: &Address,
    ) -> ContractResult<ContractTokenAmount> {
        ensure_eq!(token_id, &TOKEN_ID, ContractError::InvalidTokenId);
        Ok(self
            .token
            .get(address)
            .map(|s| s.balance)
            .unwrap_or(TOKEN_AMOUNT_ZERO))
    }

    /// Check is an address is an operator of a specific owner address.
    /// Results in an error if the token id does not exist in the state.
    fn is_operator(&self, address: &Address, owner: &Address) -> bool {
        self.token
            .get(owner)
            .map(|address_state| address_state.operators.contains(address))
            .unwrap_or(false)
    }

    /// Update the state with a transfer.
    /// Results in an error if the token id does not exist in the state or if
    /// the from address have insufficient tokens to do the transfer.
    fn transfer(
        &mut self,
        token_id: &ContractTokenId,
        amount: ContractTokenAmount,
        from: &Address,
        to: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> ContractResult<()> {
        ensure_eq!(token_id, &TOKEN_ID, ContractError::InvalidTokenId);
        if amount == TOKEN_AMOUNT_ZERO {
            return Ok(());
        }
        {
            let mut from_state = self
                .token
                .get_mut(from)
                .ok_or(ContractError::InsufficientFunds)?;
            ensure!(
                from_state.balance >= amount,
                ContractError::InsufficientFunds
            );
            from_state.balance -= amount;
        }
        let mut to_state = self.token.entry(*to).or_insert_with(|| AddressState {
            balance:   TOKEN_AMOUNT_ZERO,
            operators: state_builder.new_set(),
        });
        to_state.balance += amount;

        Ok(())
    }

    /// Update the state adding a new operator for a given token id and address.
    /// Results in an error if the token id does not exist in the state.
    /// Succeeds even if the `operator` is already an operator for this
    /// `token_id` and `address`.
    fn add_operator(
        &mut self,
        owner: &Address,
        operator: &Address,
        state_builder: &mut StateBuilder<S>,
    ) {
        let mut owner_state = self.token.entry(*owner).or_insert_with(|| AddressState {
            balance:   TOKEN_AMOUNT_ZERO,
            operators: state_builder.new_set(),
        });
        owner_state.operators.insert(*operator);
    }

    /// Update the state removing an operator for a given token id and address.
    /// Results in an error if the token id does not exist in the state.
    /// Succeeds even if the `operator` is not an operator for this `token_id`
    /// and `address`.
    fn remove_operator(&mut self, owner: &Address, operator: &Address) {
        self.token.entry(*owner).and_modify(|address_state| {
            address_state.operators.remove(operator);
        });
    }

    fn mint(
        &mut self,
        token_id: &ContractTokenId,
        amount: ContractTokenAmount,
        owner: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> ContractResult<()> {
        ensure_eq!(token_id, &TOKEN_ID, ContractError::InvalidTokenId);
        let mut owner_state = self.token.entry(*owner).or_insert_with(|| AddressState {
            balance:   TOKEN_AMOUNT_ZERO,
            operators: state_builder.new_set(),
        });
        owner_state.balance += amount;
        Ok(())
    }

    fn burn(
        &mut self,
        token_id: &ContractTokenId,
        amount: ContractTokenAmount,
        owner: &Address,
    ) -> ContractResult<()> {
        ensure_eq!(token_id, &TOKEN_ID, ContractError::InvalidTokenId);
        if amount == TOKEN_AMOUNT_ZERO {
            return Ok(());
        }

        let mut from_state = self
            .token
            .get_mut(owner)
            .ok_or(ContractError::InsufficientFunds)?;
        ensure!(
            from_state.balance >= amount,
            ContractError::InsufficientFunds
        );
        from_state.balance -= amount;

        Ok(())
    }

    fn have_implementors(&self, std_id: &StandardIdentifierOwned) -> SupportResult {
        if let Some(addresses) = self.implementors.get(std_id) {
            SupportResult::SupportBy(addresses.to_vec())
        } else {
            SupportResult::NoSupport
        }
    }

    /// Set implementors for a given standard.
    fn set_implementors(
        &mut self,
        std_id: StandardIdentifierOwned,
        implementors: Vec<ContractAddress>,
    ) {
        self.implementors.insert(std_id, implementors);
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
}

// Contract functions

/// The parameter type for the contract function `init` and `setMetadataUrl`.
#[derive(Serialize, SchemaType, Clone)]
struct SetMetadataUrlParams {
    /// The URL following the specification RFC1738.
    url:  String,
    /// The hash of the document stored at the above URL.
    hash: Option<Sha256>,
}

/// Initialize contract instance with no initial tokens.
/// Logs a `Mint` event for the single token id with no amounts.
#[init(
    contract = "cis2-bridgeable",
    parameter = "SetMetadataUrlParams",
    event = "BridgeableEvent",
    enable_logger,
    crypto_primitives
)]
fn contract_init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
    logger: &mut impl HasLogger,
    _crypto: &impl HasCryptoPrimitives,
) -> InitResult<State<S>> {
    // Get the instantiater of this contract instance.
    let invoker = Address::Account(ctx.init_origin());

    let params: SetMetadataUrlParams = ctx.parameter_cursor().get()?;

    // Create the metadata_url
    let metadata_url = MetadataUrl {
        url:  params.url.clone(),
        hash: params.hash,
    };

    // Construct the initial contract state.
    let mut state = State::new(state_builder, metadata_url.clone());

    state.grant_role(&invoker, Roles::Admin, state_builder);

    // Log event for the newly minted token.
    logger.log(&BridgeableEvent::Cis2Event(Cis2Event::Mint(MintEvent {
        token_id: TOKEN_ID,
        amount:   TOKEN_AMOUNT_ZERO,
        owner:    invoker,
    })))?;

    // Log event for where to find metadata for the token
    logger.log(&BridgeableEvent::Cis2Event(Cis2Event::TokenMetadata::<
        _,
        ContractTokenAmount,
    >(TokenMetadataEvent {
        token_id: TOKEN_ID,
        metadata_url,
    })))?;

    // Log event for the new admin.
    logger.log(&BridgeableEvent::GrantRole(GrantRoleEvent {
        address: invoker,
        role:    Roles::Admin,
    }))?;

    Ok(state)
}

// Contract functions required by CIS2

#[allow(dead_code)]
type TransferParameter = TransferParams<ContractTokenId, ContractTokenAmount>;

/// Execute a list of token transfers, in the order of the list.
///
/// Logs a `Transfer` event and invoke a receive hook function for every
/// transfer in the list.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - Any of the transfers fail to be executed, which could be if:
///     - The `token_id` does not exist.
///     - The sender is not the owner of the token, or an operator for this
///       specific `token_id` and `from` address.
///     - The token is not owned by the `from`.
/// - Fails to log event.
/// - Any of the receive hook function calls rejects.
#[receive(
    contract = "cis2-bridgeable",
    name = "transfer",
    parameter = "TransferParameter",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_transfer<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
    // Check that contract is not paused.
    ensure!(
        !host.state().paused,
        ContractError::Custom(CustomContractError::ContractPaused)
    );
    let mut cursor = ctx.parameter_cursor();
    // Parse the number of transfers.
    let transfers_length: u16 = cursor.get()?;
    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    // Loop over the number of transfers.
    for _ in 0..transfers_length {
        // Parse one of the transfers.
        let Transfer {
            token_id,
            amount,
            from,
            to,
            data,
        } = cursor.get()?;
        let (state, state_builder) = host.state_and_builder();
        // Authenticate the sender for this transfer
        ensure!(
            from == sender || state.is_operator(&sender, &from),
            ContractError::Unauthorized
        );
        let to_address = to.address();
        // Update the contract state
        state.transfer(&token_id, amount, &from, &to_address, state_builder)?;

        // Log transfer event
        logger.log(&BridgeableEvent::Cis2Event(Cis2Event::Transfer(
            TransferEvent {
                token_id,
                amount,
                from,
                to: to_address,
            },
        )))?;

        // If the receiver is a contract, we invoke it.
        if let Receiver::Contract(address, function) = to {
            let parameter = OnReceivingCis2Params {
                token_id,
                amount,
                from,
                data,
            };
            host.invoke_contract(
                &address,
                &parameter,
                function.as_entrypoint_name(),
                Amount::zero(),
            )?;
        }
    }
    Ok(())
}

/// Enable or disable addresses as operators of the sender address.
/// Logs an `UpdateOperator` event.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The operator address is the same as the sender address.
/// - Fails to log event.
#[receive(
    contract = "cis2-bridgeable",
    name = "updateOperator",
    parameter = "UpdateOperatorParams",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_update_operator<S: HasStateApi>(
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
    let UpdateOperatorParams(params) = ctx.parameter_cursor().get()?;
    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, state_builder) = host.state_and_builder();
    for param in params {
        // Update the operator in the state.
        match param.update {
            OperatorUpdate::Add => state.add_operator(&sender, &param.operator, state_builder),
            OperatorUpdate::Remove => state.remove_operator(&sender, &param.operator),
        }

        // Log the appropriate event
        logger.log(&BridgeableEvent::Cis2Event(Cis2Event::<
            ContractTokenId,
            ContractTokenAmount,
        >::UpdateOperator(
            UpdateOperatorEvent {
                owner:    sender,
                operator: param.operator,
                update:   param.update,
            },
        )))?;
    }

    Ok(())
}

/// Set token metadata url
/// Logs an `UpdateOperator` event.
#[receive(
    contract = "cis2-bridgeable",
    name = "setTokenMetadataUrl",
    parameter = "SetMetadataUrlParams",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_set_token_metadata_url<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> ContractResult<()> {
    // Parse the parameter.
    let change_params: SetMetadataUrlParams = ctx.parameter_cursor().get()?;
    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();

    ensure!(
        state.has_role(&sender, Roles::Admin),
        ContractError::Unauthorized
    );

    // Create the metadata_url
    let metadata_url = MetadataUrl {
        url:  change_params.url.clone(),
        hash: change_params.hash,
    };

    // Update the hash variable.
    *host.state_mut().metadata_url = metadata_url.clone();

    // Log event for where to find metadata for the token
    logger.log(&BridgeableEvent::Cis2Event(Cis2Event::TokenMetadata::<
        _,
        ContractTokenAmount,
    >(TokenMetadataEvent {
        token_id: TOKEN_ID,
        metadata_url,
    })))?;

    Ok(())
}

/// Parameter type for the CIS-2 function `balanceOf` specialized to the subset
/// of TokenIDs used by this contract.
type ContractBalanceOfQueryParams = BalanceOfQueryParams<ContractTokenId>;

type ContractBalanceOfQueryResponse = BalanceOfQueryResponse<ContractTokenAmount>;

/// Get the balance of given token IDs and addresses.
///
/// It rejects if:
/// - Sender is not a contract.
/// - It fails to parse the parameter.
/// - Any of the queried `token_id` does not exist.
#[receive(
    contract = "cis2-bridgeable",
    name = "balanceOf",
    parameter = "ContractBalanceOfQueryParams",
    error = "ContractError",
    return_value = "ContractBalanceOfQueryResponse"
)]
fn contract_balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ContractBalanceOfQueryResponse> {
    // Parse the parameter.
    let params: ContractBalanceOfQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for query in params.queries {
        // Query the state for balance.
        let amount = host.state().balance(&query.token_id, &query.address)?;
        response.push(amount);
    }
    let result = ContractBalanceOfQueryResponse::from(response);
    Ok(result)
}

/// Takes a list of queries. Each query is an owner address and some address to
/// check as an operator of the owner address.
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "cis2-bridgeable",
    name = "operatorOf",
    parameter = "OperatorOfQueryParams",
    error = "ContractError",
    return_value = "OperatorOfQueryResponse"
)]
fn contract_operator_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<OperatorOfQueryResponse> {
    // Parse the parameter.
    let params: OperatorOfQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for query in params.queries {
        // Query the state for address being an operator of owner.
        let is_operator = host.state().is_operator(&query.address, &query.owner);
        response.push(is_operator);
    }
    let result = OperatorOfQueryResponse::from(response);
    Ok(result)
}

/// Parameter type for the CIS-2 function `tokenMetadata` specialized to the
/// subset of TokenIDs used by this contract.
// This type is pub to silence the dead_code warning, as this type is only used
// for when generating the schema.
pub type ContractTokenMetadataQueryParams = TokenMetadataQueryParams<ContractTokenId>;

/// Get the token metadata URLs and checksums given a list of token IDs.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - Any of the queried `token_id` does not exist.
#[receive(
    contract = "cis2-bridgeable",
    name = "tokenMetadata",
    parameter = "ContractTokenMetadataQueryParams",
    error = "ContractError",
    return_value = "TokenMetadataQueryResponse"
)]
fn contract_token_metadata<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<TokenMetadataQueryResponse> {
    let mut cursor = ctx.parameter_cursor();
    // Parse the number of queries.
    let queries_length: u8 = cursor.get()?;

    let metadata_url = host.state().metadata_url.clone();

    // Build the response.
    let mut response = Vec::with_capacity(queries_length.into());
    for _ in 0..queries_length {
        let token_id: ContractTokenId = cursor.get()?;
        // Check the token exists.
        ensure_eq!(token_id, TOKEN_ID, ContractError::InvalidTokenId);

        response.push(metadata_url.clone());
    }
    let result = TokenMetadataQueryResponse::from(response);
    Ok(result)
}

/// Get the supported standards or addresses for a implementation given list of
/// standard identifiers.
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "cis2-bridgeable",
    name = "supports",
    parameter = "SupportsQueryParams",
    error = "ContractError",
    return_value = "SupportsQueryResponse"
)]
fn contract_supports<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<SupportsQueryResponse> {
    // Parse the parameter.
    let params: SupportsQueryParams = ctx.parameter_cursor().get()?;

    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for std_id in params.queries {
        if SUPPORTS_STANDARDS.contains(&std_id.as_standard_identifier()) {
            response.push(SupportResult::Support);
        } else {
            response.push(host.state().have_implementors(&std_id));
        }
    }
    let result = SupportsQueryResponse::from(response);
    Ok(result)
}

/// Set the addresses for an implementation given a standard identifier and a
/// list of contract addresses.
///
/// It rejects if:
/// - Sender is not the owner of the contract instance.
/// - It fails to parse the parameter.
#[receive(
    contract = "cis2-bridgeable",
    name = "setImplementors",
    error = "ContractError",
    parameter = "SetImplementorsParams",
    mutable
)]
fn contract_set_implementor<S: HasStateApi>(
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
    let params: SetImplementorsParams = ctx.parameter_cursor().get()?;
    // Update the implementors in the state
    host.state_mut()
        .set_implementors(params.id, params.implementors);
    Ok(())
}

#[receive(
    contract = "cis2-bridgeable",
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

/// Pause/Unpause this smart contract instance by the admin. All non-admin
/// state-mutative functions (wrap, unwrap, transfer, updateOperator) cannot be
/// executed when the contract is paused.
///
/// It rejects if:
/// - Sender is not the admin of the contract instance.
/// - It fails to parse the parameter.
#[receive(
    contract = "cis2-bridgeable",
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

/// Function to view the basic state of the contract.
#[receive(
    contract = "cis2-bridgeable",
    name = "view",
    return_value = "ReturnBasicState",
    error = "ContractError"
)]
fn contract_view<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ContractResult<ReturnBasicState> {
    let state = ReturnBasicState {
        paused:       host.state().paused,
        metadata_url: host.state().metadata_url.clone(),
    };
    Ok(state)
}

/// View function that returns the entire `roles` content of the state. Meant
/// for monitoring.
#[receive(
    contract = "cis2-bridgeable",
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

/// View function that returns all token owners. Meant for
/// monitoring.
#[receive(
    contract = "cis2-bridgeable",
    name = "viewTokenOwners",
    return_value = "ViewTokenOwners"
)]
fn contract_view_token_owners<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ViewTokenOwners> {
    let state = host.state();

    let token_owners = state.token.iter().map(|x| *x.0).collect();

    Ok(ViewTokenOwners { token_owners })
}

// Bridge functions

/// The parameter type for the contract function `hasRole`.
// Note: the order of the fields cannot be changed.
#[derive(Serialize, SchemaType)]
pub struct HasRoleQueryParamaters {
    pub address: Address,
    pub role:    Roles,
}

/// The response which is sent back when calling the contract function
/// `hasRole`.
#[derive(Serialize, SchemaType)]
pub struct HasRoleQueryResponse(pub bool);
impl From<bool> for HasRoleQueryResponse {
    fn from(ok: bool) -> Self { HasRoleQueryResponse(ok) }
}

/// Check if an address has a role.
/// TODO Should this be batched like the rest of the functions ?
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "cis2-bridgeable",
    name = "hasRole",
    parameter = "HasRoleQueryParamaters",
    error = "ContractError",
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
    pub role:    Roles,
}

/// Grant Permission to an address
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The sender does not have the required permission
#[receive(
    contract = "cis2-bridgeable",
    name = "grantRole",
    parameter = "GrantRoleParams",
    error = "ContractError",
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
    // Log event for grant role.
    logger.log(&BridgeableEvent::GrantRole(GrantRoleEvent {
        address: params.address,
        role:    params.role,
    }))?;
    Ok(())
}

/// The parameter type for the contract function `removeRole`.
#[derive(Serialize, SchemaType)]
pub struct RemoveRoleParams {
    pub address: Address,
    pub role:    Roles,
}

/// Remove Permission to an address
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - The sender does not have the required permission
/// - the address does not have the role
#[receive(
    contract = "cis2-bridgeable",
    name = "removeRole",
    parameter = "RemoveRoleParams",
    error = "ContractError",
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
    // Log event for revoke role.
    logger.log(&BridgeableEvent::RevokeRole(RevokeRoleEvent {
        address: params.address,
        role:    params.role,
    }))?;
    Ok(())
}

/// The parameter type for the contract function `deposit`.
#[derive(Serialize, SchemaType)]
pub struct DepositParams {
    pub address:  Address,
    pub amount:   ContractTokenAmount,
    pub token_id: TokenIdU64,
}
#[receive(
    contract = "cis2-bridgeable",
    name = "deposit",
    parameter = "DepositParams",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_deposit<S: HasStateApi>(
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
    let params: DepositParams = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, state_builder) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::Manager),
        ContractError::Unauthorized
    );

    state.mint(&TOKEN_ID, params.amount, &params.address, state_builder)?;
    // Log event for the newly minted token.
    logger.log(&BridgeableEvent::Cis2Event(Cis2Event::Mint(MintEvent {
        token_id: TOKEN_ID,
        amount:   params.amount,
        owner:    params.address,
    })))?;

    Ok(())
}

// The parameter type for the contract function `withdraw`.
#[derive(Serialize, SchemaType)]
pub struct WithdrawParams {
    pub address:  Address,
    pub amount:   ContractTokenAmount,
    pub token_id: TokenIdU64,
}
#[receive(
    contract = "cis2-bridgeable",
    name = "withdraw",
    parameter = "WithdrawParams",
    error = "ContractError",
    enable_logger,
    mutable
)]
fn contract_withdraw<S: HasStateApi>(
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
    let params: WithdrawParams = ctx.parameter_cursor().get()?;

    // Get the sender who invoked this contract function.
    let sender = ctx.sender();

    let (state, _) = host.state_and_builder();
    ensure!(
        state.has_role(&sender, Roles::Manager),
        ContractError::Unauthorized
    );
    ensure!(
        state.is_operator(&sender, &params.address),
        ContractError::Unauthorized
    );

    state.burn(&TOKEN_ID, params.amount, &params.address)?;

    // Log event for the newly burned token.
    logger.log(&BridgeableEvent::Cis2Event(Cis2Event::Burn(BurnEvent {
        token_id: TOKEN_ID,
        amount:   params.amount,
        owner:    params.address,
    })))?;

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
    const HASH: [u8; 32] = [57_u8; 32];
    const HASH2: [u8; 32] = [33_u8; 32];

    const TOKEN_METADATA_URL: &str = "https://example.com/metadata";

    fn initial_metadata() -> MetadataUrl {
        MetadataUrl {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([0u8; 32]),
        }
    }

    fn token_amount(amount: u64) -> ContractTokenAmount {
        let amount = U256::from(amount);
        ContractTokenAmount::from(amount)
    }

    /// Test helper function which creates a contract state where ADDRESS_0 owns
    /// 400 tokens.
    fn initial_state<S: HasStateApi>(state_builder: &mut StateBuilder<S>) -> State<S> {
        let mut state = State::new(state_builder, initial_metadata());
        state
            .mint(&TOKEN_ID, token_amount(400), &ADDRESS_0, state_builder)
            .expect_report("Failed to setup state");
        state.grant_role(&ADDRESS_0, Roles::Admin, state_builder);
        state
    }

    /// Test initialization succeeds and the tokens are owned by the contract
    /// instantiater and the appropriate events are logged.
    #[concordium_test]
    fn test_init() {
        let crypto: TestCryptoPrimitives = TestCryptoPrimitives::new();

        // Setup the context
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([10_u8; 32]),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        // Check the state
        claim_eq!(
            state.token.iter().count(),
            0,
            "Only one token is initialized"
        );
        let balance0 = state
            .balance(&TOKEN_ID, &ADDRESS_0)
            .expect_report("Token is expected to exist");
        claim_eq!(
            balance0,
            TOKEN_AMOUNT_ZERO,
            "No initial tokens are owned by the contract instantiater"
        );
        claim_eq!(
            state.has_role(&ADDRESS_0, Roles::Admin),
            true,
            "Initiator does not have admin"
        );
        // Check the logs
        claim_eq!(
            logger.logs.len(),
            3,
            "Exactly three events should be logged"
        );
        claim!(
            logger
                .logs
                .contains(&to_bytes(&BridgeableEvent::Cis2Event(Cis2Event::Mint(
                    MintEvent {
                        owner:    ADDRESS_0,
                        token_id: TOKEN_ID,
                        amount:   TOKEN_AMOUNT_ZERO,
                    }
                )))),
            "Missing event for minting the token"
        );

        claim!(
            logger.logs.contains(&to_bytes(&BridgeableEvent::Cis2Event(
                Cis2Event::TokenMetadata::<_, ContractTokenAmount>(TokenMetadataEvent {
                    token_id:     TOKEN_ID,
                    metadata_url: MetadataUrl {
                        url:  TOKEN_METADATA_URL.to_string(),
                        hash: Some([10_u8; 32]),
                    },
                })
            ))),
            "Missing event with metadata for the token"
        );

        claim!(
            logger
                .logs
                .contains(&to_bytes(&BridgeableEvent::GrantRole(GrantRoleEvent {
                    address: ADDRESS_0,
                    role:    Roles::Admin,
                }))),
            "Missing event for the new admin"
        );
    }

    /// Test `view_token_owners` function that returns all token owners.
    #[concordium_test]
    fn test_view_token_owners() {
        let crypto: TestCryptoPrimitives = TestCryptoPrimitives::new();

        // Setup the context
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([10_u8; 32]),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);

        // Grant ADDRESS_1 role MANAGER
        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role:    Roles::Manager,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0  is allowed to grant role");

        // Deposit 20 tokens to ADDRESS_2
        let deposit_param = DepositParams {
            address:  ADDRESS_2,
            amount:   token_amount(20),
            token_id: TokenIdU64(0),
        };
        let deposit_param_bytes = to_bytes(&deposit_param);

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&deposit_param_bytes);

        let result = contract_deposit(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_1 is allowed to deposit");

        // Deposit 10 tokens to ADDRESS_1
        let deposit_param = DepositParams {
            address:  ADDRESS_1,
            amount:   token_amount(10),
            token_id: TokenIdU64(0),
        };
        let deposit_param_bytes = to_bytes(&deposit_param);

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&deposit_param_bytes);
        logger.logs.clear();
        let result = contract_deposit(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_1 is allowed to deposit");

        // Check `view_token_owners` function that returns all token owners.
        let view_token_owners_result = contract_view_token_owners(&ctx, &host);

        let token_owners = view_token_owners_result
            .expect_report("Calling contract_view_token_owners expected to succeed")
            .token_owners;

        // Check the view_token_owners_result
        claim_eq!(
            token_owners.len(),
            2,
            "Exactly 2 accounts should be included in the state"
        );
        claim_eq!(
            token_owners[0],
            concordium_std::Address::Account(ACCOUNT_1),
            "ACCOUNT_1 should be a token owner."
        );
        claim_eq!(
            token_owners[1],
            concordium_std::Address::Account(ACCOUNT_2),
            "ACCOUNT_2 should be a token owner."
        );
    }

    /// Test `view_roles` function displays the `roles` content of the state.
    /// Add the ADMIN and MANAGER role to ACCOUNT_0 and the MANAGER role to
    /// ACCOUNT_1.
    #[concordium_test]
    fn test_view_roles() {
        let crypto: TestCryptoPrimitives = TestCryptoPrimitives::new();

        // Setup the context
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);
        let mut logger = TestLogger::init();

        let mut builder = TestStateBuilder::new();

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([10_u8; 32]),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let mut host = TestHost::new(state, builder);
        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role:    Roles::Manager,
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
            role:    Roles::Manager,
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

        let roles = roles_result.expect_report("Calling contract_view_roles expected to succeed");

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
                    roles: vec![Roles::Admin, Roles::Manager],
                }
            ),
            "ACCOUNT_0 should have the roles Admin and Manager"
        );
        claim_eq!(
            roles.all_roles[1],
            (
                concordium_std::Address::Account(ACCOUNT_1),
                ViewRolesState {
                    roles: vec![Roles::Manager],
                }
            ),
            "ACCOUNT_1 should have the role Manager"
        );
    }

    /// Test initial token_metadata_url is set
    /// Change token_metadata_url
    #[concordium_test]
    fn test_set_metadata_url() {
        let crypto: TestCryptoPrimitives = TestCryptoPrimitives::new();

        // Setup the context
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some(HASH),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut builder = TestStateBuilder::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        // Check the state
        claim_eq!(
            state.metadata_url.url,
            TOKEN_METADATA_URL,
            "Token metadata url is not matching"
        );

        claim_eq!(
            state.metadata_url.hash.is_some(),
            true,
            "Token metadata hash is missing"
        );

        claim_eq!(
            state.metadata_url.hash.unwrap(),
            HASH,
            "Token metadata hash is not matching"
        );

        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_1);
        // and parameter.
        let new_metadata_url = "https://example.com/new/metadata/params";
        let change_params = SetMetadataUrlParams {
            url:  new_metadata_url.to_string(),
            hash: Some(HASH2),
        };
        let parameter_bytes = to_bytes(&change_params);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let mut state = initial_state(&mut state_builder);
        state.grant_role(&ADDRESS_1, Roles::Admin, &mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> =
            contract_set_token_metadata_url(&ctx, &mut host, &mut logger);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check the state.
        let metadata_url = host.state().metadata_url.url.clone();
        let metadata_hash = host.state().metadata_url.hash;

        claim_eq!(
            metadata_url,
            *new_metadata_url,
            "Token metadata url doesn't match"
        );

        claim_eq!(
            metadata_hash.is_some(),
            true,
            "Token metadata hash is missing"
        );

        claim_eq!(
            metadata_hash.unwrap(),
            HASH2,
            "Token metadata hash is not matching"
        );

        claim!(
            logger.logs.contains(&to_bytes(
                &Cis2Event::TokenMetadata::<_, ContractTokenAmount>(TokenMetadataEvent {
                    token_id:     TOKEN_ID,
                    metadata_url: MetadataUrl {
                        url:  new_metadata_url.to_string(),
                        hash: Some(HASH2),
                    },
                })
            )),
            "Missing event with metadata for the token"
        );
    }

    /// Test transfer succeeds, when `from` is the sender.
    #[concordium_test]
    fn test_transfer_account() {
        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);

        // and parameter.
        let transfer = Transfer {
            token_id: TOKEN_ID,
            amount:   token_amount(100),
            from:     ADDRESS_0,
            to:       Receiver::from_account(ACCOUNT_1),
            data:     AdditionalData::empty(),
        };
        let parameter = TransferParams::from(vec![transfer]);
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let mut state = State::new(&mut state_builder, initial_metadata());
        state
            .mint(&TOKEN_ID, token_amount(400), &ADDRESS_0, &mut state_builder)
            .expect_report("Failed to setup state");
        let mut host = TestHost::new(state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = contract_transfer(&ctx, &mut host, &mut logger);
        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check the state.
        let balance0 = host
            .state()
            .balance(&TOKEN_ID, &ADDRESS_0)
            .expect_report("Token is expected to exist");
        let balance1 = host
            .state()
            .balance(&TOKEN_ID, &ADDRESS_1)
            .expect_report("Token is expected to exist");
        claim_eq!(
            balance0,
            token_amount(300),
            "Token owner balance should be decreased by the transferred amount"
        );
        claim_eq!(
            balance1,
            token_amount(100),
            "Token receiver balance should be increased by the transferred amount"
        );

        // Check the logs.
        claim_eq!(logger.logs.len(), 1, "Only one event should be logged");
        claim_eq!(
            logger.logs[0],
            to_bytes(&Cis2Event::Transfer(TransferEvent {
                from:     ADDRESS_0,
                to:       ADDRESS_1,
                token_id: TOKEN_ID,
                amount:   token_amount(100),
            })),
            "Incorrect event emitted"
        )
    }

    /// Test transfer token fails, when sender is neither the owner or an
    /// operator of the owner.
    #[concordium_test]
    fn test_transfer_not_authorized() {
        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_1);

        // and parameter.
        let transfer = Transfer {
            from:     ADDRESS_0,
            to:       Receiver::from_account(ACCOUNT_1),
            token_id: TOKEN_ID,
            amount:   token_amount(100),
            data:     AdditionalData::empty(),
        };
        let parameter = TransferParams::from(vec![transfer]);
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = contract_transfer(&ctx, &mut host, &mut logger);
        // Check the result.
        let err = result.expect_err_report("Expected to fail");
        claim_eq!(
            err,
            ContractError::Unauthorized,
            "Error is expected to be Unauthorized"
        )
    }

    /// Test transfer succeeds when sender is not the owner, but is an operator
    /// of the owner.
    #[concordium_test]
    fn test_operator_transfer() {
        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_1);

        // and parameter.
        let transfer = Transfer {
            from:     ADDRESS_0,
            to:       Receiver::from_account(ACCOUNT_1),
            token_id: TOKEN_ID,
            amount:   token_amount(100),
            data:     AdditionalData::empty(),
        };
        let parameter = TransferParams::from(vec![transfer]);
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let mut state = initial_state(&mut state_builder);
        state.add_operator(&ADDRESS_0, &ADDRESS_1, &mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = contract_transfer(&ctx, &mut host, &mut logger);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check the state.
        let balance0 = host
            .state()
            .balance(&TOKEN_ID, &ADDRESS_0)
            .expect_report("Token is expected to exist");
        let balance1 = host
            .state()
            .balance(&TOKEN_ID, &ADDRESS_1)
            .expect_report("Token is expected to exist");
        claim_eq!(balance0, token_amount(300)); //, "Token owner balance should be decreased by the transferred amount");
        claim_eq!(
            balance1,
            token_amount(100),
            "Token receiver balance should be increased by the transferred amount"
        );

        // Check the logs.
        claim_eq!(logger.logs.len(), 1, "Only one event should be logged");
        claim_eq!(
            logger.logs[0],
            to_bytes(&Cis2Event::Transfer(TransferEvent {
                from:     ADDRESS_0,
                to:       ADDRESS_1,
                token_id: TOKEN_ID,
                amount:   token_amount(100),
            })),
            "Incorrect event emitted"
        )
    }

    /// Test adding an operator succeeds and the appropriate event is logged.
    #[concordium_test]
    fn test_add_operator() {
        // Setup the context
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);

        // and parameter.
        let update = UpdateOperator {
            operator: ADDRESS_1,
            update:   OperatorUpdate::Add,
        };
        let parameter = UpdateOperatorParams(vec![update]);
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = contract_update_operator(&ctx, &mut host, &mut logger);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check the state.
        claim!(
            host.state().is_operator(&ADDRESS_1, &ADDRESS_0),
            "Account should be an operator"
        );

        // Check the logs.
        claim_eq!(logger.logs.len(), 1, "One event should be logged");
        claim_eq!(
            logger.logs[0],
            to_bytes(
                &Cis2Event::<ContractTokenId, ContractTokenAmount>::UpdateOperator(
                    UpdateOperatorEvent {
                        owner:    ADDRESS_0,
                        operator: ADDRESS_1,
                        update:   OperatorUpdate::Add,
                    }
                )
            ),
            "Incorrect event emitted"
        )
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
            module:  new_module_ref,
            migrate: Some((migration_entrypoint.clone(), OwnedParameter(Vec::new()))),
        };
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

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
            module:  new_module_ref,
            migrate: None,
        };
        let parameter_bytes = to_bytes(&parameter);
        ctx.set_parameter(&parameter_bytes);

        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        host.setup_mock_upgrade(new_module_ref, Err(UpgradeError::MissingModule));

        let result: ContractResult<()> = contract_upgrade(&ctx, &mut host);

        claim_eq!(
            result,
            Err(ContractError::Custom(
                CustomContractError::FailedUpgradeMissingModule
            ))
        );
    }
    /// Test adding an operator succeeds and the appropriate event is logged.
    #[concordium_test]
    fn test_roles() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([0; 32]),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut builder = TestStateBuilder::new();
        let crypto = TestCryptoPrimitives::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");

        let state_builder = TestStateBuilder::new();
        let mut host = TestHost::new(state, state_builder);

        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role:    Roles::Manager,
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

        let query = HasRoleQueryParamaters {
            address: ADDRESS_1,
            role:    Roles::Manager,
        };
        let query_bytes = to_bytes(&query);

        ctx.set_parameter(&query_bytes);

        let has_role_result = contract_has_role(&ctx, &host);
        claim!(has_role_result.is_ok(), "has role error");
        claim!(has_role_result.unwrap().0, "ADDRESS1 has manager role");

        // Remove role
        let parameter = RemoveRoleParams {
            address: ADDRESS_1,
            role:    Roles::Manager,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        result = contract_remove_role(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_0 is allowed to remove role");

        // Check has role again
        let query = HasRoleQueryParamaters {
            address: ADDRESS_1,
            role:    Roles::Manager,
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
            role:    Roles::Manager,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();
        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        result = contract_remove_role(&ctx, &mut host, &mut logger);
        claim!(result.is_err(), "ADDRESS_2 does not have role");
    }

    /// Test adding an operator succeeds and the appropriate event is logged.
    #[concordium_test]
    fn test_deposit() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([0; 32]),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut builder = TestStateBuilder::new();
        let crypto = TestCryptoPrimitives::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");
        let state_builder = TestStateBuilder::new();
        let mut host = TestHost::new(state, state_builder);

        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role:    Roles::Manager,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let mut result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0  is allowed to grant role");

        let deposit_param = DepositParams {
            address:  ADDRESS_2,
            amount:   token_amount(20),
            token_id: TokenIdU64(0),
        };
        let deposit_param_bytes = to_bytes(&deposit_param);
        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&deposit_param_bytes);
        result = contract_deposit(&ctx, &mut host, &mut logger);
        claim!(result.is_err(), "ADDRESS_0 not allowed to deposit");

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&deposit_param_bytes);
        logger.logs.clear();
        result = contract_deposit(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_1 is allowed to deposit");

        // Check Balances
        let balance = host
            .state()
            .balance(&TOKEN_ID, &ADDRESS_2)
            .expect_report("Token is expected to exist");

        claim_eq!(balance, token_amount(20));

        // Check the logs.
        claim_eq!(logger.logs.len(), 1, "Only one event should be logged");
        claim_eq!(
            logger.logs[0],
            to_bytes(&Cis2Event::Mint(MintEvent {
                owner:    ADDRESS_2,
                token_id: TOKEN_ID,
                amount:   token_amount(20),
            })),
            "Incorrect event emitted"
        )
    }

    /// Test adding an operator succeeds and the appropriate event is logged.
    #[concordium_test]
    fn test_withdraw() {
        let mut ctx = TestInitContext::empty();
        ctx.set_init_origin(ACCOUNT_0);

        // and parameter.
        let init_params = SetMetadataUrlParams {
            url:  TOKEN_METADATA_URL.to_string(),
            hash: Some([0; 32]),
        };
        let parameter_bytes = to_bytes(&init_params);
        ctx.set_parameter(&parameter_bytes);

        let mut logger = TestLogger::init();
        let mut builder = TestStateBuilder::new();
        let crypto = TestCryptoPrimitives::new();

        // Call the contract function.
        let result = contract_init(&ctx, &mut builder, &mut logger, &crypto);

        // Check the result
        let state = result.expect_report("Contract initialization failed");
        let state_builder = TestStateBuilder::new();
        let mut host = TestHost::new(state, state_builder);

        let parameter = GrantRoleParams {
            address: ADDRESS_1,
            role:    Roles::Manager,
        };
        let parameter_bytes = to_bytes(&parameter);
        let mut ctx = TestReceiveContext::empty();

        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&parameter_bytes);
        let mut result: ContractResult<()> = contract_grant_role(&ctx, &mut host, &mut logger);

        claim!(result.is_ok(), "ADDRESS_0  is allowed to grant role");

        let deposit_param = DepositParams {
            address:  ADDRESS_2,
            amount:   token_amount(20),
            token_id: TokenIdU64(0),
        };
        let deposit_param_bytes = to_bytes(&deposit_param);

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&deposit_param_bytes);
        result = contract_deposit(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_1 is allowed to deposit");

        let withdraw_param = WithdrawParams {
            token_id: TokenIdU64(0),
            address:  ADDRESS_2,
            amount:   token_amount(9),
        };
        let withdraw_param_bytes = to_bytes(&withdraw_param);
        ctx.set_sender(ADDRESS_0);
        ctx.set_parameter(&withdraw_param_bytes);

        result = contract_withdraw(&ctx, &mut host, &mut logger);
        claim!(result.is_err(), "ADDRESS_0 not allowed to withdraw");

        ctx.set_sender(ADDRESS_1);
        ctx.set_parameter(&withdraw_param_bytes);

        logger.logs.clear();
        result = contract_withdraw(&ctx, &mut host, &mut logger);
        claim!(result.is_ok(), "ADDRESS_1 is allowed to withdraw");
        // Check Balances
        let balance = host
            .state()
            .balance(&TOKEN_ID, &ADDRESS_2)
            .expect_report("Token is expected to exist");

        claim_eq!(balance, token_amount(11));

        // Check the logs.
        claim_eq!(logger.logs.len(), 1, "Only one event should be logged");
        claim_eq!(
            logger.logs[0],
            to_bytes(&Cis2Event::Burn(BurnEvent {
                owner:    ADDRESS_2,
                token_id: TOKEN_ID,
                amount:   token_amount(9),
            })),
            "Incorrect event emitted"
        )
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
        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

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
        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

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
        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

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
        let mut state_builder = TestStateBuilder::new();
        let state = initial_state(&mut state_builder);
        let mut host = TestHost::new(state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = contract_set_paused(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");

        // Check contract is paused.
        claim_eq!(host.state().paused, true, "Smart contract should be paused");

        let mut logger = TestLogger::init();

        // Call the `transfer` function.
        let result: ContractResult<()> = contract_transfer(&ctx, &mut host, &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(CustomContractError::ContractPaused)),
            "Transfer should fail because contract is paused"
        );

        // Call the `updateOperator` function.
        let result: ContractResult<()> = contract_update_operator(&ctx, &mut host, &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(CustomContractError::ContractPaused)),
            "Update operator should fail because contract is paused"
        );

        // Call the `deposit` function.
        let result: ContractResult<()> = contract_deposit(&ctx, &mut host, &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(CustomContractError::ContractPaused)),
            "Wrap should fail because contract is paused"
        );

        // Call the`unwrap` function.
        let result: ContractResult<()> = contract_withdraw(&ctx, &mut host, &mut logger);

        // Check that invoke failed.
        claim_eq!(
            result,
            Err(ContractError::Custom(CustomContractError::ContractPaused)),
            "Unwrap should fail because contract is paused"
        );
    }
}
