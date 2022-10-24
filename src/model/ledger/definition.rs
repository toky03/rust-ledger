use crate::model::ledger::amount::Amount;
use serde::{Deserialize, Serialize};
use mockall::automock;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[automock]
pub struct LedgerDefinition {
    balance: Balance,
    income: Income,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Income {
    revenue: Vec<Account>,
    expense: Vec<Account>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Balance {
    active: ActiveBalance,
    passive: PassiveBalance,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ActiveBalance {
    #[serde(rename = "working-capital")]
    working_capital: Vec<Account>,
    #[serde(rename = "fixed-assets")]
    fixed_assets: Vec<Account>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

impl LedgerDefinition {
    pub fn get_equities(&self) -> Vec<&Account> {
        self.balance.passive.equity.iter().map(|acc| acc).collect()
    }
    pub fn get_debt_capital(&self) -> Vec<&Account> {
        self.balance
            .passive
            .debt_capital
            .iter()
            .map(|acc| acc)
            .collect()
    }
    pub fn get_fixed_assets(&self) -> Vec<&Account> {
        self.balance
            .active
            .fixed_assets
            .iter()
            .map(|acc| acc)
            .collect()
    }
    pub fn get_working_capital(&self) -> Vec<&Account> {
        self.balance
            .active
            .working_capital
            .iter()
            .map(|acc| acc)
            .collect()
    }
    pub fn get_revenue(&self) -> Vec<&Account> {
        self.income.revenue.iter().map(|acc| acc).collect()
    }
    pub fn get_expenses(&self) -> Vec<&Account> {
        self.income.expense.iter().map(|acc| acc).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Account, Amount, LedgerDefinition};
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

        verify_account(
            &ledger_definition,
            LedgerDefinition::get_expenses,
            "Aufwand",
            3001,
        );
        verify_account(
            &ledger_definition,
            LedgerDefinition::get_revenue,
            "Ertrag",
            3000,
        );
        verify_account(
            &ledger_definition,
            LedgerDefinition::get_equities,
            "Eigenkapital",
            2000,
        );
        verify_account(
            &ledger_definition,
            LedgerDefinition::get_working_capital,
            "Kasse",
            1000,
        );
        verify_account(
            &ledger_definition,
            LedgerDefinition::get_fixed_assets,
            "Maschinen",
            1001,
        );
        verify_account(
            &ledger_definition,
            LedgerDefinition::get_debt_capital,
            "Fremdkapital",
            2001,
        );
        Ok(())
    }

    #[test]
    fn test_serialization_with_income() -> Result<(), serde_yaml::Error> {
        let expectedStr = r#"balance:
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
        assert_eq!(serialized, expectedStr);
        Ok(())
    }

    fn verify_account(
        ledger: &LedgerDefinition,
        function: fn(definition: &LedgerDefinition) -> Vec<&Account>,
        name: &str,
        start: u128,
    ) -> () {
        assert_eq!(function(&ledger).len(), 1);
        let account = *function(&ledger).first().expect("first account not found");
        assert_eq!(
            account,
            &Account {
                name: name.to_string(),
                start: Amount::new(start, 0),
            }
        )
    }
}
