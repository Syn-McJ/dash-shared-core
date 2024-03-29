use std::ffi::c_void;
use dash_spv_masternode_processor::ffi::ByteArray;
use dash_spv_masternode_processor::types::{self, MasternodeEntry};
use crate::ffi::selected_coins::SelectedCoins;
use crate::wallet_ex::WalletEx;

use super::coin_control::CoinControl;
use super::gathered_outputs::GatheredOutputs;
use super::input_value::InputValue;
use super::recepient::Recipient;

pub type GetInputValueByPrevoutHash = unsafe extern "C" fn(
    prevout_hash: *mut [u8; 32],
    index: u32,
    context: *const c_void,
) -> *mut InputValue;

pub type HasChainLock = unsafe extern "C" fn(
    block: *mut types::Block,
    context: *const c_void,
) -> bool;

pub type DestroyInputValue = unsafe extern "C" fn(
    input_value: *mut InputValue,
);

pub type GetWalletTransaction = unsafe extern "C" fn(
    hash: *mut [u8; 32],
    context: *const c_void,
) -> *mut types::Transaction;

pub type DestroyWalletTransaction = unsafe extern "C" fn(
    input_value: *mut types::Transaction,
);

pub type SignTransaction = unsafe extern "C" fn(
    transaction: *mut types::Transaction,
    context: *const c_void
) -> *mut types::Transaction;

pub type IsMineInput = unsafe extern "C" fn(
    prevout_hash: *mut [u8; 32],
    index: u32,
    context: *const c_void,
) -> bool;

pub type AvailableCoins = unsafe extern "C" fn(
    only_safe: bool,
    coin_control: CoinControl,
    wallet_ex: &mut WalletEx,
    context: *const c_void,
) -> *mut GatheredOutputs;

pub type DestroyGatheredOutputs = unsafe extern "C" fn(
    gathered_outputs: *mut GatheredOutputs,
);

pub type SelectCoinsGroupedByAddresses = unsafe extern "C" fn(
    skip_denominated: bool,
    anonymizable: bool,
    skip_unconfirmed: bool,
    max_oupoints_per_address: i32,
    wallet_ex: &mut WalletEx,
    context: *const c_void,
) -> *mut SelectedCoins;

pub type DestroySelectedCoins = unsafe extern "C" fn(
    selected_coins: *mut SelectedCoins,
);

pub type IsMineAddress = unsafe extern "C" fn(
    address: *mut [u8; 32],
    context: *const c_void,
) -> bool;

pub type InputsWithAmount = unsafe extern "C" fn(
    amount: u64,
    context: *const c_void,
) -> u32;

pub type FreshCoinJoinAddress = unsafe extern "C" fn(
    internal: bool,
    context: *const c_void,
) -> ByteArray;

pub type CommitTransaction = unsafe extern "C" fn(
    items: *mut *mut Recipient,
    item_count: usize,
    context: *const c_void
) -> bool;

pub type MasternodeByHash = unsafe extern "C" fn(
    address: *mut [u8; 32],
    context: *const c_void,
) -> *mut MasternodeEntry;

pub type DestroyMasternode = unsafe extern "C" fn(
    selected_coins: *mut MasternodeEntry,
);

pub type ValidMasternodeCount = unsafe extern "C" fn(
    context: *const c_void,
) -> u64;

pub type IsBlockchainSynced = unsafe extern "C" fn(
    context: *const c_void,
) -> bool;

pub type GetMasternodeList = unsafe extern "C" fn(
    context: *const c_void,
) -> *mut types::MasternodeList;

pub type DestroyMasternodeList = unsafe extern "C" fn(
    mn_list: *mut types::MasternodeList,
);