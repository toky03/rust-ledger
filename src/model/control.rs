use super::ledger::{Account, LedgerDefinition};
use crate::model::ledger::Amount;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum BalanceType {
    Passive(PassiveType),
    Active(ActiveType),
}

#[derive(Clone, PartialEq, Debug)]
enum PassiveType {
    Equity,
    DebtCapital,
}

#[derive(Clone, PartialEq, Debug)]
enum ActiveType {
    WorkingCapital,
    FixedAssets,
}

#[derive(Clone, PartialEq, Debug)]
enum IncomeType {
    Revenue,
    Expense,
}

#[derive(Clone, PartialEq, Debug)]
enum AccountType {
    Balance(BalanceType),
    Income(IncomeType),
}

#[derive(Clone, PartialEq, Debug)]
pub struct LedgerAccount {
    name: String,
    start: Amount,
    account_type: AccountType,
}

impl LedgerAccount {
    fn from(account: &Account, account_type: &AccountType) -> Self {
        LedgerAccount {
            name: String::from(&account.name),
            start: account.start.clone(),
            account_type: account_type.clone(),
        }
    }
}

pub fn from_ledger_definition(
    ledger_definition: &LedgerDefinition,
) -> HashMap<String, LedgerAccount> {
    let equities = read_accounts(
        ledger_definition,
        LedgerDefinition::get_equities,
        AccountType::Balance(BalanceType::Passive(PassiveType::Equity)),
    );
    let debt_capital = read_accounts(
        ledger_definition,
        LedgerDefinition::get_debt_capital,
        AccountType::Balance(BalanceType::Passive(PassiveType::DebtCapital)),
    );
    let fixed_assets = read_accounts(
        ledger_definition,
        LedgerDefinition::get_fixed_assets,
        AccountType::Balance(BalanceType::Active(ActiveType::FixedAssets)),
    );
    let working_capital = read_accounts(
        ledger_definition,
        LedgerDefinition::get_working_capital,
        AccountType::Balance(BalanceType::Active(ActiveType::WorkingCapital)),
    );
    let expenses = read_accounts(
        ledger_definition,
        LedgerDefinition::get_expenses,
        AccountType::Income(IncomeType::Expense),
    );
    let revenue = read_accounts(
        ledger_definition,
        LedgerDefinition::get_revenue,
        AccountType::Income(IncomeType::Revenue),
    );
    let ledger = [
        &equities[..],
        &debt_capital[..],
        &fixed_assets[..],
        &working_capital[..],
        &expenses[..],
        &revenue[..],
    ]
    .concat();

    ledger
        .into_iter()
        .map(|acc| (acc.name.clone(), acc.clone()))
        .collect()
}

fn read_accounts(
    ledger_definition: &LedgerDefinition,
    function: fn(definition: &LedgerDefinition) -> Vec<&Account>,
    account_type: AccountType,
) -> Vec<LedgerAccount> {
    function(ledger_definition)
        .iter()
        .map(|acc| LedgerAccount::from(acc, &account_type))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::ledger::LedgerDefinition;
    use super::{
        from_ledger_definition, AccountType, ActiveType, BalanceType, IncomeType, LedgerAccount,
        PassiveType,
    };

    use std::collections::HashMap;

    #[test]
    fn test_from_ledger_definition() -> Result<(), serde_yaml::Error> {
        let mock_ledger_definition = MockLedgerDefitition::new();
        mock_ledger_definition
            .expect_get_equities()
            .times(1)
            .resurn_const(());
        let ledger_accounts = from_ledger_definition(&mock_ledger_definition);
        assert_eq!(ledger_accounts.len(), 6);
        verify_account(
            &ledger_accounts,
            "Kasse",
            AccountType::Balance(BalanceType::Active(ActiveType::WorkingCapital)),
        );
        verify_account(
            &ledger_accounts,
            "Maschinen",
            AccountType::Balance(BalanceType::Active(ActiveType::FixedAssets)),
        );
        verify_account(
            &ledger_accounts,
            "Eigenkapital",
            AccountType::Balance(BalanceType::Passive(PassiveType::Equity)),
        );
        verify_account(
            &ledger_accounts,
            "Fremdkapital",
            AccountType::Balance(BalanceType::Passive(PassiveType::DebtCapital)),
        );
        verify_account(
            &ledger_accounts,
            "Ertrag",
            AccountType::Income(IncomeType::Revenue),
        );
        verify_account(
            &ledger_accounts,
            "Aufwand",
            AccountType::Income(IncomeType::Expense),
        );
        Ok(())
    }

    fn verify_account(
        ledger_accounts: &HashMap<String, LedgerAccount>,
        name: &str,
        account_type: AccountType,
    ) {
        let account = ledger_accounts
            .get(name)
            .expect(format!("cannot find account with name {name}").as_str());
        assert_eq!(account.account_type, account_type);
    }
}
