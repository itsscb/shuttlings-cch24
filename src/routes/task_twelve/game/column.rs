use serde::de::{self, Deserializer, Visitor};
use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column(usize);

impl Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Column> for usize {
    fn from(column: Column) -> Self {
        column.0
    }
}

impl<'de> Deserialize<'de> for Column {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColumnVisitor;

        impl Visitor<'_> for ColumnVisitor {
            type Value = Column;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between 1 and 4")
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    1..=4 => Ok(Column(value as usize)),
                    _ => Err(de::Error::custom("value must be between 1 and 4")),
                }
            }
        }

        deserializer.deserialize_u8(ColumnVisitor)
    }
}
