use core::slice;
use std::io::Cursor;
use std::ffi::c_void;

use dash_spv_coinjoin::coinjoin_client_session::CoinJoinClientSession;
use dash_spv_coinjoin::messages;
use dash_spv_coinjoin::messages::coinjoin_broadcast_tx::CoinJoinBroadcastTx;
use dash_spv_coinjoin::coinjoin::CoinJoin;
use dash_spv_coinjoin::ffi::callbacks::{DestroyInputValue, DestroySelectedCoins, DestroyWalletTransaction, GetInputValueByPrevoutHash, GetWalletTransaction, HasChainLock, HasCollateralInputs, InputsWithAmount, IsMineInput, SelectCoinsGroupedByAddresses, SignTransaction};
use dash_spv_coinjoin::models::tx_outpoint::TxOutPoint;
use dash_spv_coinjoin::models::CoinJoinClientOptions;
use dash_spv_coinjoin::wallet_ex::WalletEx;
use dash_spv_masternode_processor::consensus::Decodable;
use ferment_interfaces::{boxed, unbox_any};
use dash_spv_masternode_processor::crypto::UInt256;
use dash_spv_masternode_processor::ffi::from::FromFFI;
use dash_spv_masternode_processor::types;

#[no_mangle]
pub unsafe extern "C" fn register_coinjoin(
    get_input_value_by_prevout_hash: GetInputValueByPrevoutHash,
    has_chain_lock: HasChainLock,
    destroy_input_value: DestroyInputValue,
    context: *const c_void
) -> *mut CoinJoin {
    let coinjoin = CoinJoin::new(
        get_input_value_by_prevout_hash,
        has_chain_lock,
        destroy_input_value,
        context
    );
    println!("[RUST] register_coinjoin: {:?}", coinjoin);
    boxed(coinjoin)
}

#[no_mangle]
pub unsafe extern "C" fn register_client_session(
    coinjoin: *mut CoinJoin,
    options_ptr: *mut CoinJoinClientOptions,
    get_wallet_transaction: GetWalletTransaction,
    destroy_wallet_transaction: DestroyWalletTransaction,
    is_mine: IsMineInput,
    has_collateral_inputs: HasCollateralInputs,
    selected_coins: SelectCoinsGroupedByAddresses,
    destroy_selected_coins: DestroySelectedCoins,
    sign_transaction: SignTransaction,
    count_inputs_with_amount: InputsWithAmount,
    context: *const c_void
) -> *mut CoinJoinClientSession {
    let session = CoinJoinClientSession::new(
        std::ptr::read(coinjoin),
        std::ptr::read(options_ptr), 
        sign_transaction,
        get_wallet_transaction, 
        destroy_wallet_transaction, 
        is_mine, 
        has_collateral_inputs, 
        selected_coins, 
        destroy_selected_coins,
        count_inputs_with_amount,
        context
    );
    println!("[RUST] register_session");
    boxed(session)
}

#[no_mangle]
pub unsafe extern "C" fn call_session(
    session: *mut CoinJoinClientSession,
    balance_denominate: u64
) -> bool {
    println!("[RUST] call session");
    (*session).create_denominated(balance_denominate, false);
    return true;
}

#[no_mangle]
pub unsafe extern "C" fn on_transaction_signed_for_session(
    tx: *mut types::Transaction, 
    session: *mut CoinJoinClientSession,
    destroy_wallet_transaction: DestroyWalletTransaction
) {
    (*session).on_transaction_signed((*tx).decode());
    (destroy_wallet_transaction)(tx);
}

#[no_mangle]
pub unsafe extern "C" fn destroy_transaction(
    tx: *mut types::Transaction
) {
    println!("[RUST] 💀 destroy_transaction");
    let unboxed = unbox_any(tx);
}

#[no_mangle]
pub unsafe extern "C" fn unregister_coinjoin(coinjoin: *mut CoinJoin) {
    println!("[RUST] 💀 unregister_coinjoin: {:?}", coinjoin);
    let unboxed = unbox_any(coinjoin);
}

#[no_mangle]
pub unsafe extern "C" fn is_denominated_amount(
    amount: u64,
) -> bool {
    println!("[RUST] call is_denominated_amount with amount {}", amount);
    return CoinJoin::is_denominated_amount(amount);
}

#[no_mangle]
pub unsafe extern "C" fn is_collateral_amount(
    amount: u64,
) -> bool {
    println!("[RUST] call is_collateral_amount with amount {}", amount);
    return CoinJoin::is_collateral_amount(amount);
}

#[no_mangle]
pub unsafe extern "C" fn is_fully_mixed(
    wallet_ex: *mut WalletEx,
    prevout_hash: *mut [u8; 32],
    index: u32,
) -> bool {
    println!("[RUST] call wallet_ex.is_fully_mixed");
    return (*wallet_ex).is_fully_mixed(TxOutPoint::new(UInt256(*(prevout_hash)), index));
}

#[no_mangle]
pub unsafe extern "C" fn is_locked_coin(
    wallet_ex: *mut WalletEx,
    prevout_hash: *mut [u8; 32],
    index: u32,
) -> bool {
    println!("[RUST] call wallet_ex.is_fully_mixed");
    return (*wallet_ex).locked_coins_set.contains(&TxOutPoint::new(UInt256(*(prevout_hash)), index));
}

#[no_mangle]
pub unsafe extern "C" fn coinjoin_get_smallest_denomination() -> u64 {
    println!("[RUST] call coinjoin_get_smallest_denomination");
    return CoinJoin::get_smallest_denomination();
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_accept_message(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinAcceptMessage {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinAcceptMessage::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_broadcast_tx(
    message: *const u8,
    message_length: usize
) -> *mut CoinJoinBroadcastTx {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = CoinJoinBroadcastTx::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_complete_message(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinCompleteMessage {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinCompleteMessage::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_entry(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinEntry {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinEntry::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_final_transaction(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinFinalTransaction {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinFinalTransaction::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_queue_message(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinQueueMessage {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinQueueMessage::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_signed_inputs(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinSignedInputs {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinSignedInputs::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_coinjoin_status_update(
    message: *const u8,
    message_length: usize
) -> *mut messages::CoinJoinStatusUpdate {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::CoinJoinStatusUpdate::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}

#[no_mangle]
pub unsafe extern "C" fn process_send_coinjoin_queue(
    message: *const u8,
    message_length: usize
) -> *mut messages::SendCoinJoinQueue {
    let message: &[u8] = slice::from_raw_parts(message, message_length);
    let mut cursor = Cursor::new(message);
    let result = messages::SendCoinJoinQueue::consensus_decode(&mut cursor).unwrap();

    boxed(result)
}