use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue,
};

use crate::internal::*;
mod internal;
use crate::util::*;
mod util;
use crate::account::*;
mod account;
use crate::transfer_transaction::*;
mod transfer_transaction;
use chrono::prelude::*;
use chrono::{Duration, NaiveDate, NaiveDateTime};

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
    DateTransactionKey,
    TransactionKey,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct HurdlePayment {
    pub owner_id: AccountId,
    pub accounts: LookupMap<AccountId, Account>,
}

#[near_bindgen]
impl HurdlePayment {
    #[init]
    pub fn new() -> Self {
        HurdlePayment {
            owner_id: env::current_account_id(),
            accounts: LookupMap::new(StorageKey::AccountKey),
        }
    }

    #[payable]
    pub fn register_new_account(&mut self, account_id: AccountId) {
        assert_at_least_one_yocto();
        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "Invalid account id"
        );
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
    pub fn send_to_contract(
        &mut self,
        receiver_id: AccountId,
        y_amount: f64,
        cash_hold_time: i64,
        campaign_id: String,
        transation_id: String,
    ) {
        let amount = (y_amount * 1_000_000_000_000_000_000_000_000_f64) as u128;
        let before_storage_usage = env::storage_usage();
        // Refund deposited token to user's account
        self.internal_create_transfer_transaction(
            receiver_id,
            amount,
            cash_hold_time,
            campaign_id,
            transation_id,
        );
        let after_storage_usage = env::storage_usage();
        refund_deposit(amount, after_storage_usage - before_storage_usage);
    }

    #[payable]
    pub fn withdraw_unlocked_balance(&mut self, receiver_id: AccountId, y_amount: f64) {
        let amount = (y_amount * 1_000_000_000_000_000_000_000_000_f64) as u128;
        self.internal_withdraw_unlocked_balance(receiver_id, amount);
    }

    pub fn claim_for_withdraw(&mut self, account_id: AccountId, end_date: String) {
        self.internal_unlock_locked_balance(account_id, end_date);
    }

    pub fn get_account_info(&self, account_id: AccountId) -> AccountJson {
        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "Invalid account id"
        );
        let account = self.accounts.get(&account_id).unwrap();
        AccountJson::from(account_id, account)
    }

    pub fn get_transactions_info(
        &self,
        account_id: AccountId,
        start_date: String,
        end_date: String,
    ) -> Vec<TransferTransactionJson> {
        let account = self.accounts.get(&account_id).unwrap();
        let date_transactions = account.transactions.get(&start_date).unwrap();
        let mut vec = Vec::new();
        for item in date_transactions.to_vec() {
            let (transaction_id, transaction) = item;
            vec.push(TransferTransactionJson::from(transaction_id, transaction))
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .attached_deposit(10 * 1_000_000_000_000_000_000_000_000)
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);
        builder
    }
    #[test]
    fn test_init_contract() {
        let context = get_context(false);

        testing_env!(context.build());

        let contract = HurdlePayment::new();
        assert_eq!(contract.owner_id, env::current_account_id());
    }

    #[test]
    fn test_register_new_account() {
        let context = get_context(false);

        testing_env!(context.build());

        let mut contract = HurdlePayment::new();
        contract.register_new_account(accounts(0).to_string());
        assert_eq!(
            contract
                .accounts
                .get(&accounts(0).to_string())
                .unwrap()
                .total_revenue,
            0
        );
    }

    #[test]
    fn test_send_to_contract() {
        let context = get_context(false);

        testing_env!(context.build());

        let mut contract = HurdlePayment::new();
        contract.register_new_account(accounts(1).to_string());
        contract.send_to_contract(
            accounts(1).to_string(),
            1.0,
            0,
            "1".to_string(),
            "1".to_string(),
        );
        let account_info = contract.get_account_info(accounts(1).to_string());
        assert_eq!(account_info.total_revenue, U128(999999999999999983222784));
        assert_eq!(account_info.locked_balance, U128(999999999999999983222784));
        let today = Utc::now().format("%Y-%m-%d").to_string();
        assert_eq!(account_info.last_unlock_at, today);
        contract.send_to_contract(
            accounts(1).to_string(),
            1.0,
            0,
            "1".to_string(),
            "2".to_string(),
        );
        let account_info = contract.get_account_info(accounts(1).to_string());
        assert_eq!(
            account_info.total_revenue,
            U128(999999999999999983222784 * 2)
        );

        assert_eq!(
            account_info.locked_balance,
            U128(999999999999999983222784 * 2)
        );
    }

    #[test]
    fn test_unlock_balance() {
        let context = get_context(false);

        testing_env!(context.build());

        let mut contract = HurdlePayment::new();
        contract.register_new_account(accounts(1).to_string());
        contract.send_to_contract(
            accounts(1).to_string(),
            1.0,
            0,
            "1".to_string(),
            "1".to_string(),
        );

        let locked_amount = contract
            .accounts
            .get(&accounts(1).to_string())
            .unwrap()
            .locked_balance;
        contract.claim_for_withdraw(
            accounts(1).to_string(),
            Utc::now().format("%Y-%m-%d").to_string(),
        );
        assert_eq!(
            contract
                .get_account_info(accounts(1).to_string())
                .unlocked_balance,
            U128(locked_amount)
        );

        assert_eq!(
            contract
                .get_account_info(accounts(1).to_string())
                .locked_balance,
            U128(0)
        );

        assert_eq!(
            contract
                .get_account_info(accounts(1).to_string())
                .total_revenue,
            U128(locked_amount)
        );

        contract.withdraw_unlocked_balance(accounts(1).to_string(), 1.0);
        assert_eq!(
            contract
                .get_account_info(accounts(1).to_string())
                .unlocked_balance,
            U128(0)
        );
        assert_eq!(
            contract
                .get_account_info(accounts(1).to_string())
                .total_revenue,
            U128(locked_amount)
        );
    }
}
