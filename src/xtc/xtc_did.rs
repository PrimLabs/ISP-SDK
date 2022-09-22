// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use ic_cdk::export::candid::{self, CandidType, Deserialize};
use ic_cdk::api::call::CallResult;

#[derive(CandidType, Deserialize)]
enum TxError {
    NotifyDfxFailed,
    InsufficientAllowance,
    UnexpectedCyclesResponse,
    InsufficientBalance,
    InsufficientXTCFee,
    ErrorOperationStyle,
    Unauthorized,
    LedgerTrap,
    ErrorTo,
    Other,
    FetchRateFailed,
    BlockUsed,
    AmountTooSmall,
}

#[derive(CandidType, Deserialize)]
enum TxReceipt { Ok(candid::Nat), Err(TxError) }

#[derive(CandidType, Deserialize)]
struct burn_arg0 { canister_id: candid::Principal, amount: u64 }

type TransactionId = u64;
#[derive(CandidType, Deserialize)]
enum BurnError {
    InsufficientBalance,
    InvalidTokenContract,
    NotSufficientLiquidity,
}

#[derive(CandidType, Deserialize)]
enum BurnResult { Ok(TransactionId), Err(BurnError) }

#[derive(CandidType, Deserialize)]
struct events_arg0 { offset: Option<u64>, limit: u16 }

#[derive(CandidType, Deserialize)]
enum TransactionStatus { FAILED, SUCCEEDED }

#[derive(CandidType, Deserialize)]
enum EventDetail {
    Approve{ to: candid::Principal, from: candid::Principal },
    Burn{ to: candid::Principal, from: candid::Principal },
    Mint{ to: candid::Principal },
    CanisterCreated{ from: candid::Principal, canister: candid::Principal },
    CanisterCalled{
        from: candid::Principal,
        method_name: String,
        canister: candid::Principal,
    },
    Transfer{ to: candid::Principal, from: candid::Principal },
    TransferFrom{
        to: candid::Principal,
        from: candid::Principal,
        caller: candid::Principal,
    },
}

