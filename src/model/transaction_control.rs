use std::collections::HashMap;

use super::{
    definition_control::{AccountType, LedgerAccount},
    ledger::{Amount, LedgerDefinition, Transaction},
};

struct CalculatedAccount {
    name: String,
    current_ammount: Amount,
    account_type: AccountType,
}

struct TransactionResult {
    accounts: HashMap<String, CalculatedAccount>,
    ledger_definition: LedgerDefinition,
}

pub fn check_transactins(
    transactions: &Vec<Transaction>,
    accounts: &HashMap<String, LedgerAccount>,
) {
}
