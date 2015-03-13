#![feature(plugin)]
// #![allow(dead_code)]


extern crate iron;
extern crate postgres;
extern crate rustless;
extern crate openssl;
extern crate "rustc-serialize" as rustc_serialize;

use iron::Iron;
use openssl::ssl::{SslContext, SslMethod};
use postgres::{Connection, SslMode};
use rustless::{Application, Api, Nesting, Versioning};
use rustc_serialize::json;
use std::borrow::Borrow;

#[derive(RustcDecodable, RustcEncodable)]
struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    // GeoLocation common.Location
    distance: i32,
}


fn main() {
    let (port, db_url) = match fetch_env() {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };
    let ssl_mode = match SslContext::new(SslMethod::Sslv23) {
        Ok(val) => SslMode::Prefer(val),
        Err(e) => {
            println!("ERROR: Creating SSL-context failed!");
            println!("    => {}", e);
            return;
        }
    };

    let db = match Connection::connect(db_url.borrow(), &ssl_mode) {
        Ok(val) => val,
        Err(e) => {
            println!("ERROR: Connection to database failed!");
            println!("    => {}", e);
            return;
        }
    };

    let stmt = db.prepare(
        r#"SELECT canteens.id, canteens.name, city_id, cities.name, 
        -1 AS "distance"
        FROM canteens
        INNER JOIN cities ON cities.id=city_id
        WHERE canteens.id=2"#).unwrap();

    let mut vec = std::vec::Vec::new();
    for row in stmt.query(&[]).unwrap() {
        let canteen = Canteen {
            id: row.get(0),
            name: row.get(1),
            city_id: row.get(2),
            city_name: row.get(3),
            distance: row.get(4),
        };
        vec.push(canteen);
    }
    println!("{}", json::encode(&vec).unwrap());

    let api = Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::Path);
        // api.prefix("api");

        api.get("info/:id", |endpoint| {
            endpoint.handle(|client, params| {
                client.text(params.find("id")
                    .unwrap().as_string().unwrap().to_string())
            })
        });
       
    });

    let app = Application::new(api);

    println!("Listening on port {}", port);
    Iron::new(app).http(("localhost", port)).unwrap();
}


fn fetch_env() -> Result<(u16, String), String> {
    use std::str::FromStr;
    use std::env;

    let port = match env::var("PORT") {
        Err(_) => return Err("Env-Var 'PORT' is not set!".to_string()),
        Ok(val) => match FromStr::from_str(val.borrow()) {
            Err(_) => return Err("Env-Var 'PORT' is not an integer!"
                .to_string()),
            Ok(p) => p,
        },
    };

    let db_url = match env::var("DATABASE_URL") {
        Err(_) => return Err("Env-Var 'DATABASE_URL' is not set!".to_string()),
        Ok(val) => val,
    };

    Ok((port, db_url))
}