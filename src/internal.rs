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
    account_id: AccountId,
    amount: Balance,
    unlock_epoch: EpochHeight,
  ) {
    let trans = TransferTransaction {
      account_id: account_id,
      locked_balance: amount,
      transaction_start_at: env::epoch_height(),
      transaction_unlock_at: env::epoch_height() + unlock_epoch,
    };
    self.locking_transactions.push(&trans);
  }
}
