pub use bridge_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod bridge_manager {
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
    /// BridgeManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"previousAdminRole\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"newAdminRole\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"RoleAdminChanged\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleGranted\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleRevoked\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DEFAULT_ADMIN_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ETHER_ADDRESS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"MAPPER_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"MERKLE_UPDATER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"childToRootToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"cleanMapToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ccdUser\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"depositEtherFor\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ccdUser\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"depositData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"depositFor\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getMerkleRoot\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getRoleAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasRole\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"name\": \"hashChild\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"mapToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"processedExits\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vaultAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"registerVault\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rootToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"childTokenSubIndex\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"tokenType\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"remapToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"renounceRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"rootToChildToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"index\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"subindex\",\n        \"type\": \"uint64\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_merkleRoot\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"setMerkleRoot\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newStateSender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setStateSender\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"stateSenderAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"interfaceId\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"supportsInterface\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"tokenToType\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"typeToVault\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"ccdIndex\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"ccdSubIndex\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"amount\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"userWallet\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"ccdTxHash\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"ccdEventIndex\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"tokenId\",\n            \"type\": \"uint64\"\n          }\n        ],\n        \"internalType\": \"struct IRootChainManager.WithdrawParams\",\n        \"name\": \"withdraw\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"proof\",\n        \"type\": \"bytes32[]\"\n      }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]\n" ;
    /// The parsed JSON-ABI of the contract.
    pub static BRIDGEMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct BridgeManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BridgeManager<M> {
        fn clone(&self) -> Self { BridgeManager(self.0.clone()) }
    }
    impl<M> std::ops::Deref for BridgeManager<M> {
        type Target = ethers::contract::Contract<M>;

        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<M> std::fmt::Debug for BridgeManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BridgeManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BridgeManager<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a
        /// `ethers::Contract`
        /// object
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BRIDGEMANAGER_ABI.clone(), client)
                .into()
        }

        /// Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `ETHER_ADDRESS` (0xcf1d21c0) function
        pub fn ether_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 29, 33, 192], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `MAPPER_ROLE` (0x568b80b5) function
        pub fn mapper_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([86, 139, 128, 181], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `MERKLE_UPDATER` (0x3e9e3e73) function
        pub fn merkle_updater(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([62, 158, 62, 115], ())
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `childToRootToken` (0xab14248e) function
        pub fn child_to_root_token(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([171, 20, 36, 142], p0)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `cleanMapToken` (0x52174ca9) function
        pub fn clean_map_token(
            &self,
            root_token: ethers::core::types::Address,
            child_token_index: u64,
            child_token_sub_index: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [82, 23, 76, 169],
                    (root_token, child_token_index, child_token_sub_index),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `depositEtherFor` (0xf57663d1) function
        pub fn deposit_ether_for(
            &self,
            user: ethers::core::types::Address,
            ccd_user: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 118, 99, 209], (user, ccd_user))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `depositFor` (0x594986a4) function
        pub fn deposit_for(
            &self,
            user: ethers::core::types::Address,
            ccd_user: [u8; 32],
            root_token: ethers::core::types::Address,
            deposit_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [89, 73, 134, 164],
                    (user, ccd_user, root_token, deposit_data),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `getMerkleRoot` (0x49590657) function
        pub fn get_merkle_root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([73, 89, 6, 87], ())
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

        /// Calls the contract's `hashChild` (0xff57e1d8) function
        pub fn hash_child(
            &self,
            child_token_index: u64,
            child_token_sub_index: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [255, 87, 225, 216],
                    (child_token_index, child_token_sub_index),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `mapToken` (0x6f3cde1f) function
        pub fn map_token(
            &self,
            root_token: ethers::core::types::Address,
            child_token_index: u64,
            child_token_sub_index: u64,
            token_type: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [111, 60, 222, 31],
                    (
                        root_token,
                        child_token_index,
                        child_token_sub_index,
                        token_type,
                    ),
                )
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `processedExits` (0x607f2d42) function
        pub fn processed_exits(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 127, 45, 66], p0)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `registerVault` (0x286f8c74) function
        pub fn register_vault(
            &self,
            token_type: [u8; 32],
            vault_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 111, 140, 116], (token_type, vault_address))
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `remapToken` (0xf0471dc0) function
        pub fn remap_token(
            &self,
            root_token: ethers::core::types::Address,
            child_token_index: u64,
            child_token_sub_index: u64,
            token_type: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [240, 71, 29, 192],
                    (
                        root_token,
                        child_token_index,
                        child_token_sub_index,
                        token_type,
                    ),
                )
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

        /// Calls the contract's `rootToChildToken` (0xea60c7c4) function
        pub fn root_to_child_token(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([234, 96, 199, 196], p0)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `setMerkleRoot` (0x7cb64759) function
        pub fn set_merkle_root(
            &self,
            merkle_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 182, 71, 89], merkle_root)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `setStateSender` (0x6cb136b0) function
        pub fn set_state_sender(
            &self,
            new_state_sender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 177, 54, 176], new_state_sender)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `stateSenderAddress` (0xe2c49de1) function
        pub fn state_sender_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([226, 196, 157, 225], ())
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

        /// Calls the contract's `tokenToType` (0xe43009a6) function
        pub fn token_to_type(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([228, 48, 9, 166], p0)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `typeToVault` (0xc85d2631) function
        pub fn type_to_vault(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([200, 93, 38, 49], p0)
                .expect("method not found (this should never happen)")
        }

        /// Calls the contract's `withdraw` (0xb6fbcdae) function
        pub fn withdraw(
            &self,
            withdraw: WithdrawParams,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 251, 205, 174], (withdraw, proof))
                .expect("method not found (this should never happen)")
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

        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for
        /// all events of this contract
        pub fn events(&self) -> ethers::contract::builders::Event<M, BridgeManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BridgeManager<M> {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BridgeManagerEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ethers::contract::EthLogDecode for BridgeManagerEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized, {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(BridgeManagerEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(BridgeManagerEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(BridgeManagerEvents::RoleRevokedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BridgeManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BridgeManagerEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                BridgeManagerEvents::RoleGrantedFilter(element) => element.fmt(f),
                BridgeManagerEvents::RoleRevokedFilter(element) => element.fmt(f),
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
    /// Container type for all input parameters for the `ETHER_ADDRESS` function
    /// with signature `ETHER_ADDRESS()` and selector `[207, 29, 33, 192]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ETHER_ADDRESS", abi = "ETHER_ADDRESS()")]
    pub struct EtherAddressCall;
    /// Container type for all input parameters for the `MAPPER_ROLE` function
    /// with signature `MAPPER_ROLE()` and selector `[86, 139, 128, 181]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAPPER_ROLE", abi = "MAPPER_ROLE()")]
    pub struct MapperRoleCall;
    /// Container type for all input parameters for the `MERKLE_UPDATER`
    /// function with signature `MERKLE_UPDATER()` and selector `[62, 158, 62,
    /// 115]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MERKLE_UPDATER", abi = "MERKLE_UPDATER()")]
    pub struct MerkleUpdaterCall;
    /// Container type for all input parameters for the `childToRootToken`
    /// function with signature `childToRootToken(bytes32)` and selector `[171,
    /// 20, 36, 142]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "childToRootToken", abi = "childToRootToken(bytes32)")]
    pub struct ChildToRootTokenCall(pub [u8; 32]);
    /// Container type for all input parameters for the `cleanMapToken` function
    /// with signature `cleanMapToken(address,uint64,uint64)` and selector `[82,
    /// 23, 76, 169]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cleanMapToken", abi = "cleanMapToken(address,uint64,uint64)")]
    pub struct CleanMapTokenCall {
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
    }
    /// Container type for all input parameters for the `depositEtherFor`
    /// function with signature `depositEtherFor(address,bytes32)` and selector
    /// `[245, 118, 99, 209]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "depositEtherFor", abi = "depositEtherFor(address,bytes32)")]
    pub struct DepositEtherForCall {
        pub user:     ethers::core::types::Address,
        pub ccd_user: [u8; 32],
    }
    /// Container type for all input parameters for the `depositFor` function
    /// with signature `depositFor(address,bytes32,address,bytes)` and selector
    /// `[89, 73, 134, 164]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "depositFor", abi = "depositFor(address,bytes32,address,bytes)")]
    pub struct DepositForCall {
        pub user:         ethers::core::types::Address,
        pub ccd_user:     [u8; 32],
        pub root_token:   ethers::core::types::Address,
        pub deposit_data: ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `getMerkleRoot` function
    /// with signature `getMerkleRoot()` and selector `[73, 89, 6, 87]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMerkleRoot", abi = "getMerkleRoot()")]
    pub struct GetMerkleRootCall;
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
    /// Container type for all input parameters for the `hashChild` function
    /// with signature `hashChild(uint64,uint64)` and selector `[255, 87, 225,
    /// 216]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hashChild", abi = "hashChild(uint64,uint64)")]
    pub struct HashChildCall {
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
    }
    /// Container type for all input parameters for the `mapToken` function with
    /// signature `mapToken(address,uint64,uint64,bytes32)` and selector `[111,
    /// 60, 222, 31]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mapToken", abi = "mapToken(address,uint64,uint64,bytes32)")]
    pub struct MapTokenCall {
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
        pub token_type:            [u8; 32],
    }
    /// Container type for all input parameters for the `processedExits`
    /// function with signature `processedExits(bytes32)` and selector `[96,
    /// 127, 45, 66]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "processedExits", abi = "processedExits(bytes32)")]
    pub struct ProcessedExitsCall(pub [u8; 32]);
    /// Container type for all input parameters for the `registerVault` function
    /// with signature `registerVault(bytes32,address)` and selector `[40, 111,
    /// 140, 116]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registerVault", abi = "registerVault(bytes32,address)")]
    pub struct RegisterVaultCall {
        pub token_type:    [u8; 32],
        pub vault_address: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `remapToken` function
    /// with signature `remapToken(address,uint64,uint64,bytes32)` and selector
    /// `[240, 71, 29, 192]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "remapToken", abi = "remapToken(address,uint64,uint64,bytes32)")]
    pub struct RemapTokenCall {
        pub root_token:            ethers::core::types::Address,
        pub child_token_index:     u64,
        pub child_token_sub_index: u64,
        pub token_type:            [u8; 32],
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
    /// Container type for all input parameters for the `rootToChildToken`
    /// function with signature `rootToChildToken(address)` and selector `[234,
    /// 96, 199, 196]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rootToChildToken", abi = "rootToChildToken(address)")]
    pub struct RootToChildTokenCall(pub ethers::core::types::Address);
    /// Container type for all input parameters for the `setMerkleRoot` function
    /// with signature `setMerkleRoot(bytes32)` and selector `[124, 182, 71,
    /// 89]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMerkleRoot", abi = "setMerkleRoot(bytes32)")]
    pub struct SetMerkleRootCall {
        pub merkle_root: [u8; 32],
    }
    /// Container type for all input parameters for the `setStateSender`
    /// function with signature `setStateSender(address)` and selector `[108,
    /// 177, 54, 176]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setStateSender", abi = "setStateSender(address)")]
    pub struct SetStateSenderCall {
        pub new_state_sender: ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `stateSenderAddress`
    /// function with signature `stateSenderAddress()` and selector `[226, 196,
    /// 157, 225]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stateSenderAddress", abi = "stateSenderAddress()")]
    pub struct StateSenderAddressCall;
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
    /// Container type for all input parameters for the `tokenToType` function
    /// with signature `tokenToType(address)` and selector `[228, 48, 9, 166]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenToType", abi = "tokenToType(address)")]
    pub struct TokenToTypeCall(pub ethers::core::types::Address);
    /// Container type for all input parameters for the `typeToVault` function
    /// with signature `typeToVault(bytes32)` and selector `[200, 93, 38, 49]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "typeToVault", abi = "typeToVault(bytes32)")]
    pub struct TypeToVaultCall(pub [u8; 32]);
    /// Container type for all input parameters for the `withdraw` function with
    /// signature `withdraw((uint64,uint64,uint64,address,string,uint64,uint64),
    /// bytes32[])` and selector `[182, 251, 205, 174]`
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
        name = "withdraw",
        abi = "withdraw((uint64,uint64,uint64,address,string,uint64,uint64),bytes32[])"
    )]
    pub struct WithdrawCall {
        pub withdraw: WithdrawParams,
        pub proof:    ::std::vec::Vec<[u8; 32]>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BridgeManagerCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        EtherAddress(EtherAddressCall),
        MapperRole(MapperRoleCall),
        MerkleUpdater(MerkleUpdaterCall),
        ChildToRootToken(ChildToRootTokenCall),
        CleanMapToken(CleanMapTokenCall),
        DepositEtherFor(DepositEtherForCall),
        DepositFor(DepositForCall),
        GetMerkleRoot(GetMerkleRootCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        HashChild(HashChildCall),
        MapToken(MapTokenCall),
        ProcessedExits(ProcessedExitsCall),
        RegisterVault(RegisterVaultCall),
        RemapToken(RemapTokenCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RootToChildToken(RootToChildTokenCall),
        SetMerkleRoot(SetMerkleRootCall),
        SetStateSender(SetStateSenderCall),
        StateSenderAddress(StateSenderAddressCall),
        SupportsInterface(SupportsInterfaceCall),
        TokenToType(TokenToTypeCall),
        TypeToVault(TypeToVaultCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for BridgeManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <EtherAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::EtherAddress(decoded));
            }
            if let Ok(decoded) =
                <MapperRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::MapperRole(decoded));
            }
            if let Ok(decoded) =
                <MerkleUpdaterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::MerkleUpdater(decoded));
            }
            if let Ok(decoded) =
                <ChildToRootTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::ChildToRootToken(decoded));
            }
            if let Ok(decoded) =
                <CleanMapTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::CleanMapToken(decoded));
            }
            if let Ok(decoded) =
                <DepositEtherForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::DepositEtherFor(decoded));
            }
            if let Ok(decoded) =
                <DepositForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::DepositFor(decoded));
            }
            if let Ok(decoded) =
                <GetMerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::GetMerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HashChildCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::HashChild(decoded));
            }
            if let Ok(decoded) =
                <MapTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::MapToken(decoded));
            }
            if let Ok(decoded) =
                <ProcessedExitsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::ProcessedExits(decoded));
            }
            if let Ok(decoded) =
                <RegisterVaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::RegisterVault(decoded));
            }
            if let Ok(decoded) =
                <RemapTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::RemapToken(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <RootToChildTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::RootToChildToken(decoded));
            }
            if let Ok(decoded) =
                <SetMerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::SetMerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <SetStateSenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::SetStateSender(decoded));
            }
            if let Ok(decoded) =
                <StateSenderAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::StateSenderAddress(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <TokenToTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::TokenToType(decoded));
            }
            if let Ok(decoded) =
                <TypeToVaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::TypeToVault(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeManagerCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BridgeManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BridgeManagerCalls::DefaultAdminRole(element) => element.encode(),
                BridgeManagerCalls::EtherAddress(element) => element.encode(),
                BridgeManagerCalls::MapperRole(element) => element.encode(),
                BridgeManagerCalls::MerkleUpdater(element) => element.encode(),
                BridgeManagerCalls::ChildToRootToken(element) => element.encode(),
                BridgeManagerCalls::CleanMapToken(element) => element.encode(),
                BridgeManagerCalls::DepositEtherFor(element) => element.encode(),
                BridgeManagerCalls::DepositFor(element) => element.encode(),
                BridgeManagerCalls::GetMerkleRoot(element) => element.encode(),
                BridgeManagerCalls::GetRoleAdmin(element) => element.encode(),
                BridgeManagerCalls::GrantRole(element) => element.encode(),
                BridgeManagerCalls::HasRole(element) => element.encode(),
                BridgeManagerCalls::HashChild(element) => element.encode(),
                BridgeManagerCalls::MapToken(element) => element.encode(),
                BridgeManagerCalls::ProcessedExits(element) => element.encode(),
                BridgeManagerCalls::RegisterVault(element) => element.encode(),
                BridgeManagerCalls::RemapToken(element) => element.encode(),
                BridgeManagerCalls::RenounceRole(element) => element.encode(),
                BridgeManagerCalls::RevokeRole(element) => element.encode(),
                BridgeManagerCalls::RootToChildToken(element) => element.encode(),
                BridgeManagerCalls::SetMerkleRoot(element) => element.encode(),
                BridgeManagerCalls::SetStateSender(element) => element.encode(),
                BridgeManagerCalls::StateSenderAddress(element) => element.encode(),
                BridgeManagerCalls::SupportsInterface(element) => element.encode(),
                BridgeManagerCalls::TokenToType(element) => element.encode(),
                BridgeManagerCalls::TypeToVault(element) => element.encode(),
                BridgeManagerCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BridgeManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BridgeManagerCalls::DefaultAdminRole(element) => element.fmt(f),
                BridgeManagerCalls::EtherAddress(element) => element.fmt(f),
                BridgeManagerCalls::MapperRole(element) => element.fmt(f),
                BridgeManagerCalls::MerkleUpdater(element) => element.fmt(f),
                BridgeManagerCalls::ChildToRootToken(element) => element.fmt(f),
                BridgeManagerCalls::CleanMapToken(element) => element.fmt(f),
                BridgeManagerCalls::DepositEtherFor(element) => element.fmt(f),
                BridgeManagerCalls::DepositFor(element) => element.fmt(f),
                BridgeManagerCalls::GetMerkleRoot(element) => element.fmt(f),
                BridgeManagerCalls::GetRoleAdmin(element) => element.fmt(f),
                BridgeManagerCalls::GrantRole(element) => element.fmt(f),
                BridgeManagerCalls::HasRole(element) => element.fmt(f),
                BridgeManagerCalls::HashChild(element) => element.fmt(f),
                BridgeManagerCalls::MapToken(element) => element.fmt(f),
                BridgeManagerCalls::ProcessedExits(element) => element.fmt(f),
                BridgeManagerCalls::RegisterVault(element) => element.fmt(f),
                BridgeManagerCalls::RemapToken(element) => element.fmt(f),
                BridgeManagerCalls::RenounceRole(element) => element.fmt(f),
                BridgeManagerCalls::RevokeRole(element) => element.fmt(f),
                BridgeManagerCalls::RootToChildToken(element) => element.fmt(f),
                BridgeManagerCalls::SetMerkleRoot(element) => element.fmt(f),
                BridgeManagerCalls::SetStateSender(element) => element.fmt(f),
                BridgeManagerCalls::StateSenderAddress(element) => element.fmt(f),
                BridgeManagerCalls::SupportsInterface(element) => element.fmt(f),
                BridgeManagerCalls::TokenToType(element) => element.fmt(f),
                BridgeManagerCalls::TypeToVault(element) => element.fmt(f),
                BridgeManagerCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for BridgeManagerCalls {
        fn from(var: DefaultAdminRoleCall) -> Self { BridgeManagerCalls::DefaultAdminRole(var) }
    }
    impl ::std::convert::From<EtherAddressCall> for BridgeManagerCalls {
        fn from(var: EtherAddressCall) -> Self { BridgeManagerCalls::EtherAddress(var) }
    }
    impl ::std::convert::From<MapperRoleCall> for BridgeManagerCalls {
        fn from(var: MapperRoleCall) -> Self { BridgeManagerCalls::MapperRole(var) }
    }
    impl ::std::convert::From<MerkleUpdaterCall> for BridgeManagerCalls {
        fn from(var: MerkleUpdaterCall) -> Self { BridgeManagerCalls::MerkleUpdater(var) }
    }
    impl ::std::convert::From<ChildToRootTokenCall> for BridgeManagerCalls {
        fn from(var: ChildToRootTokenCall) -> Self { BridgeManagerCalls::ChildToRootToken(var) }
    }
    impl ::std::convert::From<CleanMapTokenCall> for BridgeManagerCalls {
        fn from(var: CleanMapTokenCall) -> Self { BridgeManagerCalls::CleanMapToken(var) }
    }
    impl ::std::convert::From<DepositEtherForCall> for BridgeManagerCalls {
        fn from(var: DepositEtherForCall) -> Self { BridgeManagerCalls::DepositEtherFor(var) }
    }
    impl ::std::convert::From<DepositForCall> for BridgeManagerCalls {
        fn from(var: DepositForCall) -> Self { BridgeManagerCalls::DepositFor(var) }
    }
    impl ::std::convert::From<GetMerkleRootCall> for BridgeManagerCalls {
        fn from(var: GetMerkleRootCall) -> Self { BridgeManagerCalls::GetMerkleRoot(var) }
    }
    impl ::std::convert::From<GetRoleAdminCall> for BridgeManagerCalls {
        fn from(var: GetRoleAdminCall) -> Self { BridgeManagerCalls::GetRoleAdmin(var) }
    }
    impl ::std::convert::From<GrantRoleCall> for BridgeManagerCalls {
        fn from(var: GrantRoleCall) -> Self { BridgeManagerCalls::GrantRole(var) }
    }
    impl ::std::convert::From<HasRoleCall> for BridgeManagerCalls {
        fn from(var: HasRoleCall) -> Self { BridgeManagerCalls::HasRole(var) }
    }
    impl ::std::convert::From<HashChildCall> for BridgeManagerCalls {
        fn from(var: HashChildCall) -> Self { BridgeManagerCalls::HashChild(var) }
    }
    impl ::std::convert::From<MapTokenCall> for BridgeManagerCalls {
        fn from(var: MapTokenCall) -> Self { BridgeManagerCalls::MapToken(var) }
    }
    impl ::std::convert::From<ProcessedExitsCall> for BridgeManagerCalls {
        fn from(var: ProcessedExitsCall) -> Self { BridgeManagerCalls::ProcessedExits(var) }
    }
    impl ::std::convert::From<RegisterVaultCall> for BridgeManagerCalls {
        fn from(var: RegisterVaultCall) -> Self { BridgeManagerCalls::RegisterVault(var) }
    }
    impl ::std::convert::From<RemapTokenCall> for BridgeManagerCalls {
        fn from(var: RemapTokenCall) -> Self { BridgeManagerCalls::RemapToken(var) }
    }
    impl ::std::convert::From<RenounceRoleCall> for BridgeManagerCalls {
        fn from(var: RenounceRoleCall) -> Self { BridgeManagerCalls::RenounceRole(var) }
    }
    impl ::std::convert::From<RevokeRoleCall> for BridgeManagerCalls {
        fn from(var: RevokeRoleCall) -> Self { BridgeManagerCalls::RevokeRole(var) }
    }
    impl ::std::convert::From<RootToChildTokenCall> for BridgeManagerCalls {
        fn from(var: RootToChildTokenCall) -> Self { BridgeManagerCalls::RootToChildToken(var) }
    }
    impl ::std::convert::From<SetMerkleRootCall> for BridgeManagerCalls {
        fn from(var: SetMerkleRootCall) -> Self { BridgeManagerCalls::SetMerkleRoot(var) }
    }
    impl ::std::convert::From<SetStateSenderCall> for BridgeManagerCalls {
        fn from(var: SetStateSenderCall) -> Self { BridgeManagerCalls::SetStateSender(var) }
    }
    impl ::std::convert::From<StateSenderAddressCall> for BridgeManagerCalls {
        fn from(var: StateSenderAddressCall) -> Self { BridgeManagerCalls::StateSenderAddress(var) }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for BridgeManagerCalls {
        fn from(var: SupportsInterfaceCall) -> Self { BridgeManagerCalls::SupportsInterface(var) }
    }
    impl ::std::convert::From<TokenToTypeCall> for BridgeManagerCalls {
        fn from(var: TokenToTypeCall) -> Self { BridgeManagerCalls::TokenToType(var) }
    }
    impl ::std::convert::From<TypeToVaultCall> for BridgeManagerCalls {
        fn from(var: TypeToVaultCall) -> Self { BridgeManagerCalls::TypeToVault(var) }
    }
    impl ::std::convert::From<WithdrawCall> for BridgeManagerCalls {
        fn from(var: WithdrawCall) -> Self { BridgeManagerCalls::Withdraw(var) }
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
    /// Container type for all return fields from the `ETHER_ADDRESS` function
    /// with signature `ETHER_ADDRESS()` and selector `[207, 29, 33, 192]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct EtherAddressReturn(pub ethers::core::types::Address);
    /// Container type for all return fields from the `MAPPER_ROLE` function
    /// with signature `MAPPER_ROLE()` and selector `[86, 139, 128, 181]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MapperRoleReturn(pub [u8; 32]);
    /// Container type for all return fields from the `MERKLE_UPDATER` function
    /// with signature `MERKLE_UPDATER()` and selector `[62, 158, 62, 115]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MerkleUpdaterReturn(pub [u8; 32]);
    /// Container type for all return fields from the `childToRootToken`
    /// function with signature `childToRootToken(bytes32)` and selector `[171,
    /// 20, 36, 142]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ChildToRootTokenReturn(pub ethers::core::types::Address);
    /// Container type for all return fields from the `getMerkleRoot` function
    /// with signature `getMerkleRoot()` and selector `[73, 89, 6, 87]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMerkleRootReturn(pub [u8; 32]);
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
    /// Container type for all return fields from the `hashChild` function with
    /// signature `hashChild(uint64,uint64)` and selector `[255, 87, 225, 216]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HashChildReturn(pub [u8; 32]);
    /// Container type for all return fields from the `processedExits` function
    /// with signature `processedExits(bytes32)` and selector `[96, 127, 45,
    /// 66]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ProcessedExitsReturn(pub bool);
    /// Container type for all return fields from the `rootToChildToken`
    /// function with signature `rootToChildToken(address)` and selector `[234,
    /// 96, 199, 196]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RootToChildTokenReturn {
        pub index:    u64,
        pub subindex: u64,
    }
    /// Container type for all return fields from the `stateSenderAddress`
    /// function with signature `stateSenderAddress()` and selector `[226, 196,
    /// 157, 225]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct StateSenderAddressReturn(pub ethers::core::types::Address);
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
    /// Container type for all return fields from the `tokenToType` function
    /// with signature `tokenToType(address)` and selector `[228, 48, 9, 166]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenToTypeReturn(pub [u8; 32]);
    /// Container type for all return fields from the `typeToVault` function
    /// with signature `typeToVault(bytes32)` and selector `[200, 93, 38, 49]`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TypeToVaultReturn(pub ethers::core::types::Address);
    /// `WithdrawParams(uint64,uint64,uint64,address,string,uint64,uint64)`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WithdrawParams {
        pub ccd_index:       u64,
        pub ccd_sub_index:   u64,
        pub amount:          u64,
        pub user_wallet:     ethers::core::types::Address,
        pub ccd_tx_hash:     String,
        pub ccd_event_index: u64,
        pub token_id:        u64,
    }
}
