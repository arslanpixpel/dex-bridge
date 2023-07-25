import type {
  OpenAPIClient,
  Parameters,
  UnknownParamsObject,
  OperationResponse,
  AxiosRequestConfig,
} from 'openapi-client-axios'; 

export declare namespace Components {
    namespace Schemas {
        export interface EthMerkleProofResponse {
            params: WithdrawParams;
            proof: string;
        }
        export interface TokenMapItem {
            /**
             * Smart contract instance address.
             */
            ccd_contract?: {
                index?: number; // int64
                subindex?: number; // int64
            };
            ccd_name: string;
            decimals: number; // int32
            /**
             * Hex string
             */
            eth_address?: string;
            eth_name: string;
        }
        export type TransactionStatus = "pending" | "failed" | "processed" | "missing";
        export interface WalletDepositTx {
            amount: string;
            origin_event_index: number; // int64
            /**
             * Hex string
             */
            origin_tx_hash?: string;
            /**
             * Hex string
             */
            root_token?: string;
            status: TransactionStatus;
            timestamp: number; // int64
            /**
             * Optional transaction hash
             */
            tx_hash?: string | null;
        }
        export type WalletTx = {
            Withdraw: WalletWithdrawTx;
        } | {
            Deposit: WalletDepositTx;
        };
        export interface WalletWithdrawTx {
            amount: string;
            /**
             * Smart contract instance address.
             */
            child_token?: {
                index?: number; // int64
                subindex?: number; // int64
            };
            origin_event_index: number; // int64
            /**
             * Hex string
             */
            origin_tx_hash?: string;
            status: WithdrawalStatus;
            timestamp: number; // int64
            /**
             * Optional transaction hash
             */
            tx_hash?: string | null;
        }
        export interface WatchTxResponse {
            /**
             * Optional transaction hash
             */
            concordium_tx_hash?: string | null;
            status: TransactionStatus;
        }
        export interface WatchWithdrawalResponse {
            concordium_event_id?: number; // int64
            status: TransactionStatus;
        }
        export interface WithdrawParams {
            amount: string;
            ccd_event_index: number; // int64
            ccd_index: number; // int64
            ccd_sub_index: number; // int64
            /**
             * Hex string
             */
            ccd_tx_hash?: string;
            /**
             * Hex string
             */
            token_id?: string;
            /**
             * Hex string
             */
            user_wallet?: string;
        }
        export type WithdrawalStatus = "pending" | "processed";
    }
}
export declare namespace Paths {
    namespace EthMerkleProof {
        namespace Parameters {
            export type EventId = number; // int64
            export type TxHash = string;
        }
        export interface PathParameters {
            tx_hash: Parameters.TxHash;
            event_id: Parameters.EventId /* int64 */;
        }
        namespace Responses {
            export type $200 = Components.Schemas.EthMerkleProofResponse;
            export type $400 = string;
            export type $404 = string;
            export type $500 = string;
        }
    }
    namespace ExpectedMerkleRootUpdate {
        namespace Responses {
            export type $200 = number; // int64
            export type $500 = string;
        }
    }
    namespace ListTokens {
        namespace Responses {
            export type $200 = Components.Schemas.TokenMapItem[];
            export type $500 = string;
        }
    }
    namespace WalletTxs {
        namespace Parameters {
            export type Wallet = string;
        }
        export interface PathParameters {
            wallet: Parameters.Wallet;
        }
        namespace Responses {
            export type $200 = Components.Schemas.WalletTx[];
            export type $400 = string;
            export type $500 = string;
        }
    }
    namespace WatchDepositTx {
        namespace Parameters {
            export type TxHash = string;
        }
        export interface PathParameters {
            tx_hash: Parameters.TxHash;
        }
        namespace Responses {
            export type $200 = Components.Schemas.WatchTxResponse;
            export type $400 = string;
            export type $500 = string;
        }
    }
    namespace WatchWithdrawTx {
        namespace Parameters {
            export type TxHash = string;
        }
        export interface PathParameters {
            tx_hash: Parameters.TxHash;
        }
        namespace Responses {
            export type $200 = Components.Schemas.WatchWithdrawalResponse;
            export type $400 = string;
            export type $500 = string;
        }
    }
}

