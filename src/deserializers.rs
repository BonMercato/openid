use std::fmt::Display;
use std::str::FromStr;
use de::Visitor;
use serde::{de, Deserialize};
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

pub fn deserialize_option_number_from_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + serde::Deserialize<'de>,
        <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumericOrNull<'a, T> {
        Str(&'a str),
        String(String),
        Numeric(T),
        Null,
    }

    match NumericOrNull::<T>::deserialize(deserializer)? {
        NumericOrNull::Str(s) => match s {
            "" => Ok(None),
            _ => T::from_str(s).map(Some).map_err(serde::de::Error::custom),
        },
        NumericOrNull::String(s) => match s.as_str() {
            "" => Ok(None),
            _ => T::from_str(s.as_str()).map(Some).map_err(serde::de::Error::custom),
        },
        NumericOrNull::Numeric(i) => Ok(Some(i)),
        NumericOrNull::Null => Ok(None),
    }
}
