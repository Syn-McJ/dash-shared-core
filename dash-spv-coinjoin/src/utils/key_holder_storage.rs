use std::{cell::RefCell, rc::Rc};

use crate::{utils::key_holder::KeyHolder, wallet_ex::WalletEx};

#[derive(Debug)]
pub(crate) struct KeyHolderStorage {
    storage: Vec<KeyHolder>,
}

impl KeyHolderStorage {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn add_key(&mut self, wallet: Rc<RefCell<WalletEx>>) -> Option<Vec<u8>>  {
        let key_holder = KeyHolder::new(wallet);
        let script = key_holder.destination.clone();
        self.storage.push(key_holder);
        
        return script;
    }

    pub fn keep_all(&self) {
        // TODO
    }

    pub fn return_all(&self) {
        // TODO
    }
}