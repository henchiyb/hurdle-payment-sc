use crate::*;
use near_sdk::Timestamp;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
  pub unlock_balance: Balance,
  pub locked_balance: Balance,
  pub total_balance: Balance,
  pub available_epoch: EpochHeight,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
  pub account_id: AccountId,
  pub unlock_balance: U128,
  pub locked_balance: U128,
  pub total_balance: U128,
  pub available_epoch: EpochHeight,
}

impl AccountJson {
  pub fn from(account_id: AccountId, account: Account) -> Self {
    AccountJson {
      account_id,
      unlock_balance: U128(account.unlock_balance),
      locked_balance: U128(account.locked_balance),
      total_balance: U128(account.total_balance),
      available_epoch: account.available_epoch,
    }
  }
}

// code .
