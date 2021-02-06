use rocket::fairing::{Fairing, Info, Kind};


pub struct JsonApiFairing;

impl Fairing for JsonApiFairing {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Attach,
            name: "JsonApiFairing",
        }
    }
}
