use crate::*;
use chrono::prelude::*;

#[near_bindgen]
impl HurdlePayment {
  pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
    let account = Account {
      unlocked_balance: 0,
      locked_balance: 0,
      total_revenue: 0,
      last_unlock_at: Utc::now().format("%Y-%m-%d").to_string(),
      transactions: UnorderedMap::new(StorageKey::DateTransactionKey),
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
    let utc_datetime = Utc::now();
    let today_str = Utc::now().format("%Y-%m-%d").to_string();
    let trans = TransferTransaction {
      sender_id: env::predecessor_account_id(),
      receiver_id: receiver_id.clone(),
      campaign_id: campaign_id,
      locked_balance: amount,
      created_at: utc_datetime.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
      claimable_at: (utc_datetime + Duration::days(cash_hold_time))
        .format("%Y-%m-%d %H:%M:%S %Z")
        .to_string(),
      status: "LOCK".to_string(),
    };
    let transactions = account.transactions.get(&today_str);
    if transactions.is_none() {
      let mut map = UnorderedMap::new(StorageKey::TransactionKey);
      map.insert(&transaction_id, &trans);
      account.transactions.insert(&today_str, &map);
    }
    account.locked_balance += amount;
    account.total_revenue += amount;
    self.accounts.insert(&receiver_id, &account);
  }

  pub(crate) fn internal_unlock_locked_balance(&mut self, account_id: AccountId, end_date: String) {
    let account = self.accounts.get(&account_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    let end_date_parse = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();
    let mut last_unlock_at =
      NaiveDate::parse_from_str(&account.last_unlock_at, "%Y-%m-%d").unwrap();
    let utc_datetime = Utc::now();
    while last_unlock_at <= end_date_parse {
      let dt = last_unlock_at.format("%Y-%m-%d").to_string();
      let transactions = account.transactions.get(&dt);
      if transactions.is_some() {
        let mut transactions = transactions.unwrap();
        println!("{}", transactions.len());
        for transaction in transactions.to_vec() {
          let (transaction_id, mut transaction) = transaction;
          if utc_datetime
            > Utc
              .datetime_from_str(&transaction.claimable_at, "%Y-%m-%d %H:%M:%S UTC")
              .unwrap()
          {
            transaction.status = "CLAIM".to_string();
            transactions.insert(&transaction_id, &transaction);
            println!("{}", account.locked_balance);
            println!("{}", transaction.locked_balance);
            account.locked_balance = account
              .locked_balance
              .checked_sub(transaction.locked_balance)
              .unwrap();
            account.unlocked_balance += transaction.locked_balance;
          }
        }
      }
      last_unlock_at += Duration::days(1);
    }
    account.last_unlock_at = end_date;
    self.accounts.insert(&account_id, &account);
  }

  pub(crate) fn internal_withdraw_unlocked_balance(
    &mut self,
    receiver_id: AccountId,
    amount: Balance,
  ) {
    let account = self.accounts.get(&receiver_id);
    assert!(account.is_some(), "Account not found");
    let mut account = account.unwrap();
    assert!(
      amount <= account.unlocked_balance,
      "Can't withdraw amount large than unlocked balance"
    );
    account.unlocked_balance -= amount;
    Promise::new(receiver_id.clone()).transfer(amount);
    self.accounts.insert(&receiver_id, &account);
  }
}
