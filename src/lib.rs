use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
mod internal;

use crate::util::*;
mod util;

use crate::account::*;
mod account;

use crate::transfer_transaction::*;
mod transfer_transaction;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
    TransactionKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct HurdlePayment {
    pub owner_id: AccountId,
    pub accounts: LookupMap<AccountId, Account>,
    pub locking_transactions: Vector<TransferTransaction>,
}

#[near_bindgen]
impl HurdlePayment {
    #[init]
    pub fn new() -> Self {
        HurdlePayment {
            owner_id: env::current_account_id(),
            accounts: LookupMap::new(StorageKey::AccountKey),
            locking_transactions: Vector::new(StorageKey::TransactionKey),
        }
    }

    #[payable]
    pub fn register_new_account(&mut self, account_id: AccountId) {
        assert_at_least_one_yocto();
        let account_stake = self.accounts.get(&account_id);

        if account_stake.is_some() {
            // refund all deposited token
            refund_deposit(0, 0);
        } else {
            // Create new account
            let before_storage_usage = env::storage_usage();
            // Refund deposited token to user's account
            self.internal_register_account(account_id.clone());
            let after_storage_usage = env::storage_usage();
            refund_deposit(0, after_storage_usage - before_storage_usage);
        }
    }

    #[payable]
    pub fn send_to_receiver(&mut self, receiver_id: AccountId, y_amount: f64) {
        let attached_amount = env::attached_deposit();
        let amount = (y_amount * 1_000_000_000_000_000_000_000_000_f64) as u128;
        if attached_amount >= amount {
            Promise::new(receiver_id).transfer(amount);
            refund_deposit(amount, 0)
        } else {
            refund_deposit(0, 0)
        }
    }

    #[payable]
    pub fn send_to_contract(&mut self, receiver_id: AccountId, y_amount: f64) {
        let amount = (y_amount * 1_000_000_000_000_000_000_000_000_f64) as u128;
        let before_storage_usage = env::storage_usage();
        // Refund deposited token to user's account
        self.internal_create_transfer_transaction(receiver_id, amount, 14);
        let after_storage_usage = env::storage_usage();
        refund_deposit(amount, after_storage_usage - before_storage_usage);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
