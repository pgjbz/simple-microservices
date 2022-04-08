use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    uuid: String,
    name: String,
    email: String,
    phone: String,
    product_id: String,
    status: Status,
    created_at: String
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    PENDING,
    APPROVED,
    REPROVED
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match self {
            Self::PENDING => "PENDING",
            Self::APPROVED => "APPROVED",
            Self::REPROVED => "REPROVED"
        };
        writeln!(f, "{}", to_print)
    }
}