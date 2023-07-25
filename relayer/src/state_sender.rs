pub use state_sender::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod state_sender {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::Middleware,
    };
    /// StateSender was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"version\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"Initialized\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"depositor\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"depositReceiver\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"depositData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"LockedToken\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"root\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"MerkleRoot\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"previousAdminRole\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"newAdminRole\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"RoleAdminChanged\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleGranted\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleRevoked\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"TokenMapAdded\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"TokenMapRemoved\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"vaultAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"VaultRegistered\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint64\",\n        \"name\": \"ccdIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint64\",\n        \"name\": \"ccdSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"userWallet\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"ccdTxHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"ccdEventIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint64\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"WithdrawEvent\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DEFAULT_ADMIN_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"EMITTER_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"userCcd\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"depositData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"emitDeposit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"merkleRoot\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"emitMerkleRoot\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"emitTokenMapAdd\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"emitTokenMapRemove\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vaultAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"emitVaultRegistered\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"ccdIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"ccdSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"userWallet\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ccdTxHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"ccdEventIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"emitWithdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getRoleAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasRole\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"renounceRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"interfaceId\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"supportsInterface\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n" ;
    /// The parsed JSON-ABI of the contract.
    pub static STATESENDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct StateSender<M>(ethers::contract::Contract<M>);
    impl<M> Clone for StateSender<M> {
        fn clone(&self) -> Self { StateSender(self.0.clone()) }
    }
    impl<M> std::ops::Deref for StateSender<M> {
        type Target = ethers::contract::Contract<M>;

        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> std::fmt::Debug for StateSender<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(StateSender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> StateSender<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a
        /// `ethers::Contract`
        /// object
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), STATESENDER_ABI.clone(), client).into()
        }

        /// Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `EMITTER_ROLE` (0x18004b39) function
        pub fn emitter_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([24, 0, 75, 57], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `emitDeposit` (0x3fc3294a) function
        pub fn emit_deposit(
            &self,
            user: ethers::core::types::Address,
            user_ccd: [u8; 32],
            root_token: ethers::core::types::Address,
            vault: ethers::core::types::Address,
            deposit_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [63, 195, 41, 74],
                    (user, user_ccd, root_token, vault, deposit_data),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `emitMerkleRoot` (0x38835bcb) function
        pub fn emit_merkle_root(
            &self,
            merkle_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 131, 91, 203], merkle_root)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `emitTokenMapAdd` (0x38745f16) function
        pub fn emit_token_map_add(
            &self,
            root_token: ethers::core::types::Address,
            child_token_index: u64,
            child_token_sub_index: u64,
            token_type: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [56, 116, 95, 22],
                    (
                        root_token,
                        child_token_index,
                        child_token_sub_index,
                        token_type,
                    ),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `emitTokenMapRemove` (0x97131fbe) function
        pub fn emit_token_map_remove(
            &self,
            root_token: ethers::core::types::Address,
            child_token_index: u64,
            child_token_sub_index: u64,
            token_type: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [151, 19, 31, 190],
                    (
                        root_token,
                        child_token_index,
                        child_token_sub_index,
                        token_type,
                    ),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `emitVaultRegistered` (0x3001397d) function
        pub fn emit_vault_registered(
            &self,
            token_type: [u8; 32],
            vault_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 1, 57, 125], (token_type, vault_address))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `emitWithdraw` (0x325f4a4a) function
        pub fn emit_withdraw(
            &self,
            ccd_index: u64,
            ccd_sub_index: u64,
            amount: ethers::core::types::U256,
            user_wallet: ethers::core::types::Address,
            ccd_tx_hash: [u8; 32],
            ccd_event_index: u64,
            token_id: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [50, 95, 74, 74],
                    (
                        ccd_index,
                        ccd_sub_index,
                        amount,
                        user_wallet,
                        ccd_tx_hash,
                        ccd_event_index,
                        token_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], owner)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }

        /// Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }

        /// Gets the contract's `LockedToken` event
        pub fn locked_token_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LockedTokenFilter> {
            self.0.event()
        }

        /// Gets the contract's `MerkleRoot` event
        pub fn merkle_root_filter(&self) -> ethers::contract::builders::Event<M, MerkleRootFilter> {
            self.0.event()
        }

        /// Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }

        /// Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }

        /// Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }

        /// Gets the contract's `TokenMapAdded` event
        pub fn token_map_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenMapAddedFilter> {
            self.0.event()
        }

        /// Gets the contract's `TokenMapRemoved` event
        pub fn token_map_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenMapRemovedFilter> {
            self.0.event()
        }

        /// Gets the contract's `VaultRegistered` event
        pub fn vault_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VaultRegisteredFilter> {
            self.0.event()
        }

        /// Gets the contract's `WithdrawEvent` event
        pub fn withdraw_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawEventFilter> {
            self.0.event()
        }

        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for
        /// all events of this contract
        pub fn events(&self) -> ethers::contract::builders::Event<M, StateSenderEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for StateSender<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self { Self(contract) }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "LockedToken",
        abi = "LockedToken(uint256,address,bytes32,address,address,bytes)"
    )]
    pub struct LockedTokenFilter {
        pub id:               ethers::core::types::U256,
        #[ethevent(indexed)]
        pub depositor:        ethers::core::types::Address,
        pub deposit_receiver: [u8; 32],
        #[ethevent(indexed)]
        pub root_token:       ethers::core::types::Address,
        #[ethevent(indexed)]
        pub vault:            ethers::core::types::Address,
        pub deposit_data:     ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "MerkleRoot", abi = "MerkleRoot(uint256,bytes32)")]
    pub struct MerkleRootFilter {
        pub id:   ethers::core::types::U256,
        pub root: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role:                [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role:      [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role:    [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender:  ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role:    [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender:  ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TokenMapAdded",
        abi = "TokenMapAdded(uint256,address,uint64,uint64,bytes32)"
    )]
    pub struct TokenMapAddedFilter {
        pub id:                    ethers::core::types::U256,
        #[ethevent(indexed)]
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
        #[ethevent(indexed)]
        pub token_type:            [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TokenMapRemoved",
        abi = "TokenMapRemoved(uint256,address,uint64,uint64,bytes32)"
    )]
    pub struct TokenMapRemovedFilter {
        pub id:                    ethers::core::types::U256,
        #[ethevent(indexed)]
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
        #[ethevent(indexed)]
        pub token_type:            [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "VaultRegistered",
        abi = "VaultRegistered(uint256,bytes32,address)"
    )]
    pub struct VaultRegisteredFilter {
        pub id:            ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_type:    [u8; 32],
        #[ethevent(indexed)]
        pub vault_address: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "WithdrawEvent",
        abi = "WithdrawEvent(uint256,uint64,uint64,uint256,address,bytes32,uint64,uint64)"
    )]
    pub struct WithdrawEventFilter {
        pub id:              ethers::core::types::U256,
        #[ethevent(indexed)]
        pub ccd_index:       u64,
        #[ethevent(indexed)]
        pub ccd_sub_index:   u64,
        pub amount:          ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user_wallet:     ethers::core::types::Address,
        pub ccd_tx_hash:     [u8; 32],
        pub ccd_event_index: u64,
        pub token_id:        u64,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StateSenderEvents {
        InitializedFilter(InitializedFilter),
        LockedTokenFilter(LockedTokenFilter),
        MerkleRootFilter(MerkleRootFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TokenMapAddedFilter(TokenMapAddedFilter),
        TokenMapRemovedFilter(TokenMapRemovedFilter),
        VaultRegisteredFilter(VaultRegisteredFilter),
        WithdrawEventFilter(WithdrawEventFilter),
    }
    impl ethers::contract::EthLogDecode for StateSenderEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized, {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StateSenderEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LockedTokenFilter::decode_log(log) {
                return Ok(StateSenderEvents::LockedTokenFilter(decoded));
            }
            if let Ok(decoded) = MerkleRootFilter::decode_log(log) {
                return Ok(StateSenderEvents::MerkleRootFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(StateSenderEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(StateSenderEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(StateSenderEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = TokenMapAddedFilter::decode_log(log) {
                return Ok(StateSenderEvents::TokenMapAddedFilter(decoded));
            }
            if let Ok(decoded) = TokenMapRemovedFilter::decode_log(log) {
                return Ok(StateSenderEvents::TokenMapRemovedFilter(decoded));
            }
            if let Ok(decoded) = VaultRegisteredFilter::decode_log(log) {
                return Ok(StateSenderEvents::VaultRegisteredFilter(decoded));
            }
            if let Ok(decoded) = WithdrawEventFilter::decode_log(log) {
                return Ok(StateSenderEvents::WithdrawEventFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for StateSenderEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StateSenderEvents::InitializedFilter(element) => element.fmt(f),
                StateSenderEvents::LockedTokenFilter(element) => element.fmt(f),
                StateSenderEvents::MerkleRootFilter(element) => element.fmt(f),
                StateSenderEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                StateSenderEvents::RoleGrantedFilter(element) => element.fmt(f),
                StateSenderEvents::RoleRevokedFilter(element) => element.fmt(f),
                StateSenderEvents::TokenMapAddedFilter(element) => element.fmt(f),
                StateSenderEvents::TokenMapRemovedFilter(element) => element.fmt(f),
                StateSenderEvents::VaultRegisteredFilter(element) => element.fmt(f),
                StateSenderEvents::WithdrawEventFilter(element) => element.fmt(f),
            }
        }
    }
    /// Container type for all input parameters for the `DEFAULT_ADMIN_ROLE`
    /// function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23,
    /// 253, 223]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    /// Container type for all input parameters for the `EMITTER_ROLE` function
    /// with signature `EMITTER_ROLE()` and selector `[24, 0, 75, 57]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "EMITTER_ROLE", abi = "EMITTER_ROLE()")]
    pub struct EmitterRoleCall;
    /// Container type for all input parameters for the `emitDeposit` function
    /// with signature `emitDeposit(address,bytes32,address,address,bytes)` and
    /// selector `[63, 195, 41, 74]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "emitDeposit",
        abi = "emitDeposit(address,bytes32,address,address,bytes)"
    )]
    pub struct EmitDepositCall {
        pub user:         ethers::core::types::Address,
        pub user_ccd:     [u8; 32],
        pub root_token:   ethers::core::types::Address,
        pub vault:        ethers::core::types::Address,
        pub deposit_data: ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `emitMerkleRoot`
    /// function with signature `emitMerkleRoot(bytes32)` and selector `[56,
    /// 131, 91, 203]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "emitMerkleRoot", abi = "emitMerkleRoot(bytes32)")]
    pub struct EmitMerkleRootCall {
        pub merkle_root: [u8; 32],
    }
    /// Container type for all input parameters for the `emitTokenMapAdd`
    /// function with signature `emitTokenMapAdd(address,uint64,uint64,bytes32)`
    /// and selector `[56, 116, 95, 22]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "emitTokenMapAdd",
        abi = "emitTokenMapAdd(address,uint64,uint64,bytes32)"
    )]
    pub struct EmitTokenMapAddCall {
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
        pub token_type:            [u8; 32],
    }
    /// Container type for all input parameters for the `emitTokenMapRemove`
    /// function with signature
    /// `emitTokenMapRemove(address,uint64,uint64,bytes32)` and selector `[151,
    /// 19, 31, 190]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "emitTokenMapRemove",
        abi = "emitTokenMapRemove(address,uint64,uint64,bytes32)"
    )]
    pub struct EmitTokenMapRemoveCall {
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
        pub token_type:            [u8; 32],
    }
    /// Container type for all input parameters for the `emitVaultRegistered`
    /// function with signature `emitVaultRegistered(bytes32,address)` and
    /// selector `[48, 1, 57, 125]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "emitVaultRegistered",
        abi = "emitVaultRegistered(bytes32,address)"
    )]
    pub struct EmitVaultRegisteredCall {
        pub token_type:    [u8; 32],
        pub vault_address: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `emitWithdraw` function
    /// with signature
    /// `emitWithdraw(uint64,uint64,uint256,address,bytes32,uint64,uint64)` and
    /// selector `[50, 95, 74, 74]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "emitWithdraw",
        abi = "emitWithdraw(uint64,uint64,uint256,address,bytes32,uint64,uint64)"
    )]
    pub struct EmitWithdrawCall {
        pub ccd_index:       u64,
        pub ccd_sub_index:   u64,
        pub amount:          ethers::core::types::U256,
        pub user_wallet:     ethers::core::types::Address,
        pub ccd_tx_hash:     [u8; 32],
        pub ccd_event_index: u64,
        pub token_id:        u64,
    }
    /// Container type for all input parameters for the `getRoleAdmin` function
    /// with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156,
    /// 163]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    /// Container type for all input parameters for the `grantRole` function
    /// with signature `grantRole(bytes32,address)` and selector `[47, 47, 241,
    /// 93]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role:    [u8; 32],
        pub account: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `hasRole` function with
    /// signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role:    [u8; 32],
        pub account: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `initialize` function
    /// with signature `initialize(address)` and selector `[196, 214, 109, 232]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub owner: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `renounceRole` function
    /// with signature `renounceRole(bytes32,address)` and selector `[54, 86,
    /// 138, 190]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role:    [u8; 32],
        pub account: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `revokeRole` function
    /// with signature `revokeRole(bytes32,address)` and selector `[213, 71,
    /// 116, 31]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role:    [u8; 32],
        pub account: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `supportsInterface`
    /// function with signature `supportsInterface(bytes4)` and selector `[1,
    /// 255, 201, 167]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StateSenderCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EmitterRole(EmitterRoleCall),
        EmitDeposit(EmitDepositCall),
        EmitMerkleRoot(EmitMerkleRootCall),
        EmitTokenMapAdd(EmitTokenMapAddCall),
        EmitTokenMapRemove(EmitTokenMapRemoveCall),
        EmitVaultRegistered(EmitVaultRegisteredCall),
        EmitWithdraw(EmitWithdrawCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Initialize(InitializeCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ethers::core::abi::AbiDecode for StateSenderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <EmitterRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitterRole(decoded));
            }
            if let Ok(decoded) =
                <EmitDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitDeposit(decoded));
            }
            if let Ok(decoded) =
                <EmitMerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitMerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <EmitTokenMapAddCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitTokenMapAdd(decoded));
            }
            if let Ok(decoded) =
                <EmitTokenMapRemoveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitTokenMapRemove(decoded));
            }
            if let Ok(decoded) =
                <EmitVaultRegisteredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitVaultRegistered(decoded));
            }
            if let Ok(decoded) =
                <EmitWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::EmitWithdraw(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StateSenderCalls::SupportsInterface(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for StateSenderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                StateSenderCalls::DefaultAdminRole(element) => element.encode(),
                StateSenderCalls::EmitterRole(element) => element.encode(),
                StateSenderCalls::EmitDeposit(element) => element.encode(),
                StateSenderCalls::EmitMerkleRoot(element) => element.encode(),
                StateSenderCalls::EmitTokenMapAdd(element) => element.encode(),
                StateSenderCalls::EmitTokenMapRemove(element) => element.encode(),
                StateSenderCalls::EmitVaultRegistered(element) => element.encode(),
                StateSenderCalls::EmitWithdraw(element) => element.encode(),
                StateSenderCalls::GetRoleAdmin(element) => element.encode(),
                StateSenderCalls::GrantRole(element) => element.encode(),
                StateSenderCalls::HasRole(element) => element.encode(),
                StateSenderCalls::Initialize(element) => element.encode(),
                StateSenderCalls::RenounceRole(element) => element.encode(),
                StateSenderCalls::RevokeRole(element) => element.encode(),
                StateSenderCalls::SupportsInterface(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for StateSenderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StateSenderCalls::DefaultAdminRole(element) => element.fmt(f),
                StateSenderCalls::EmitterRole(element) => element.fmt(f),
                StateSenderCalls::EmitDeposit(element) => element.fmt(f),
                StateSenderCalls::EmitMerkleRoot(element) => element.fmt(f),
                StateSenderCalls::EmitTokenMapAdd(element) => element.fmt(f),
                StateSenderCalls::EmitTokenMapRemove(element) => element.fmt(f),
                StateSenderCalls::EmitVaultRegistered(element) => element.fmt(f),
                StateSenderCalls::EmitWithdraw(element) => element.fmt(f),
                StateSenderCalls::GetRoleAdmin(element) => element.fmt(f),
                StateSenderCalls::GrantRole(element) => element.fmt(f),
                StateSenderCalls::HasRole(element) => element.fmt(f),
                StateSenderCalls::Initialize(element) => element.fmt(f),
                StateSenderCalls::RenounceRole(element) => element.fmt(f),
                StateSenderCalls::RevokeRole(element) => element.fmt(f),
                StateSenderCalls::SupportsInterface(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for StateSenderCalls {
        fn from(var: DefaultAdminRoleCall) -> Self { StateSenderCalls::DefaultAdminRole(var) }
    }
    impl ::std::convert::From<EmitterRoleCall> for StateSenderCalls {
        fn from(var: EmitterRoleCall) -> Self { StateSenderCalls::EmitterRole(var) }
    }
    impl ::std::convert::From<EmitDepositCall> for StateSenderCalls {
        fn from(var: EmitDepositCall) -> Self { StateSenderCalls::EmitDeposit(var) }
    }
    impl ::std::convert::From<EmitMerkleRootCall> for StateSenderCalls {
        fn from(var: EmitMerkleRootCall) -> Self { StateSenderCalls::EmitMerkleRoot(var) }
    }
    impl ::std::convert::From<EmitTokenMapAddCall> for StateSenderCalls {
        fn from(var: EmitTokenMapAddCall) -> Self { StateSenderCalls::EmitTokenMapAdd(var) }
    }
    impl ::std::convert::From<EmitTokenMapRemoveCall> for StateSenderCalls {
        fn from(var: EmitTokenMapRemoveCall) -> Self { StateSenderCalls::EmitTokenMapRemove(var) }
    }
    impl ::std::convert::From<EmitVaultRegisteredCall> for StateSenderCalls {
        fn from(var: EmitVaultRegisteredCall) -> Self { StateSenderCalls::EmitVaultRegistered(var) }
    }
    impl ::std::convert::From<EmitWithdrawCall> for StateSenderCalls {
        fn from(var: EmitWithdrawCall) -> Self { StateSenderCalls::EmitWithdraw(var) }
    }
    impl ::std::convert::From<GetRoleAdminCall> for StateSenderCalls {
        fn from(var: GetRoleAdminCall) -> Self { StateSenderCalls::GetRoleAdmin(var) }
    }
    impl ::std::convert::From<GrantRoleCall> for StateSenderCalls {
        fn from(var: GrantRoleCall) -> Self { StateSenderCalls::GrantRole(var) }
    }
    impl ::std::convert::From<HasRoleCall> for StateSenderCalls {
        fn from(var: HasRoleCall) -> Self { StateSenderCalls::HasRole(var) }
    }
    impl ::std::convert::From<InitializeCall> for StateSenderCalls {
        fn from(var: InitializeCall) -> Self { StateSenderCalls::Initialize(var) }
    }
    impl ::std::convert::From<RenounceRoleCall> for StateSenderCalls {
        fn from(var: RenounceRoleCall) -> Self { StateSenderCalls::RenounceRole(var) }
    }
    impl ::std::convert::From<RevokeRoleCall> for StateSenderCalls {
        fn from(var: RevokeRoleCall) -> Self { StateSenderCalls::RevokeRole(var) }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for StateSenderCalls {
        fn from(var: SupportsInterfaceCall) -> Self { StateSenderCalls::SupportsInterface(var) }
    }
    /// Container type for all return fields from the `DEFAULT_ADMIN_ROLE`
    /// function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23,
    /// 253, 223]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    /// Container type for all return fields from the `EMITTER_ROLE` function
    /// with signature `EMITTER_ROLE()` and selector `[24, 0, 75, 57]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct EmitterRoleReturn(pub [u8; 32]);
    /// Container type for all return fields from the `getRoleAdmin` function
    /// with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156,
    /// 163]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    /// Container type for all return fields from the `hasRole` function with
    /// signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasRoleReturn(pub bool);
    /// Container type for all return fields from the `supportsInterface`
    /// function with signature `supportsInterface(bytes4)` and selector `[1,
    /// 255, 201, 167]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
