use {
    crate::rpc_response::RpcSimulateTransactionResult,
    derivative::Derivative,
    serde_json::{json, Value},
    solana_sdk::{clock::Slot, pubkey::Pubkey},
    std::fmt,
    thiserror::Error,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RpcRequest {
    Custom {
        method: &'static str,
    },
    DeregisterNode,
    GetAccountInfo,
    GetBalance,
    GetBlock,
    GetBlockHeight,
    GetBlockProduction,
    GetBlocks,
    GetBlocksWithLimit,
    GetBlockTime,
    GetClusterNodes,
    #[deprecated(since = "1.7.0", note = "Please use RpcRequest::GetBlock instead")]
    GetConfirmedBlock,
    #[deprecated(since = "1.7.0", note = "Please use RpcRequest::GetBlocks instead")]
    GetConfirmedBlocks,
    #[deprecated(
        since = "1.7.0",
        note = "Please use RpcRequest::GetBlocksWithLimit instead"
    )]
    GetConfirmedBlocksWithLimit,

    #[deprecated(
        since = "1.7.0",
        note = "Please use RpcRequest::GetSignaturesForAddress instead"
    )]
    GetConfirmedSignaturesForAddress2,
    #[deprecated(
        since = "1.7.0",
        note = "Please use RpcRequest::GetTransaction instead"
    )]
    GetConfirmedTransaction,
    GetEpochInfo,
    GetEpochSchedule,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetFeeForMessage instead"
    )]
    GetFeeCalculatorForBlockhash,
    GetFeeForMessage,
    #[deprecated(
        since = "1.9.0",
        note = "Please do not use, will no longer be available in the future"
    )]
    GetFeeRateGovernor,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetFeeForMessage instead"
    )]
    GetFees,
    GetFirstAvailableBlock,
    GetGenesisHash,
    GetHealth,
    GetIdentity,
    GetInflationGovernor,
    GetInflationRate,
    GetInflationReward,
    GetLargestAccounts,
    GetLatestBlockhash,
    GetLeaderSchedule,
    GetMinimumBalanceForRentExemption,
    GetMultipleAccounts,
    GetProgramAccounts,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetLatestBlockhash instead"
    )]
    GetRecentBlockhash,
    GetRecentPerformanceSamples,
    GetHighestSnapshotSlot,
    #[deprecated(
        since = "1.9.0",
        note = "Please use RpcRequest::GetHighestSnapshotSlot instead"
    )]
    GetSnapshotSlot,
    GetSignaturesForAddress,
    GetSignatureStatuses,
    GetSlot,
    GetMaxRetransmitSlot,
    GetMaxShredInsertSlot,
    GetSlotLeader,
    GetSlotLeaders,
    GetStorageTurn,
    GetStorageTurnRate,
    GetSlotsPerSegment,
    GetStakeActivation,
    GetStoragePubkeysForSlot,
    GetSupply,
    GetTokenLargestAccounts,
    GetTokenAccountBalance,
    GetTokenAccountsByDelegate,
    GetTokenAccountsByOwner,
    GetTokenSupply,
    GetTransaction,
    GetTransactionCount,
    GetVersion,
    GetVoteAccounts,
    IsBlockhashValid,
    MinimumLedgerSlot,
    RegisterNode,
    RequestAirdrop,
    SendTransaction,
    SimulateTransaction,
    SignVote,

    /// EVM scope
    EthGetBlockTransactionCountByHash,
    EthGetBlockTransactionCountByNumber,
    EthGetTransactionByBlockHashAndIndex,
    EthGetTransactionByBlockNumberAndIndex,
    EthGetTransactionCount,
    EthGetBalance,
    EthGetBlockByNumber,
    EthGetBlockByHash,
    EthBlockNumber,
    EthGetStorageAt,
    EthGetCode,
    EthGetTransactionByHash,
    EthGetTransactionReceipt,
    EthCall,
    EthEstimateGas,
    EthGetLogs,
    EthSyncing,
    EthTraceCall,
    EthTraceCallMany,
    EthTraceReplayTransaction,
    EthTraceReplayBlock,
    // nzt methods
    NztCall,
    NztGetTransactionCount,
    NztGetTransactionReceipt,

    /// Nexis Account scope
    GetNexisAccountsByOperationalKey,
    GetNexisAccountsByOwnerKey,
    GetNexisRelyingPartiesByOwnerKey,

    /// Debug scope
    DebugRecoverBlockHeader,
}

