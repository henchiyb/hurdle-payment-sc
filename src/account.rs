use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]

pub struct Account {
  pub unlocked_balance: Balance,
  pub locked_balance: Balance,
  pub total_revenue: Balance,
  pub transactions: UnorderedMap<u64, UnorderedMap<String, TransferTransaction>>, // date_string: { transaction_id: TransferTransaction}
  pub last_unlock_at: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
  pub account_id: AccountId,
  pub unlocked_balance: U128,
  pub locked_balance: U128,
  pub total_revenue: U128,
  pub last_unlock_at: u64,
}

impl AccountJson {
  pub fn from(account_id: AccountId, account: Account) -> Self {
    AccountJson {
      account_id,
      unlocked_balance: U128(account.unlocked_balance),
      locked_balance: U128(account.locked_balance),
      total_revenue: U128(account.total_revenue),
      last_unlock_at: account.last_unlock_at,
    }
  }
}

// code .
