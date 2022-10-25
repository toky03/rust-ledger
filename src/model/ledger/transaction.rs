use super::Amount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

trait TransactionAccountReader {
    fn read_accounts(&self) -> Vec<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    description: String,
    amount: Amount,
    deb: String,
    cred: String,
    date: NaiveDate,
}

impl TransactionAccountReader for Transaction {
    fn read_accounts(&self) -> Vec<String> {
        vec![self.deb.clone(), self.cred.clone()]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionLedger {
    pub transactions: Vec<Transaction>,
}

impl Transaction {
    pub fn new(
        description: String,
        amount: Amount,
        deb: String,
        cred: String,
        date: NaiveDate,
    ) -> Self {
        Transaction {
            description,
            amount,
            deb,
            cred,
            date,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::ledger::transaction::Transaction;
    use crate::model::ledger::{Amount, TransactionLedger};
    use chrono::NaiveDate;

    #[test]
    fn test_deserialization() -> Result<(), serde_yaml::Error> {
        let definition = r#"
transactions:
  - date: 2022-01-01
    cred: Aufwand
    deb: Maschinen
    amount: 100.0
    description: Abschreibungen von Maschinen"#;

        let ledger_transactions: TransactionLedger = serde_yaml::from_str(definition)?;

        assert_eq!(ledger_transactions.transactions.len(), 1);
        Ok(())
    }

    #[test]
    fn test_serialization() -> Result<(), serde_yaml::Error> {
        let expected_str = r#"transactions:
- description: Kauf von Maschinen aus Kasse
  amount: 20.22
  deb: Kasse
  cred: Maschinen
  date: 2022-01-10
"#;

        let ledger_transactions = TransactionLedger {
            transactions: vec![Transaction::new(
                String::from("Kauf von Maschinen aus Kasse"),
                Amount::new(20, 22),
                String::from("Kasse"),
                String::from("Maschinen"),
                NaiveDate::from_ymd(2022, 1, 10),
            )],
        };

        let serialized = serde_yaml::to_string(&ledger_transactions)?;

        assert_eq!(serialized, expected_str);
        Ok(())
    }
}