#[allow(deprecated)]
impl fmt::Display for RpcRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method = match self {
            RpcRequest::Custom { method } => method,
            RpcRequest::DeregisterNode => "deregisterNode",
            RpcRequest::GetAccountInfo => "getAccountInfo",
            RpcRequest::GetBalance => "getBalance",
            RpcRequest::GetBlock => "getBlock",
            RpcRequest::GetBlockHeight => "getBlockHeight",
            RpcRequest::GetBlockProduction => "getBlockProduction",
            RpcRequest::GetBlocks => "getBlocks",
            RpcRequest::GetBlocksWithLimit => "getBlocksWithLimit",
            RpcRequest::GetBlockTime => "getBlockTime",
            RpcRequest::GetClusterNodes => "getClusterNodes",
            RpcRequest::GetConfirmedBlock => "getConfirmedBlock",
            RpcRequest::GetConfirmedBlocks => "getConfirmedBlocks",
            RpcRequest::GetConfirmedBlocksWithLimit => "getConfirmedBlocksWithLimit",
            RpcRequest::GetConfirmedSignaturesForAddress2 => "getConfirmedSignaturesForAddress2",
            RpcRequest::GetConfirmedTransaction => "getConfirmedTransaction",
            RpcRequest::GetEpochInfo => "getEpochInfo",
            RpcRequest::GetEpochSchedule => "getEpochSchedule",
            RpcRequest::GetFeeCalculatorForBlockhash => "getFeeCalculatorForBlockhash",
            RpcRequest::GetFeeForMessage => "getFeeForMessage",
            RpcRequest::GetFeeRateGovernor => "getFeeRateGovernor",
            RpcRequest::GetFees => "getFees",
            RpcRequest::GetFirstAvailableBlock => "getFirstAvailableBlock",
            RpcRequest::GetGenesisHash => "getGenesisHash",
            RpcRequest::GetHealth => "getHealth",
            RpcRequest::GetIdentity => "getIdentity",
            RpcRequest::GetInflationGovernor => "getInflationGovernor",
            RpcRequest::GetInflationRate => "getInflationRate",
            RpcRequest::GetInflationReward => "getInflationReward",
            RpcRequest::GetLargestAccounts => "getLargestAccounts",
            RpcRequest::GetLatestBlockhash => "getLatestBlockhash",
            RpcRequest::GetLeaderSchedule => "getLeaderSchedule",
            RpcRequest::GetMaxRetransmitSlot => "getMaxRetransmitSlot",
            RpcRequest::GetMaxShredInsertSlot => "getMaxShredInsertSlot",
            RpcRequest::GetMinimumBalanceForRentExemption => "getMinimumBalanceForRentExemption",
            RpcRequest::GetMultipleAccounts => "getMultipleAccounts",
            RpcRequest::GetProgramAccounts => "getProgramAccounts",
            RpcRequest::GetRecentBlockhash => "getRecentBlockhash",
            RpcRequest::GetRecentPerformanceSamples => "getRecentPerformanceSamples",
            RpcRequest::GetHighestSnapshotSlot => "getHighestSnapshotSlot",
            RpcRequest::GetSnapshotSlot => "getSnapshotSlot",
            RpcRequest::GetSignaturesForAddress => "getSignaturesForAddress",
            RpcRequest::GetSignatureStatuses => "getSignatureStatuses",
            RpcRequest::GetSlot => "getSlot",
            RpcRequest::GetSlotLeader => "getSlotLeader",
            RpcRequest::GetSlotLeaders => "getSlotLeaders",
            RpcRequest::GetStakeActivation => "getStakeActivation",
            RpcRequest::GetStorageTurn => "getStorageTurn",
            RpcRequest::GetStorageTurnRate => "getStorageTurnRate",
            RpcRequest::GetSlotsPerSegment => "getSlotsPerSegment",
            RpcRequest::GetStoragePubkeysForSlot => "getStoragePubkeysForSlot",
            RpcRequest::GetSupply => "getSupply",
            RpcRequest::GetTokenLargestAccounts => "getTokenLargestAccounts",
            RpcRequest::GetTokenAccountBalance => "getTokenAccountBalance",
            RpcRequest::GetTokenAccountsByDelegate => "getTokenAccountsByDelegate",
            RpcRequest::GetTokenAccountsByOwner => "getTokenAccountsByOwner",
            RpcRequest::GetTokenSupply => "getTokenSupply",
            RpcRequest::GetTransaction => "getTransaction",
            RpcRequest::GetTransactionCount => "getTransactionCount",
            RpcRequest::GetVersion => "getVersion",
            RpcRequest::GetVoteAccounts => "getVoteAccounts",
            RpcRequest::IsBlockhashValid => "isBlockhashValid",
            RpcRequest::MinimumLedgerSlot => "minimumLedgerSlot",
            RpcRequest::RegisterNode => "registerNode",
            RpcRequest::RequestAirdrop => "requestAirdrop",
            RpcRequest::SendTransaction => "sendTransaction",
            RpcRequest::SimulateTransaction => "simulateTransaction",
            RpcRequest::SignVote => "signVote",
            RpcRequest::EthGetTransactionCount => "eth_getTransactionCount",
            RpcRequest::EthGetBalance => "eth_getBalance",
            RpcRequest::EthGetBlockByNumber => "eth_getBlockByNumber",
            RpcRequest::EthGetBlockByHash => "eth_getBlockByHash",
            RpcRequest::EthBlockNumber => "eth_blockNumber",
            RpcRequest::EthGetStorageAt => "eth_getStorageAt",
            RpcRequest::EthGetCode => "eth_getCode",
            RpcRequest::EthGetTransactionByHash => "eth_getTransactionByHash",
            RpcRequest::EthGetTransactionReceipt => "eth_getTransactionReceipt",
            RpcRequest::EthCall => "eth_call",
            RpcRequest::EthTraceCall => "trace_call",
            RpcRequest::EthTraceCallMany => "trace_callMany",
            RpcRequest::EthGetBlockTransactionCountByHash => "eth_getBlockTransactionCountByHash",
            RpcRequest::EthGetBlockTransactionCountByNumber => {
                "eth_getBlockTransactionCountByNumber"
            }
            RpcRequest::EthGetTransactionByBlockHashAndIndex => {
                "eth_getTransactionByBlockHashAndIndex"
            }
            RpcRequest::EthGetTransactionByBlockNumberAndIndex => {
                "eth_getTransactionByBlockNumberAndIndex"
            }
            RpcRequest::EthTraceReplayTransaction => "trace_replayTransaction",
            RpcRequest::EthTraceReplayBlock => "trace_replayBlockTransactions",
            RpcRequest::EthEstimateGas => "eth_estimateGas",
            RpcRequest::EthGetLogs => "eth_getLogs",
            RpcRequest::EthSyncing => "eth_syncing",
            RpcRequest::NztCall => "nzt_call",
            RpcRequest::NztGetTransactionReceipt => "nzt_getTransactionReceipt",

