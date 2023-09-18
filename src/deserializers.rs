use de::Visitor;
use serde::de;
use serde::Deserializer;

pub fn bool_from_str_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolOrStringVisitor)
}

struct BoolOrStringVisitor;

impl<'de> Visitor<'de> for BoolOrStringVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean or string of \"true\", \"false\".")
    }

    fn visit_bool<E>(self, value: bool) -> Result<bool, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<bool, E>
    where
        E: de::Error,
    {
        match value {
            "true" => Ok(true),
            "false" => Ok(false),
            _s => Err(E::custom(format!("Unknown string value: {}", _s))),
        }
    }
}

pub fn i64_from_str_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
{
    deserializer.deserialize_any(I64OrStringVisitor)
}

struct I64OrStringVisitor;

impl<'de> Visitor<'de> for I64OrStringVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a i64 or string of i64.")
    }

    fn visit_i64<E>(self, value: i64) -> Result<i64, E>
        where
            E: de::Error,
    {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<i64, E>
        where
            E: de::Error,
    {
        match value.parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(E::custom(format!("Unknown string value: {}", value))),
        }
    }
}

// also deserialize i64 or string to Option<i64>
pub fn option_i64_from_str_or_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserializer.deserialize_any(OptionI64OrStringVisitor)
}

struct OptionI64OrStringVisitor;

impl<'de> Visitor<'de> for OptionI64OrStringVisitor {
    type Value = Option<i64>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a i64 or string of i64.")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Option<i64>, E>
        where
            E: de::Error,
    {
        Ok(Some(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Option<i64>, E>
        where
            E: de::Error,
    {
        match value.parse::<i64>() {
            Ok(i) => Ok(Some(i)),
            Err(_) => Err(E::custom(format!("Unknown string value: {}", value))),
        }
    }
}
