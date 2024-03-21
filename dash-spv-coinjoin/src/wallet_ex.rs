use std::collections::{HashSet, HashMap};
use std::slice;
use byte::{BytesExt, LE};
use dash_spv_masternode_processor::consensus::Encodable;
use dash_spv_masternode_processor::crypto::UInt256;
use dash_spv_masternode_processor::crypto::byte_util::Reversable;
use dash_spv_masternode_processor::ffi::boxer::boxed_vec;
use dash_spv_masternode_processor::ffi::from::FromFFI;
use dash_spv_masternode_processor::ffi::to::ToFFI;
use dash_spv_masternode_processor::tx::{Transaction, TransactionInput};
use dash_spv_masternode_processor::util::address::address;
use ferment_interfaces::{boxed, unbox_any_vec, unbox_vec_ptr};

use crate::coin_selection::compact_tally_item::CompactTallyItem;
use crate::coin_selection::input_coin::InputCoin;
use crate::ffi::callbacks::{AvailableCoins, CommitTransaction, DestroyGatheredOutputs, DestroySelectedCoins, DestroyWalletTransaction, FreshCoinJoinAddress, GetWalletTransaction, InputsWithAmount, IsMineInput, SelectCoinsGroupedByAddresses, SignTransaction};
use crate::coinjoin::CoinJoin;
use crate::constants::MAX_COINJOIN_ROUNDS;
use crate::ffi::recepient::Recipient;
use crate::models::coin_control::{CoinControl, CoinType};
use crate::models::tx_destination::TxDestination;
use crate::models::tx_outpoint::TxOutPoint;
use crate::models::CoinJoinClientOptions;

#[derive(Debug)]
pub struct WalletEx {
    opaque_context: *const std::ffi::c_void,
    options: CoinJoinClientOptions,
    pub locked_coins_set: HashSet<TxOutPoint>,
    anonymizable_tally_cached_non_denom: bool,
    vec_anonymizable_tally_cached_non_denom: Vec<CompactTallyItem>, // TODO: is there a better way to cache?
    anonymizable_tally_cached: bool,
    vec_anonymizable_tally_cached: Vec<CompactTallyItem>,
    map_outpoint_rounds_cache: HashMap<TxOutPoint, i32>,
    unused_keys: HashMap<UInt256, Vec<u8>>,
    // TODO: we may not need keyUsage, it is used as a way to audit unusedKeys
    key_usage: HashMap<UInt256, bool>,
    coinjoin_salt: UInt256,
    get_wallet_transaction: GetWalletTransaction,
    sign_transaction: SignTransaction,
    destroy_transaction: DestroyWalletTransaction,
    is_mine_input: IsMineInput,
    available_coins: AvailableCoins,
    destroy_gathered_outputs: DestroyGatheredOutputs,
    select_coins: SelectCoinsGroupedByAddresses,
    destroy_selected_coins: DestroySelectedCoins,
    inputs_with_amount: InputsWithAmount,
    fresh_coinjoin_address: FreshCoinJoinAddress,
    commit_transaction: CommitTransaction
}

impl WalletEx {
    pub fn new(
        opaque_context: *const std::ffi::c_void,
        options: CoinJoinClientOptions,
        get_wallet_transaction: GetWalletTransaction,
        sign_transaction: SignTransaction,
        destroy_transaction: DestroyWalletTransaction,
        is_mine_input: IsMineInput,
        available_coins: AvailableCoins,
        destroy_gathered_outputs: DestroyGatheredOutputs,
        select_coins: SelectCoinsGroupedByAddresses,
        destroy_selected_coins: DestroySelectedCoins,
        inputs_with_amount: InputsWithAmount,
        fresh_coinjoin_address: FreshCoinJoinAddress,
        commit_transaction: CommitTransaction
    ) -> Self {
        WalletEx {
            opaque_context,
            options,
            locked_coins_set: HashSet::new(),
            anonymizable_tally_cached_non_denom: false,
            vec_anonymizable_tally_cached_non_denom: Vec::new(),
            anonymizable_tally_cached: false,
            vec_anonymizable_tally_cached: Vec::new(),
            map_outpoint_rounds_cache: HashMap::new(),
            coinjoin_salt: UInt256([0;32]), // TODO: InitCoinJoinSalt ?
            unused_keys: HashMap::with_capacity(1024),
            key_usage: HashMap::new(),
            get_wallet_transaction,
            sign_transaction,
            destroy_transaction,
            is_mine_input,
            available_coins,
            destroy_gathered_outputs,
            select_coins,
            destroy_selected_coins,
            inputs_with_amount,
            fresh_coinjoin_address,
            commit_transaction
        }
    }

