use super::Amount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    description: String,
    amount: Amount,
    deb: String,
    cred: String,
    date: NaiveDate,
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

        let ledgerTransactions: TransactionLedger = serde_yaml::from_str(definition)?;

        assert_eq!(ledgerTransactions.transactions.len(), 1);
        Ok(())
    }

    #[test]
    fn test_serialization() -> Result<(), serde_yaml::Error> {
        let expectedStr = r#"transactions:
- description: Kauf von Maschinen aus Kasse
  amount: 20.22
  deb: Kasse
  cred: Maschinen
  date: 2022-01-10
"#;

        let ledgerTransactions = TransactionLedger {
            transactions: vec![Transaction::new(
                String::from("Kauf von Maschinen aus Kasse"),
                Amount::new(20, 22),
                String::from("Kasse"),
                String::from("Maschinen"),
                NaiveDate::from_ymd(2022, 1, 10),
            )],
        };

        let serialized = serde_yaml::to_string(&ledgerTransactions)?;

        assert_eq!(serialized, expectedStr);
        Ok(())
    }
}