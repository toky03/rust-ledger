use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Debug, Clone)]
pub struct Amount {
    decimal_digits: u128,
    dividend_digits: u32,
}

impl Amount {
    pub fn new(decimal_digits: u128, dividend_digits: u32) -> Self {
        Amount {
            decimal_digits,
            dividend_digits,
        }
    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = format!("{}.{}", self.decimal_digits, self.dividend_digits);
        let parsed: f64 = v.parse().map_err(serde::ser::Error::custom)?;
        serializer.serialize_f64(parsed)
    }
}

struct AmountVisitor;

impl<'de> Visitor<'de> for AmountVisitor {
    type Value = Amount;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an amount with decimal places like 10.00")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let parts: Vec<&str> = v.split(".").collect();
        let pre_decimal: u128 = parts[0].parse().map_err(serde::de::Error::custom)?;
        let decimal_places: u32 = if parts.len() > 1 && parts[1].trim_matches('0').len() > 0 {
            parts[1]
                .trim_matches('0')
                .parse()
                .map_err(serde::de::Error::custom)?
        } else {
            0
        };
        Ok(Amount::new(pre_decimal, decimal_places))
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AmountVisitor)
    }
}

#[cfg(test)]
mod test_deserialize_amount {
    #[test]
    fn test_amount_witout_decimal_digits() -> Result<(), serde_yaml::Error> {
        let amount: super::Amount = serde_yaml::from_str(r#"10"#)?;
        assert_eq!(amount, super::Amount::new(10, 0));
        Ok(())
    }

    #[test]
    fn test_amount_with_decimal_digits() -> Result<(), serde_yaml::Error> {
        let amount: super::Amount = serde_yaml::from_str(r#"10.0"#)?;
        assert_eq!(amount, super::Amount::new(10, 0));
        Ok(())
    }

    #[test]
    fn test_amount_with_decimal_digits_non_zero() -> Result<(), serde_yaml::Error> {
        let amount: super::Amount = serde_yaml::from_str(r#"999.990"#)?;
        assert_eq!(amount, super::Amount::new(999, 99));
        Ok(())
    }

    #[test]
    fn test_amount_with_decimal_digits_leading_zero() -> Result<(), serde_yaml::Error> {
        let amount: super::Amount = serde_yaml::from_str(r#"011.0"#)?;
        assert_eq!(amount, super::Amount::new(11, 0));
        Ok(())
    }
}

#[cfg(test)]
mod test_serialize_amount {
    #[test]
    fn test_amount_witout_decimal_digits() -> Result<(), serde_yaml::Error> {
        let amount: String = serde_yaml::to_string(&super::Amount::new(10, 0))?;
        assert_eq!(amount, "10.0\n");
        Ok(())
    }

    #[test]
    fn test_amount_with_decimal_digits_non_zero() -> Result<(), serde_yaml::Error> {
        let amount: String = serde_yaml::to_string(&super::Amount::new(999, 99))?;
        assert_eq!(amount, "999.99\n");
        Ok(())
    }
}
