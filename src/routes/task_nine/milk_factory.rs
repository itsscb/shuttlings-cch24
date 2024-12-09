use std::{
    fmt::{Display, Formatter},
    sync::{atomic::AtomicU32, Arc},
    thread,
};

use axum::{http::StatusCode, response::IntoResponse};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct MilkFactory {
    stock: Arc<AtomicU32>,
}

impl MilkFactory {
    pub fn new() -> Self {
        let mf = Self {
            stock: Arc::new(AtomicU32::new(5)),
        };
        mf.run();
        mf
    }

    pub fn run(&self) {
        thread::spawn({
            let stock = self.stock.clone();
            move || {
                let mut start = std::time::Instant::now();
                loop {
                    if start.elapsed().as_millis() >= 990 {
                        let current_stock = stock.load(std::sync::atomic::Ordering::Relaxed);
                        if current_stock < 5 {
                            stock.store(current_stock + 1, std::sync::atomic::Ordering::Relaxed);
                        }
                        start = std::time::Instant::now();
                    }
                }
            }
        });
    }

    pub fn magic_refill(&self) {
        self.stock.store(5, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn withdraw(&self) -> Result<MilkMessage, MilkMessage> {
        let current_stock = self.stock.load(std::sync::atomic::Ordering::Relaxed);
        if current_stock < 1 {
            return Err(MilkMessage::NoMilkAvailable);
        }
        self.stock
            .store(current_stock - 1, std::sync::atomic::Ordering::Relaxed);
        Ok(MilkMessage::Withdraw)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MilkMessage {
    Withdraw,
    Refill,
    NoMilkAvailable,
    WithdrawUnit(Unit),
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub enum Unit {
    #[serde(rename(deserialize = "liters", serialize = "gallons"))]
    Liters(f32),
    #[serde(rename(deserialize = "gallons", serialize = "liters"))]
    Gallons(f32),
    #[serde(rename(deserialize = "litres", serialize = "pints"))]
    Litres(f32),
    #[serde(rename(deserialize = "pints", serialize = "litres"))]
    Pints(f32),
}

impl Unit {
    pub fn from_json(json: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(json)
    }

    pub fn json(self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(&self)
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Unit", 1)?;
        match self {
            Self::Liters(value) => {
                state.serialize_field::<f32>("gallons", &(*value * 0.264_172_06))?;
            }
            Self::Gallons(value) => {
                state.serialize_field::<f32>("liters", &(*value * 3.785_412))?;
            }
            Self::Litres(value) => {
                state.serialize_field::<f32>("pints", &(*value * 1.759_754))?;
            }
            Self::Pints(value) => {
                state.serialize_field::<f32>("litres", &(*value * 0.568_261))?;
            }
        }
        state.end()
    }
}

#[test]
fn test_unit() {
    let unit = Unit::Liters(5.0);
    assert_eq!(
        serde_json::to_string(&unit).unwrap(),
        r#"{"gallons":1.3208603}"#
    );

    let unit = Unit::from_json(serde_json::json!({"liters": 5.0}).to_string().as_str()).unwrap();
    assert_eq!(unit, Unit::Liters(5.0));
    assert_eq!(unit.json().unwrap(), r#"{"gallons":1.3208603}"#);

    let unit = Unit::from_json(
        serde_json::json!({"gallons": -2.000_000_000_000_001})
            .to_string()
            .as_str(),
    )
    .unwrap();
    assert_eq!(unit, Unit::Gallons(-2.000_000_000_000_001));
    assert_eq!(unit.json().unwrap(), r#"{"liters":-7.570824}"#);
}

impl Display for MilkMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Withdraw => writeln!(f, "Milk withdrawn"),
            Self::Refill => writeln!(f, "Refilled milk"),
            Self::NoMilkAvailable => writeln!(f, "No milk available"),
            Self::WithdrawUnit(u) => write!(f, "{}", u.json().unwrap()),
        }
    }
}

impl IntoResponse for MilkMessage {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Withdraw | Self::WithdrawUnit(_) => (StatusCode::OK, self.to_string()),
            Self::Refill => (StatusCode::OK, String::new()),
            Self::NoMilkAvailable => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
        }
        .into_response()
    }
}