            RpcRequest::NztGetTransactionCount => "nzt_getTransactionCount",
            RpcRequest::GetNexisAccountsByOperationalKey => "getNexisAccountsByOperationalKey",
            RpcRequest::GetNexisAccountsByOwnerKey => "getNexisAccountsByOwnerKey",
            RpcRequest::GetNexisRelyingPartiesByOwnerKey => "getNexisRelyingPartiesByOwnerKey",
            RpcRequest::DebugRecoverBlockHeader => "debug_recoverBlockHeader",
        };

        write!(f, "{}", method)
    }
}

pub const MAX_GET_SIGNATURE_STATUSES_QUERY_ITEMS: usize = 256;
pub const MAX_GET_CONFIRMED_SIGNATURES_FOR_ADDRESS_SLOT_RANGE: u64 = 10_000;
pub const MAX_GET_CONFIRMED_BLOCKS_RANGE: u64 = 500_000;
pub const MAX_GET_CONFIRMED_SIGNATURES_FOR_ADDRESS2_LIMIT: usize = 1_000;
pub const MAX_MULTIPLE_ACCOUNTS: usize = 100;
pub const NUM_LARGEST_ACCOUNTS: usize = 20;
pub const MAX_GET_PROGRAM_ACCOUNT_FILTERS: usize = 4;
pub const MAX_GET_SLOT_LEADERS: usize = 5000;

// Validators that are this number of slots behind are considered delinquent
pub const DELINQUENT_VALIDATOR_SLOT_DISTANCE: u64 = 128;

impl RpcRequest {
    pub fn build_request_json(self, id: u64, params: Value) -> Value {
        let jsonrpc = "2.0";
        json!({
           "jsonrpc": jsonrpc,
           "id": id,
           "method": format!("{}", self),
           "params": params,
        })
    }
}

