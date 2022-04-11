use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    uuid: String,
    name: String,
    email: String,
    phone: String,
    product_id: String,
    pub status: Status,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    PENDING,
    APPROVED,
    REPROVED,
}

impl From<u8> for Status {
    fn from(input: u8) -> Self {
        match input {
            // 1 => Self::PENDING, //Just ignore PENDING status, just simulate payment processment
            1..=2 => Self::APPROVED,
            _ => Self::REPROVED,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match self {
            Self::PENDING => "PENDING",
            Self::APPROVED => "APPROVED",
            Self::REPROVED => "REPROVED",
        };
        writeln!(f, "{}", to_print)
    }
}