    pub fn lock_coin(&mut self, outpoint: TxOutPoint) {
        self.locked_coins_set.insert(outpoint);
        self.clear_anonymizable_caches();
    }

    pub fn unlock_coin(&mut self, outpoint: &TxOutPoint) {
        self.locked_coins_set.remove(outpoint);
        self.clear_anonymizable_caches();
    }

    pub fn is_fully_mixed(&mut self, outpoint: TxOutPoint) -> bool {
        let rounds = self.get_real_outpoint_coinjoin_rounds(outpoint.clone(), 0);
        
        // Mix again if we don't have N rounds yet
        if rounds < self.options.coinjoin_rounds {
            return false;
        }

        // Try to mix a "random" number of rounds more than minimum.
        // If we have already mixed N + MaxOffset rounds, don't mix again.
        // Otherwise, we should mix again 50% of the time, this results in an exponential decay
        // N rounds 50% N+1 25% N+2 12.5%... until we reach N + GetRandomRounds() rounds where we stop.
        if rounds < self.options.coinjoin_rounds + self.options.coinjoin_random_rounds {
            let mut buffer = Vec::new();
            outpoint.consensus_encode(&mut buffer).unwrap();
            buffer.extend_from_slice(&self.coinjoin_salt.reversed().0);
            let hash = UInt256::sha256(&buffer);

            if &hash.0.read_with::<u32>(&mut 0, LE).unwrap() % 2 == 0 {
                return false;
            }
        }

        true
    }


    pub fn get_real_outpoint_coinjoin_rounds(&mut self, outpoint: TxOutPoint, rounds: i32) -> i32 {
        let rounds_max = MAX_COINJOIN_ROUNDS + self.options.coinjoin_random_rounds;

        if rounds >= rounds_max {
            // there can only be rounds_max rounds max
            return rounds_max - 1;
        }

        let mut rounds_ref = *self.map_outpoint_rounds_cache.entry(outpoint.clone()).or_insert(-10);

        if rounds_ref != -10 {
            return rounds_ref;
        }

        let wtx: Option<Transaction> = self.get_wallet_transaction(outpoint.hash);

        if wtx.is_none() {
            // no such tx in this wallet
            rounds_ref = -1;
            println!("FAILED    {:?} {} (no such tx)", outpoint, rounds_ref);
            self.map_outpoint_rounds_cache.insert(outpoint, rounds_ref);
            return rounds_ref;
        }

        let transaction = wtx.unwrap();
        // bounds check
        if outpoint.index >= transaction.outputs.len() as u32 {
            // should never actually hit this
            rounds_ref = -4;
            println!("FAILED    {:?} {} (bad index)", outpoint, rounds_ref);
            self.map_outpoint_rounds_cache.insert(outpoint, rounds_ref);
            return rounds_ref;
        }

        let tx_out = &transaction.outputs[outpoint.index as usize];

        if CoinJoin::is_collateral_amount(tx_out.amount) {
            rounds_ref = -3;
            println!("UPDATED    {:?} {} (collateral)", outpoint, rounds_ref);
            self.map_outpoint_rounds_cache.insert(outpoint, rounds_ref);
            return rounds_ref;
        }

        // make sure the final output is non-denominate
        if !CoinJoin::is_denominated_amount(tx_out.amount) {
            rounds_ref = -2;
            println!("UPDATED    {:?} {} (non-denominated)", outpoint, rounds_ref);
            self.map_outpoint_rounds_cache.insert(outpoint, rounds_ref);
            return rounds_ref;
        }

        for out in &transaction.outputs {
            if !CoinJoin::is_denominated_amount(out.amount) {
                // this one is denominated but there is another non-denominated output found in the same tx
                rounds_ref = 0;
                println!("UPDATED    {:?} {} (non-denominated)", outpoint, rounds_ref);
                self.map_outpoint_rounds_cache.insert(outpoint, rounds_ref);
                return rounds_ref;
            }
        }

        let mut n_shortest = -10; // an initial value, should be no way to get this by calculations
        let mut denom_found = false;

        // only denoms here so let's look up
        for txin_next in &transaction.inputs {
            if self.is_mine_input(&txin_next) {
                let outpoint = TxOutPoint::new(txin_next.input_hash, txin_next.index);
                let n = self.get_real_outpoint_coinjoin_rounds(outpoint, rounds + 1);

                // denom found, find the shortest chain or initially assign nShortest with the first found value
                if n >= 0 && (n < n_shortest || n_shortest == -10) {
                    n_shortest = n;
                    denom_found = true;
                }
            }
        }

        rounds_ref = if denom_found {
            if n_shortest >= rounds_max - 1 { rounds_max } else { n_shortest + 1 }
        } else {
            0
        };

        println!("UPDATED    {:?} {} (coinjoin)", outpoint, rounds_ref);
        self.map_outpoint_rounds_cache.insert(outpoint, rounds_ref);
        rounds_ref
    }

