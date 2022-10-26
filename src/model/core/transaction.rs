use super::super::core::account::{AccountType, LedgerAccount};
use super::super::error::Result;
use crate::model::entity::{
    AmountEntity, LedgerEntity, TransactionAccountReader, TransactionEntity,
};
use crate::model::error::AccError;
use std::collections::HashMap;

struct CalculatedAccount {
    name: String,
    current_ammount: AmountEntity,
    account_type: AccountType,
}

struct TransactionResult {
    accounts: HashMap<String, CalculatedAccount>,
    ledger_definition: LedgerEntity,
}

pub fn check_transactions(
    transactions: &Vec<Box<dyn TransactionAccountReader>>,
    accounts: &HashMap<String, LedgerAccount>,
) -> Result<()> {
    for transaction in transactions.iter() {
        if !accounts.contains_key(transaction.read_creditor_account()) {
            return Err(AccError::new(format!(
                "creditor Account {} from Transaction {} not found in defined accounts",
                transaction.read_creditor_account().to_owned(),
                transaction.read_description().to_owned()
            )));
        }
        if !accounts.contains_key(transaction.read_debitor_account()) {
            return Err(AccError::new(format!(
                "debitor Account {} from Transaction {} not found in defined accounts",
                transaction.read_debitor_account().to_owned(),
                transaction.read_description().to_owned()
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::entity::MockTransactionEntity;
    use super::super::super::error::Result;
    use crate::model::core::account::LedgerAccount;
    use std::collections::HashMap;
    #[test]
    fn test_check_transactions() -> Result<()> {
        let creditor_account = "Cred";
        let debitor_account = "Deb";
        let mut mock_transaction = MockTransactionEntity::new();
        let dummy_ledger_account = LedgerAccount::dummy();

        let m: HashMap<String, LedgerAccount> = HashMap::from([
            ("Cred".to_string(), dummy_ledger_account.clone()),
            ("Deb".to_string(), dummy_ledger_account.clone()),
        ]);

        mock_transaction
            .expect_read_creditor_account()
            .return_const(creditor_account.to_owned());

        mock_transaction
            .expect_read_debitor_account()
            .return_const(debitor_account.to_owned());

        assert_eq!(
            super::check_transactions(&vec![Box::new(mock_transaction)], &m).is_err(),
            false
        );
        Ok(())
    }
}
