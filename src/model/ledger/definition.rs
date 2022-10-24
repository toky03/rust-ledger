use crate::model::ledger::amount::Amount;
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LedgerDefinition {
    balance: Balance,
    income: Income,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Income {
    revenue: Vec<Account>,
    expense: Vec<Account>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Balance {
    active: ActiveBalance,
    passive: PassiveBalance,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct ActiveBalance {
    #[serde(rename = "working-capital")]
    working_capital: Vec<Account>,
    #[serde(rename = "fixed-assets")]
    fixed_assets: Vec<Account>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct PassiveBalance {
    equity: Vec<Account>,
    #[serde(rename = "debt-capital")]
    debt_capital: Vec<Account>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Account {
    pub name: String,
    pub start: Amount,
}

#[cfg(test)]
impl Account {
    pub fn new(name: &str, start: u128) -> Self {
        Account {
            name: String::from(name),
            start: Amount::new(start, 0),
        }
    }
}

pub trait AccountsReader {
    fn get_equities(&self) -> Vec<Account>;
    fn get_debt_capital(&self) -> Vec<Account>;
    fn get_fixed_assets(&self) -> Vec<Account>;
    fn get_working_capital(&self) -> Vec<Account>;
    fn get_revenue(&self) -> Vec<Account>;
    fn get_expenses(&self) -> Vec<Account>;
}

#[cfg_attr(test, automock)]
impl AccountsReader for LedgerDefinition {
    fn get_equities<'a>(&'a self) -> Vec<Account> {
        self.balance
            .passive
            .equity
            .iter()
            .map(|e| e.clone())
            .collect()
    }
    fn get_debt_capital<'a>(&'a self) -> Vec<Account> {
        self.balance
            .passive
            .debt_capital
            .iter()
            .map(|e| e.clone())
            .collect()
    }
    fn get_fixed_assets<'a>(&'a self) -> Vec<Account> {
        self.balance
            .active
            .fixed_assets
            .iter()
            .map(|e| e.clone())
            .collect()
    }
    fn get_working_capital<'a>(&'a self) -> Vec<Account> {
        self.balance
            .active
            .working_capital
            .iter()
            .map(|e| e.clone())
            .collect()
    }
    fn get_revenue<'a>(&'a self) -> Vec<Account> {
        self.income.revenue.iter().map(|e| e.clone()).collect()
    }
    fn get_expenses<'a>(&'a self) -> Vec<Account> {
        self.income.expense.iter().map(|e| e.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Account, AccountsReader, Amount, LedgerDefinition};
    use crate::model::ledger::definition::{ActiveBalance, Balance, Income, PassiveBalance};

    #[test]
    fn test_with_getter() -> Result<(), serde_yaml::Error> {
        let definition = r#"
balance:
  active:
    working-capital:
      - name: Kasse
        start: 1000
    fixed-assets:
      - name: Maschinen
        start: 1001
  passive:
    equity:
      - name: Eigenkapital
        start: 2000
    debt-capital:
      - name: Fremdkapital
        start: 2001
income:
  revenue:
    - name: Ertrag
      start: 3000
  expense:
    - name: Aufwand
      start: 3001"#;

        let ledger_definition: LedgerDefinition = serde_yaml::from_str(definition)?;

        verify_account(ledger_definition.get_expenses(), "Aufwand", 3001);
        verify_account(ledger_definition.get_revenue(), "Ertrag", 3000);
        verify_account(ledger_definition.get_equities(), "Eigenkapital", 2000);
        verify_account(ledger_definition.get_working_capital(), "Kasse", 1000);
        verify_account(ledger_definition.get_fixed_assets(), "Maschinen", 1001);
        verify_account(ledger_definition.get_debt_capital(), "Fremdkapital", 2001);
        Ok(())
    }

    #[test]
    fn test_serialization_with_income() -> Result<(), serde_yaml::Error> {
        let expected_str = r#"balance:
  active:
    working-capital: []
    fixed-assets: []
  passive:
    equity: []
    debt-capital: []
income:
  revenue: []
  expense: []
"#;
        let definition = LedgerDefinition {
            balance: Balance {
                active: ActiveBalance {
                    working_capital: vec![],
                    fixed_assets: vec![],
                },
                passive: PassiveBalance {
                    equity: vec![],
                    debt_capital: vec![],
                },
            },
            income: Income {
                revenue: vec![],
                expense: vec![],
            },
        };

        let serialized = serde_yaml::to_string(&definition)?;
        assert_eq!(serialized, expected_str);
        Ok(())
    }

    fn verify_account(accounts: Vec<Account>, name: &str, start: u128) -> () {
        assert_eq!(accounts.len(), 1);
        let account = accounts.first().expect("first account not found");
        assert_eq!(
            account,
            &Account {
                name: name.to_string(),
                start: Amount::new(start, 0),
            }
        )
    }
}
