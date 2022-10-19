use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AccountDefinition {
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Account {
    name: String,
    start: Amount,
}

#[derive(PartialEq, Debug)]
struct Amount {
    pre_decimal: u16,
    decimal_places: u8,
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Amount", 3)?;
        s.serialize_field("pre_decimal", &self.pre_decimal)?;
        //s.collect_str(".")?;
        s.serialize_field("decimal_places", &self.decimal_places)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let v: Vec<&str> = s.split(".").collect();
        let pre_decimal = deserializer.deserialize_u16(v[0])?;
        let decimal_places = deserializer.deserialize_u8(v[1])?;
        Ok(Amount {
            pre_decimal,
            decimal_places,
        })
    }
}