use crate::*;

pub(crate) fn assert_at_least_one_yocto() {
  assert!(
    env::attached_deposit() >= 1,
    "Requir attached deposit at least 1 yoctoNEAR"
  )
}

pub(crate) fn assert_one_yocto() {
  assert_eq!(env::attached_deposit(), 1, "Attached 1 yoctoNEAR");
}

pub(crate) fn refund_deposit(amount: Balance, storaged_use: u64) {
  let required_cost = env::storage_byte_cost() * Balance::from(storaged_use) + amount;
  let attached_deposit = env::attached_deposit();

  assert!(
    attached_deposit >= required_cost,
    "Must attach {} yoctoNear to cover starage",
    required_cost
  );

  let refund = attached_deposit - required_cost;
  if refund > 0 {
    Promise::new(env::predecessor_account_id()).transfer(refund);
  }
}
