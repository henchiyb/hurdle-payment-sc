use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferTransaction {
  pub sender_id: AccountId,
  pub receiver_id: AccountId,
  pub campaign_id: String,
  pub locked_balance: Balance,
  pub created_at: String,
  pub claimable_at: String,
  pub status: String, // LOCK UNLOCK REFUND
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferTransactionJson {
  pub transaction_id: String,
  pub sender_id: AccountId,
  pub receiver_id: AccountId,
  pub campaign_id: String,
  pub locked_balance: U128,
  pub created_at: String,
  pub claimable_at: String,
  pub status: String, // LOCK UNLOCK REFUND
}

impl TransferTransactionJson {
  pub fn from(transaction_id: String, transaction: TransferTransaction) -> Self {
    TransferTransactionJson {
      transaction_id: transaction_id,
      sender_id: transaction.sender_id,
      receiver_id: transaction.receiver_id,
      campaign_id: transaction.campaign_id,
      locked_balance: U128(transaction.locked_balance),
      created_at: transaction.created_at,
      claimable_at: transaction.claimable_at,
      status: transaction.status,
    }
  }
}

// code .