#[derive(CandidType, Deserialize)]
struct Event {
    fee: u64,
    status: TransactionStatus,
    kind: EventDetail,
    cycles: u64,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
struct EventsConnection {
    data: Vec<Event>,
    next_offset: TransactionId,
    next_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
struct Metadata {
    fee: candid::Nat,
    decimals: u8,
    owner: candid::Principal,
    logo: String,
    name: String,
    totalSupply: candid::Nat,
    symbol: String,
}

#[derive(CandidType, Deserialize)]
enum Operation {
    transferFrom,
    burn,
    mint,
    approve,
    canisterCalled,
    transfer,
    canisterCreated,
}

type Time = candid::Int;
#[derive(CandidType, Deserialize)]
struct TxRecord {
    op: Operation,
    to: candid::Principal,
    fee: candid::Nat,
    status: TransactionStatus,
    from: candid::Principal,
    timestamp: Time,
    caller: Option<candid::Principal>,
    index: candid::Nat,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
enum MintError { NotSufficientLiquidity }

#[derive(CandidType, Deserialize)]
enum MintResult { Ok(TransactionId), Err(MintError) }

#[derive(CandidType, Deserialize)]
struct Stats {
    fee: candid::Nat,
    transfers_count: u64,
    balance: u64,
    mints_count: u64,
    transfers_from_count: u64,
    canisters_created_count: u64,
    supply: candid::Nat,
    burns_count: u64,
    approvals_count: u64,
    proxy_calls_count: u64,
    history_events: u64,
}

#[derive(CandidType, Deserialize)]
enum TxReceiptLegacy_Err { InsufficientAllowance, InsufficientBalance }

#[derive(CandidType, Deserialize)]
enum TxReceiptLegacy { Ok(candid::Nat), Err(TxReceiptLegacy_Err) }

#[derive(CandidType, Deserialize)]
struct wallet_balance_ret0 { amount: u64 }

#[derive(CandidType, Deserialize)]
struct wallet_call_arg0 {
    args: Vec<u8>,
    cycles: u64,
    method_name: String,
    canister: candid::Principal,
}

#[derive(CandidType, Deserialize)]
enum ResultCall { Ok{ r#return: Vec<u8> }, Err(String) }

#[derive(CandidType, Deserialize)]
struct wallet_create_canister_arg0 {
    controller: Option<candid::Principal>,
    cycles: u64,
}

#[derive(CandidType, Deserialize)]
enum CreateResult { Ok{ canister_id: candid::Principal }, Err(String) }

#[derive(CandidType, Deserialize)]
struct wallet_create_wallet_arg0 {
    controller: Option<candid::Principal>,
    cycles: u64,
}

#[derive(CandidType, Deserialize)]
struct wallet_send_arg0 { canister: candid::Principal, amount: u64 }

#[derive(CandidType, Deserialize)]
enum ResultSend { Ok, Err(String) }

struct SERVICE(candid::Principal);
impl SERVICE{
    pub async fn allowance(
        &self,
        arg0: candid::Principal,
        arg1: candid::Principal,
    ) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "allowance", (arg0,arg1,)).await
    }
    pub async fn approve(
        &self,
        arg0: candid::Principal,
        arg1: candid::Nat,
    ) -> CallResult<(TxReceipt,)> {
        ic_cdk::call(self.0, "approve", (arg0,arg1,)).await
    }
    pub async fn balance(&self, arg0: Option<candid::Principal>) -> CallResult<
        (u64,)
    > { ic_cdk::call(self.0, "balance", (arg0,)).await }
    pub async fn balanceOf(&self, arg0: candid::Principal) -> CallResult<
        (candid::Nat,)
    > { ic_cdk::call(self.0, "balanceOf", (arg0,)).await }
    pub async fn burn(&self, arg0: burn_arg0) -> CallResult<(BurnResult,)> {
        ic_cdk::call(self.0, "burn", (arg0,)).await
    }
    pub async fn decimals(&self) -> CallResult<(u8,)> {
        ic_cdk::call(self.0, "decimals", ()).await
    }
    pub async fn events(&self, arg0: events_arg0) -> CallResult<
        (EventsConnection,)
    > { ic_cdk::call(self.0, "events", (arg0,)).await }
    pub async fn getBlockUsed(&self) -> CallResult<(Vec<u64>,)> {
        ic_cdk::call(self.0, "getBlockUsed", ()).await
    }
    pub async fn getMetadata(&self) -> CallResult<(Metadata,)> {
        ic_cdk::call(self.0, "getMetadata", ()).await
    }
    pub async fn getTransaction(&self, arg0: candid::Nat) -> CallResult<
        (TxRecord,)
    > { ic_cdk::call(self.0, "getTransaction", (arg0,)).await }
    pub async fn getTransactions(
        &self,
        arg0: candid::Nat,
        arg1: candid::Nat,
    ) -> CallResult<(Vec<TxRecord>,)> {
        ic_cdk::call(self.0, "getTransactions", (arg0,arg1,)).await
    }
    pub async fn get_map_block_used(&self, arg0: u64) -> CallResult<
        (Option<u64>,)
    > { ic_cdk::call(self.0, "get_map_block_used", (arg0,)).await }
    pub async fn get_transaction(&self, arg0: TransactionId) -> CallResult<
        (Option<Event>,)
    > { ic_cdk::call(self.0, "get_transaction", (arg0,)).await }
    pub async fn halt(&self) -> CallResult<()> {
        ic_cdk::call(self.0, "halt", ()).await
    }
    pub async fn historySize(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "historySize", ()).await
    }
    pub async fn isBlockUsed(&self, arg0: u64) -> CallResult<(bool,)> {
        ic_cdk::call(self.0, "isBlockUsed", (arg0,)).await
    }
    pub async fn logo(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "logo", ()).await
    }
    pub async fn mint(
        &self,
        arg0: candid::Principal,
        arg1: candid::Nat,
    ) -> CallResult<(MintResult,)> {
        ic_cdk::call(self.0, "mint", (arg0,arg1,)).await
    }
    pub async fn mint_by_icp(
        &self,
        arg0: Option<Vec<u8>>,
        arg1: u64,
    ) -> CallResult<(TxReceipt,)> {
        ic_cdk::call(self.0, "mint_by_icp", (arg0,arg1,)).await
    }
    pub async fn name(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "name", ()).await
    }
    pub async fn nameErc20(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "nameErc20", ()).await
    }
    pub async fn stats(&self) -> CallResult<(Stats,)> {
        ic_cdk::call(self.0, "stats", ()).await
    }
    pub async fn symbol(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "symbol", ()).await
    }
    pub async fn totalSupply(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "totalSupply", ()).await
    }
    pub async fn transfer(
        &self,
        arg0: candid::Principal,
        arg1: candid::Nat,
    ) -> CallResult<(TxReceipt,)> {
        ic_cdk::call(self.0, "transfer", (arg0,arg1,)).await
    }
    pub async fn transferErc20(
        &self,
        arg0: candid::Principal,
        arg1: candid::Nat,
    ) -> CallResult<(TxReceiptLegacy,)> {
        ic_cdk::call(self.0, "transferErc20", (arg0,arg1,)).await
    }
    pub async fn transferFrom(
        &self,
        arg0: candid::Principal,
        arg1: candid::Principal,
        arg2: candid::Nat,
    ) -> CallResult<(TxReceipt,)> {
        ic_cdk::call(self.0, "transferFrom", (arg0,arg1,arg2,)).await
    }
    pub async fn wallet_balance(&self) -> CallResult<(wallet_balance_ret0,)> {
        ic_cdk::call(self.0, "wallet_balance", ()).await
    }
    pub async fn wallet_call(&self, arg0: wallet_call_arg0) -> CallResult<
        (ResultCall,)
    > { ic_cdk::call(self.0, "wallet_call", (arg0,)).await }
    pub async fn wallet_create_canister(
        &self,
        arg0: wallet_create_canister_arg0,
    ) -> CallResult<(CreateResult,)> {
        ic_cdk::call(self.0, "wallet_create_canister", (arg0,)).await
    }
    pub async fn wallet_create_wallet(
        &self,
        arg0: wallet_create_wallet_arg0,
    ) -> CallResult<(CreateResult,)> {
        ic_cdk::call(self.0, "wallet_create_wallet", (arg0,)).await
    }
    pub async fn wallet_send(&self, arg0: wallet_send_arg0) -> CallResult<
        (ResultSend,)
    > { ic_cdk::call(self.0, "wallet_send", (arg0,)).await }
}
