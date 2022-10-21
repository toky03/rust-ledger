use crate::model::definition::LedgerDefinition;

pub fn read_default_ledger() -> Result<LedgerDefinition, serde_yaml::Error> {
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
    Ok(ledger_definition)
}