    pub fn has_collateral_inputs(&mut self, only_confirmed: bool) -> bool {
        let mut coin_control = CoinControl::new();
        coin_control.coin_type = CoinType::OnlyCoinJoinCollateral;
        let result = self.availalbe_coins(only_confirmed, coin_control);

        return !result.is_empty();
    }

    pub fn availalbe_coins(&mut self, only_safe: bool, coin_control: CoinControl) -> Vec<InputCoin> {
        let mut vec_gathered_outputs: Vec<InputCoin> = Vec::new();
        
        unsafe {
            let gathered_outputs = (self.available_coins)(only_safe, coin_control.encode(), self, self.opaque_context);
            (0..(*gathered_outputs).item_count)
                .into_iter()
                .map(|i| (**(*gathered_outputs).items.add(i)).decode())
                .for_each(
                    |item| vec_gathered_outputs.push(item)
                );

            (self.destroy_gathered_outputs)(gathered_outputs);
        }

        return vec_gathered_outputs;
    }

    pub fn select_coins_grouped_by_addresses(
        &mut self, 
        skip_denominated: bool, 
        anonymizable: bool, 
        skip_unconfirmed: bool, 
        max_outpoints_per_address: i32
    ) -> Vec<CompactTallyItem> {
        // Try using the cache for already confirmed mixable inputs.
        // This should only be used if maxOupointsPerAddress was NOT specified.
        if max_outpoints_per_address == -1 && anonymizable && skip_unconfirmed {
            if skip_denominated && self.anonymizable_tally_cached_non_denom {
                println!("[RUST] CoinJoin: SelectCoinsGroupedByAddresses - using cache for non-denom inputs {}", self.vec_anonymizable_tally_cached_non_denom.len());
                return self.vec_anonymizable_tally_cached_non_denom.clone();
            }

            if !skip_denominated && self.anonymizable_tally_cached {
                println!("[RUST] CoinJoin: SelectCoinsGroupedByAddresses - using cache for all inputs {}", self.vec_anonymizable_tally_cached.len());
                return self.vec_anonymizable_tally_cached.clone();
            }
        }
        
        let mut vec_tally_ret: Vec<CompactTallyItem> = Vec::new();

        unsafe {
            let selected_coins = (self.select_coins)(skip_denominated, anonymizable, skip_unconfirmed, max_outpoints_per_address, self, self.opaque_context);
            (0..(*selected_coins).item_count)
                .into_iter()
                .map(|i| (**(*selected_coins).items.add(i)).decode())
                .for_each(
                    |item| vec_tally_ret.push(item)
                );

            (self.destroy_selected_coins)(selected_coins);
        }

        // Cache already confirmed mixable entries for later use.
        // This should only be used if nMaxOupointsPerAddress was NOT specified.
        if max_outpoints_per_address == -1 && anonymizable && skip_unconfirmed {
            if skip_denominated {
                self.vec_anonymizable_tally_cached_non_denom = vec_tally_ret.clone();
                self.anonymizable_tally_cached_non_denom = true;
            } else {
                self.vec_anonymizable_tally_cached = vec_tally_ret.clone();
                self.anonymizable_tally_cached = true;
            }
        }
        
        println!("[RUST] CoinJoin, vec_tally_ret items: {:?}", vec_tally_ret);

        return vec_tally_ret;
    }

    pub fn get_anonymizable_balance(&mut self, skip_denominated: bool, skip_unconfirmed: bool) -> u64 {
        if !self.options.enable_coinjoin {
            return 0;
        }

        let tally_items = self.select_coins_grouped_by_addresses(skip_denominated, true, skip_unconfirmed, -1);
        
        if tally_items.is_empty() {
            return 0;
        }

        let mut total = 0;
        let smallest_denom = CoinJoin::get_smallest_denomination();
        let mixing_collateral = CoinJoin::get_collateral_amount();

        for item in tally_items {
            let is_denominated = CoinJoin::is_denominated_amount(item.amount);
            
            if skip_denominated && is_denominated {
                continue;
            }

            // assume that the fee to create denoms should be mixing collateral at max
            if item.amount >= smallest_denom + if is_denominated { 0 } else { mixing_collateral } {
                total = total + item.amount;
            }
        }

        return total;
    }