#[derive(Debug)]
pub enum RpcResponseErrorData {
    Empty,
    SendTransactionPreflightFailure(RpcSimulateTransactionResult),
    NodeUnhealthy { num_slots_behind: Option<Slot> },
}

impl fmt::Display for RpcResponseErrorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RpcResponseErrorData::SendTransactionPreflightFailure(
                RpcSimulateTransactionResult {
                    logs: Some(logs), ..
                },
            ) => {
                if logs.is_empty() {
                    Ok(())
                } else {
                    // Give the user a hint that there is more useful logging information available...
                    write!(f, "[{} log messages]", logs.len())
                }
            }
            _ => Ok(()),
        }
    }
}

#[derive(Derivative, Error)]
#[derivative(Debug)]
pub enum RpcError {
    #[error("RPC request error: {0}")]
    RpcRequestError(String),
    #[error("RPC response error {code}: {message} {data}")]
    RpcResponseError {
        code: i64,
        message: String,
        data: RpcResponseErrorData,
        #[derivative(Debug = "ignore")]
        original_err: serde_json::Value,
    },
    #[error("parse error: expected {0}")]
    ParseError(String), /* "expected" */
    // Anything in a `ForUser` needs to die.  The caller should be
    // deciding what to tell their user
    #[error("{0}")]
    ForUser(String), /* "direct-to-user message" */
}

#[derive(Serialize, Deserialize)]
pub enum TokenAccountsFilter {
    Mint(Pubkey),
    ProgramId(Pubkey),
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::rpc_config::RpcTokenAccountsFilter,
        solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel},
    };

    #[test]
    fn test_build_request_json() {
        let test_request = RpcRequest::GetAccountInfo;
        let addr = json!("deadbeefXjn8o3yroDHxUtKsZZgoy4GPkPPXfouKNHhx");
        let request = test_request.build_request_json(1, json!([addr]));
        assert_eq!(request["method"], "getAccountInfo");
        assert_eq!(request["params"], json!([addr]));

        let test_request = RpcRequest::GetBalance;
        let request = test_request.build_request_json(1, json!([addr]));
        assert_eq!(request["method"], "getBalance");

        let test_request = RpcRequest::GetEpochInfo;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "getEpochInfo");

        #[allow(deprecated)]
        let test_request = RpcRequest::GetRecentBlockhash;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "getRecentBlockhash");

        #[allow(deprecated)]
        let test_request = RpcRequest::GetFeeCalculatorForBlockhash;
        let request = test_request.build_request_json(1, json!([addr]));
        assert_eq!(request["method"], "getFeeCalculatorForBlockhash");

        #[allow(deprecated)]
        let test_request = RpcRequest::GetFeeRateGovernor;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "getFeeRateGovernor");

        let test_request = RpcRequest::GetSlot;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "getSlot");

        let test_request = RpcRequest::GetTransactionCount;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "getTransactionCount");

        let test_request = RpcRequest::RequestAirdrop;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "requestAirdrop");

        let test_request = RpcRequest::SendTransaction;
        let request = test_request.build_request_json(1, Value::Null);
        assert_eq!(request["method"], "sendTransaction");
    }

    #[test]
    fn test_build_request_json_config_options() {
        let commitment_config = CommitmentConfig {
            commitment: CommitmentLevel::Finalized,
        };
        let addr = json!("deadbeefXjn8o3yroDHxUtKsZZgoy4GPkPPXfouKNHhx");

        // Test request with CommitmentConfig and no params
        #[allow(deprecated)]
        let test_request = RpcRequest::GetRecentBlockhash;
        let request = test_request.build_request_json(1, json!([commitment_config]));
        assert_eq!(request["params"], json!([commitment_config.clone()]));

        // Test request with CommitmentConfig and params
        let test_request = RpcRequest::GetBalance;
        let request = test_request.build_request_json(1, json!([addr, commitment_config]));
        assert_eq!(request["params"], json!([addr, commitment_config]));

        // Test request with CommitmentConfig and params
        let test_request = RpcRequest::GetTokenAccountsByOwner;
        let mint = solana_sdk::pubkey::new_rand();
        let token_account_filter = RpcTokenAccountsFilter::Mint(mint.to_string());
        let request = test_request
            .build_request_json(1, json!([addr, token_account_filter, commitment_config]));
        assert_eq!(
            request["params"],
            json!([addr, token_account_filter, commitment_config])
        );
    }
}
