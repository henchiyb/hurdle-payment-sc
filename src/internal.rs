use crate::*;

#[near_bindgen]
impl HurdlePayment {
  pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
    let account = Account {
      locked_balance: 0,
      total_revenue: 0,
      last_unlock_at: env::epoch_height(),
      transactions: UnorderedMap::new(StorageKey::AccountTransactionByDate {
        account_hash: env::sha256(account_id.as_bytes()),
      }),
    };
    self.accounts.insert(&account_id, &account);
  }

  pub(crate) fn internal_create_transfer_transaction(
    &mut self,
    receiver_id: AccountId,
    amount: Balance,
    cash_hold_time: i64,
    campaign_id: String,
    transaction_id: String,
  ) {
    let account = self.accounts.get(&receiver_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    let trans = TransferTransaction {
      sender_id: env::predecessor_account_id(),
      receiver_id: receiver_id.clone(),
      campaign_id: campaign_id,
      locked_balance: amount,
      created_at: env::epoch_height(),
      claimable_at: env::epoch_height() + cash_hold_time as u64,
      status: "LOCK".to_string(),
    };
    let today_epoch = env::epoch_height();
    let transactions = account.transactions.get(&today_epoch);
    if transactions.is_none() {
      let mut map = UnorderedMap::new(StorageKey::AccountTransaction {
        account_hash: env::sha256(env::epoch_height().to_string().as_bytes()),
      });
      map.insert(&transaction_id, &trans);
      account.transactions.insert(&today_epoch, &map);
    } else {
      let mut transactions = transactions.unwrap();
      let check_transaction = transactions.get(&transaction_id);
      assert_eq!(
        check_transaction.is_none(),
        true,
        "Transaction ID Duplicated"
      );
      transactions.insert(&transaction_id, &trans);
      account.transactions.insert(&today_epoch, &transactions);
    }

    account.locked_balance += amount;
    account.total_revenue += amount;
    self.accounts.insert(&receiver_id, &account);
  }

  pub(crate) fn internal_unlock_locked_balance(&mut self, account_id: AccountId) {
    let account = self.accounts.get(&account_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    let mut last_unlock_at = account.last_unlock_at;
    let mut transer_amount = 0;
    while last_unlock_at <= env::epoch_height() {
      let transactions = account.transactions.get(&last_unlock_at);
      if transactions.is_some() {
        let mut transactions = transactions.unwrap();
        for transaction in transactions.to_vec() {
          let (transaction_id, mut transaction) = transaction;
          if env::epoch_height() >= transaction.claimable_at
            && transaction.status == "LOCK".to_string()
          {
            transaction.status = "CLAIM".to_string();
            transactions.insert(&transaction_id, &transaction);
            account.locked_balance = account
              .locked_balance
              .checked_sub(transaction.locked_balance)
              .unwrap();
            transer_amount += transaction.locked_balance;
          }
        }
      }
      last_unlock_at += 1;
    }
    Promise::new(account_id.clone()).transfer(transer_amount);
    account.last_unlock_at = env::epoch_height();
    self.accounts.insert(&account_id, &account);
  }

  pub(crate) fn internal_refund_by_transaction_id(
    &mut self,
    sender_id: AccountId,
    receiver_id: AccountId,
    transaction_id: String,
    create_epoch: u64,
  ) {
    let account = self.accounts.get(&receiver_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    let transactions = account.transactions.get(&create_epoch);
    if transactions.is_some() {
      let mut transactions = transactions.unwrap();
      let transaction = transactions.get(&transaction_id);
      if transaction.is_some() {
        let mut transaction = transaction.unwrap();
        if env::epoch_height() < transaction.claimable_at
          && transaction.status == "LOCK".to_string()
        {
          transaction.status = "REFUND".to_string();
          account.locked_balance = account
            .locked_balance
            .checked_sub(transaction.locked_balance)
            .unwrap();
          account.total_revenue = account
            .total_revenue
            .checked_sub(transaction.locked_balance)
            .unwrap();
          transactions.insert(&transaction_id, &transaction);
          Promise::new(sender_id.clone()).transfer(transaction.locked_balance);
          self.accounts.insert(&receiver_id, &account);
        }
      }
    }
  }

  pub(crate) fn internal_refund_by_epoch(
    &mut self,
    sender_id: AccountId,
    receiver_id: AccountId,
    cash_hold_time: u64,
  ) {
    let account = self.accounts.get(&receiver_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    let mut start_epoch = env::epoch_height() - cash_hold_time * 2;
    let mut transer_amount = 0;
    while start_epoch <= env::epoch_height() {
      let transactions = account.transactions.get(&start_epoch);
      if transactions.is_some() {
        let mut transactions = transactions.unwrap();
        for transaction in transactions.to_vec() {
          let (transaction_id, mut transaction) = transaction;
          if env::epoch_height() < transaction.claimable_at
            && sender_id == transaction.sender_id
            && receiver_id == transaction.receiver_id
          {
            transaction.status = "REFUND".to_string();
            transactions.insert(&transaction_id, &transaction);
            account.locked_balance = account
              .locked_balance
              .checked_sub(transaction.locked_balance)
              .unwrap();
            account.total_revenue = account
              .total_revenue
              .checked_sub(transaction.locked_balance)
              .unwrap();
            transer_amount += transaction.locked_balance;
          }
        }
      }
      start_epoch += 1;
    }
    Promise::new(sender_id.clone()).transfer(transer_amount);
    self.accounts.insert(&receiver_id, &account);
  }
}
