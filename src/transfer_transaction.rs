use crate::*;
use near_sdk::Timestamp;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferTransaction {
  pub account_id: AccountId,
  pub locked_balance: Balance,
  pub transaction_start_at: Timestamp,
  pub transaction_unlock_at: Timestamp,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferTransactionJson {
  pub account_id: AccountId,
  pub locked_balance: U128,
  pub transaction_start_at: Timestamp,
  pub transaction_unlock_at: Timestamp,
}

impl TransferTransactionJson {
  pub fn from(account_id: AccountId, transaction: TransferTransaction) -> Self {
    TransferTransactionJson {
      account_id,
      locked_balance: U128(transaction.locked_balance),
      transaction_start_at: transaction.transaction_start_at,
      transaction_unlock_at: transaction.transaction_unlock_at,
    }
  }
}

// code .
