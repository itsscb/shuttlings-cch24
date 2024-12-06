use std::fmt::Display;

use serde::Deserialize;

mod manifest;
pub use manifest::manifest;
use toml::Value as TomlValue;

#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    item: String,
    quantity: u32,
}

impl Order {
    pub const fn new(item: String, quantity: u32) -> Self {
        Self { item, quantity }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.item, self.quantity)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Orders(Vec<Order>);

impl Orders {
    pub const fn new(orders: Vec<Order>) -> Self {
        Self(orders)
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<&Vec<TomlValue>> for Orders {
    fn from(value: &Vec<TomlValue>) -> Self {
        value
            .iter()
            .filter_map(|order| {
                let table = order.as_table()?;
                let item = table.get("item")?.as_str()?.trim_matches('"').to_string();
                let quantity = table.get("quantity")?.as_integer()?;
                let quantity = if u32::try_from(quantity).is_ok() {
                    quantity as u32
                } else {
                    return None;
                };
                Some(Order::new(item, quantity))
            })
            .collect::<Self>()
    }
}

impl std::iter::FromIterator<Order> for Orders {
    fn from_iter<I: IntoIterator<Item = Order>>(iter: I) -> Self {
        let orders: Vec<Order> = iter.into_iter().collect();
        Self::new(orders)
    }
}

impl Display for Orders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = &self.0.len();
        for (i, order) in self.0.iter().enumerate() {
            write!(f, "{order}")?;
            if i + 1 < *len {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_display() {
        let order1 = Order {
            item: "Toy car".to_string(),
            quantity: 2,
        };
        assert_eq!(order1.to_string(), "Toy car: 2");

        let order2 = Order {
            item: "Lego brick".to_string(),
            quantity: 230,
        };
        assert_eq!(order2.to_string(), "Lego brick: 230");

        let order_list = Orders::new(vec![order1, order2]);
        assert_eq!(order_list.to_string(), "Toy car: 2\nLego brick: 230");
    }
}
