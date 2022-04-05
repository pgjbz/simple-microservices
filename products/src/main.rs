use std::fs;

use rocket::serde::{json::Json, Serialize, Deserialize};

use serde::{de, Deserializer};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct Product {
    uuid: String,
    product: String,
    #[serde(deserialize_with = "de_from_str")]
    price: f64,
}

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.trim().parse::<f64>().map_err(de::Error::custom)
}

#[derive( Deserialize)]
struct ProductWrapper {
    products: Vec<Product>
}

fn load_products() -> Vec<Product> {
    let json = fs::read_to_string("products.json").unwrap();
    let products: ProductWrapper = serde_json::from_str(&json).unwrap();
    products.products
}

#[get("/products")]
fn find_all() -> Json<Vec<Product>> {
    Json::from(load_products())
}

#[get("/product/<id>")]
fn find_by_id(id: &str) -> Json<Option<Product>> {
    for product in load_products() {
        if product.uuid == id {
            return Json::from(Some(product))
        }
    }
    Json::from(None)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![find_all, find_by_id])
}
