extern crate json_api;
#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate serde;
extern crate serde_json;

mod error;
mod fairing;

mod config {
    lazy_static! {
        pub static ref ROCKET_ENV: rocket::Config = rocket::Config::default();
    }
}

pub mod request;
pub mod response;

pub use self::fairing::JsonApiFairing;
pub use self::request::*;
pub use self::response::*;