    pub fn get_wallet_transaction(&self, hash: UInt256) -> Option<Transaction> {
        unsafe {
            let wtx = (self.get_wallet_transaction)(boxed(hash.0), self.opaque_context);
            
            if wtx.is_null() {
                return None;
            }
            
            let transaction = (*wtx).decode();
            (self.destroy_transaction)(wtx);
            Some(transaction)
        }
    }

    /**
     * Count the number of unspent outputs that have a certain value
     */
    pub fn count_input_with_amount(&self, value: u64) -> u32 {
        return unsafe { (self.inputs_with_amount)(value, self.opaque_context) };
    }

    pub fn get_unused_key(&mut self, internal: bool) -> TxDestination {
        if self.unused_keys.is_empty() {
            println!("[RUST] WalletEx - obtaining fresh key");
            println!("[RUST] WalletEx - keyUsage map has unused keys: {}", !self.key_usage.is_empty() && self.key_usage.values().all(|used| !used));
            return Some(self.fresh_receive_key(internal));
        }

        let key: UInt256;
        let item: Vec<u8>;

        if let Some(pair) = self.unused_keys.iter().next() {
            key = *pair.0;
            item = pair.1.clone();

            println!("[RUST] WalletEx - reusing key: {:?}", address::with_script_sig(&item, &self.options.chain_type.script_map()));
            println!("[RUST] WalletEx - keyUsage map says this key is used: {}", self.key_usage.get(&key).unwrap());
        } else {
            return None;
        }

        // remove the key
        self.unused_keys.remove(&key);
        self.key_usage.insert(key, true);
        
        return Some(item);
    }

    pub fn add_unused_key(&mut self, destination: &TxDestination) {
        if let Some(key) = destination {
            let key_id = UInt256::sha256(key);
            self.unused_keys.insert(key_id, key.clone());
            self.key_usage.insert(key_id, false);
            println!("[RUST] WalletEx - add unused key: {:?}", address::with_script_sig(&key, &self.options.chain_type.script_map()));
        }
    }

    pub fn remove_unused_key(&mut self, destination: &TxDestination) {
        if let Some(key) = destination {
            let key_id = UInt256::sha256(key);
            self.unused_keys.remove(&key_id);
            self.key_usage.insert(key_id, true);
            println!("[RUST] WalletEx - remove unused key: {:?}", address::with_script_sig(&key, &self.options.chain_type.script_map()));
        }
    }

    pub fn commit_transaction(&self, vec_send: &Vec<Recipient>) -> bool {
        let mut result = false;

        unsafe {
            let boxed = boxed_vec(
                vec_send
                    .iter()
                    .map(|input| boxed((*input).clone()))
                    .collect()
            );
            result = (self.commit_transaction)(boxed, vec_send.len(), self.opaque_context);
            let vec = unbox_vec_ptr(boxed, vec_send.len());
            unbox_any_vec(vec);
        }

        return result;
    }

    pub fn sign_transaction(&self, tx: &Transaction) -> Option<Transaction> {
        unsafe {
            let raw_tx = (self.sign_transaction)(boxed(tx.encode()), self.opaque_context);

            if raw_tx.is_null() {
                return None;
            }

            let signed_tx =  (*raw_tx).decode();
            (self.destroy_transaction)(raw_tx);

            return Some(signed_tx);
        }
    }

    fn clear_anonymizable_caches(&mut self) {
        self.anonymizable_tally_cached_non_denom = false;
        self.anonymizable_tally_cached = false;
    }

    fn is_mine_input(&self, txin: &TransactionInput) -> bool {
        unsafe { (self.is_mine_input)(boxed(txin.input_hash.0), txin.index, self.opaque_context) }
    }

    fn fresh_receive_key(&mut self, internal: bool) -> Vec<u8> {
        let fresh_key = unsafe {
            let data = (self.fresh_coinjoin_address)(internal, self.opaque_context);
            let result = slice::from_raw_parts(data.ptr, data.len);
            unbox_vec_ptr(data.ptr as *mut u8, data.len);
            result
        };

        let result = fresh_key.to_vec();
        println!("[RUST] WalletEx - fresh key: {:?}", address::with_script_pub_key(&result, &self.options.chain_type.script_map()));
        self.key_usage.insert(UInt256::sha256(fresh_key), true);

        return result;
    }
}
