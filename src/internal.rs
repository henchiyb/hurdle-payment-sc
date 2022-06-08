use crate::*;

#[near_bindgen]
impl HurdlePayment {
  pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
    let account = Account {
      unlock_balance: 0,
      locked_balance: 0,
      total_balance: 0,
      available_epoch: 0,
    };
    self.accounts.insert(&account_id, &account);
  }

  pub(crate) fn internal_create_transfer_transaction(
    &mut self,
    receiver_id: AccountId,
    amount: Balance,
    unlock_epoch: EpochHeight,
  ) {
    let account = self.accounts.get(&receiver_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    let trans = TransferTransaction {
      sender_id: env::predecessor_account_id(),
      receiver_id: receiver_id.clone(),
      locked_balance: amount,
      transaction_start_at: env::epoch_height(),
      transaction_unlock_at: env::epoch_height() + unlock_epoch,
    };
    account.locked_balance += amount;
    account.total_balance += amount;
    self.accounts.insert(&receiver_id, &account);
    self.locking_transactions.push(&trans);
  }

  pub(crate) fn internal_withdraw_unlock_balance(
    &mut self,
    receiver_id: AccountId,
    amount: Balance,
  ) {
    let account = self.accounts.get(&receiver_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    assert!(
      amount <= account.unlock_balance,
      "Can't withdraw large than unlocked balance"
    );
    account.unlock_balance -= amount;
    account.total_balance -= amount;
    Promise::new(receiver_id.clone()).transfer(amount);
    self.accounts.insert(&receiver_id, &account);
  }
}
