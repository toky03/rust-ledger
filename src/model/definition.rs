use crate::model::amount::Amount;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
mod test_deserialize_definition {
    use crate::model::amount::Amount;
    use crate::model::definition::{Account, LedgerDefinition};
    use crate::model::mock::read_default_ledger;

    #[test]
    fn test_with_getter() -> Result<(), serde_yaml::Error> {
        let ledger_definition = read_default_ledger()?;

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