export interface OperationMethods {
  /**
   * eth_merkle_proof
   */
  'eth_merkle_proof'(
    parameters?: Parameters<Paths.EthMerkleProof.PathParameters> | null,
    data?: any,
    config?: AxiosRequestConfig  
  ): OperationResponse<Paths.EthMerkleProof.Responses.$200>
  /**
   * watch_deposit_tx - Queried by Ethereum transaction hash, respond with the status of the
   * 
   * Queried by Ethereum transaction hash, respond with the status of the
   * corresponding transaction on Concordium that handles the deposit.
   */
  'watch_deposit_tx'(
    parameters?: Parameters<Paths.WatchDepositTx.PathParameters> | null,
    data?: any,
    config?: AxiosRequestConfig  
  ): OperationResponse<Paths.WatchDepositTx.Responses.$200>
  /**
   * expected_merkle_root_update - Queried by Ethereum transaction hash, respond with the status of the
   * 
   * Queried by Ethereum transaction hash, respond with the status of the
   * corresponding transaction on Concordium that handles the deposit.
   */
  'expected_merkle_root_update'(
    parameters?: Parameters<UnknownParamsObject> | null,
    data?: any,
    config?: AxiosRequestConfig  
  ): OperationResponse<Paths.ExpectedMerkleRootUpdate.Responses.$200>
  /**
   * list_tokens - List all tokens that are mapped.
   * 
   * List all tokens that are mapped.
   */
  'list_tokens'(
    parameters?: Parameters<UnknownParamsObject> | null,
    data?: any,
    config?: AxiosRequestConfig  
  ): OperationResponse<Paths.ListTokens.Responses.$200>
  /**
   * wallet_txs
   */
  'wallet_txs'(
    parameters?: Parameters<Paths.WalletTxs.PathParameters> | null,
    data?: any,
    config?: AxiosRequestConfig  
  ): OperationResponse<Paths.WalletTxs.Responses.$200>
  /**
   * watch_withdraw_tx - Queried by Concordium transaction hash, respond with the status of
   * 
   * Queried by Concordium transaction hash, respond with the status of
   * withdrawal on Ethereum.
   */
  'watch_withdraw_tx'(
    parameters?: Parameters<Paths.WatchWithdrawTx.PathParameters> | null,
    data?: any,
    config?: AxiosRequestConfig  
  ): OperationResponse<Paths.WatchWithdrawTx.Responses.$200>
}

export interface PathsDictionary {
  ['/api/v1/ethereum/proof/{tx_hash}/{event_id}']: {
    /**
     * eth_merkle_proof
     */
    'get'(
      parameters?: Parameters<Paths.EthMerkleProof.PathParameters> | null,
      data?: any,
      config?: AxiosRequestConfig  
    ): OperationResponse<Paths.EthMerkleProof.Responses.$200>
  }
  ['api/v1/deposit/{tx_hash}']: {
    /**
     * watch_deposit_tx - Queried by Ethereum transaction hash, respond with the status of the
     * 
     * Queried by Ethereum transaction hash, respond with the status of the
     * corresponding transaction on Concordium that handles the deposit.
     */
    'get'(
      parameters?: Parameters<Paths.WatchDepositTx.PathParameters> | null,
      data?: any,
      config?: AxiosRequestConfig  
    ): OperationResponse<Paths.WatchDepositTx.Responses.$200>
  }
  ['api/v1/expectedMerkleRootUpdate']: {
    /**
     * expected_merkle_root_update - Queried by Ethereum transaction hash, respond with the status of the
     * 
     * Queried by Ethereum transaction hash, respond with the status of the
     * corresponding transaction on Concordium that handles the deposit.
     */
    'get'(
      parameters?: Parameters<UnknownParamsObject> | null,
      data?: any,
      config?: AxiosRequestConfig  
    ): OperationResponse<Paths.ExpectedMerkleRootUpdate.Responses.$200>
  }
  ['api/v1/tokens']: {
    /**
     * list_tokens - List all tokens that are mapped.
     * 
     * List all tokens that are mapped.
     */
    'get'(
      parameters?: Parameters<UnknownParamsObject> | null,
      data?: any,
      config?: AxiosRequestConfig  
    ): OperationResponse<Paths.ListTokens.Responses.$200>
  }
  ['api/v1/wallet/{wallet}']: {
    /**
     * wallet_txs
     */
    'get'(
      parameters?: Parameters<Paths.WalletTxs.PathParameters> | null,
      data?: any,
      config?: AxiosRequestConfig  
    ): OperationResponse<Paths.WalletTxs.Responses.$200>
  }
  ['api/v1/withdraw/{tx_hash}']: {
    /**
     * watch_withdraw_tx - Queried by Concordium transaction hash, respond with the status of
     * 
     * Queried by Concordium transaction hash, respond with the status of
     * withdrawal on Ethereum.
     */
    'get'(
      parameters?: Parameters<Paths.WatchWithdrawTx.PathParameters> | null,
      data?: any,
      config?: AxiosRequestConfig  
    ): OperationResponse<Paths.WatchWithdrawTx.Responses.$200>
  }
}

export type AxiosClient = OpenAPIClient<OperationMethods, PathsDictionary>
