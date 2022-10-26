use super::amount_entity::AmountEntity;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

trait TransactionAccountReader {
    fn read_accounts(&self) -> Vec<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionEntity {
    description: String,
    amount: AmountEntity,
    deb: String,
    cred: String,
    date: NaiveDate,
}

impl TransactionAccountReader for TransactionEntity {
    fn read_accounts(&self) -> Vec<String> {
        vec![self.deb.clone(), self.cred.clone()]
    }
}

impl TransactionEntity {
    pub fn new(
        description: String,
        amount: AmountEntity,
        deb: String,
        cred: String,
        date: NaiveDate,
    ) -> Self {
        TransactionEntity {
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
    use crate::model::entity::transaction_entity::TransactionEntity;
    use crate::model::entity::AmountEntity;
    use chrono::NaiveDate;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TransactionsWrapper {
        transactions: Vec<TransactionEntity>,
    }

    #[test]
    fn test_deserialization() -> Result<(), serde_yaml::Error> {
        let definition = r#"
transactions:
  - date: 2022-01-01
    cred: Aufwand
    deb: Maschinen
    amount: 100.0
    description: Abschreibungen von Maschinen"#;

        let ledger_transactions: TransactionsWrapper = serde_yaml::from_str(definition)?;

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

        let ledger_transactions = TransactionsWrapper {
            transactions: vec![TransactionEntity::new(
                String::from("Kauf von Maschinen aus Kasse"),
                AmountEntity::new(20, 22),
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
