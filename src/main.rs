#![feature(plugin)]
// #![allow(dead_code)]


extern crate iron;
extern crate openssl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rustless;
extern crate typemap;

use db::DatabaseExt;
use iron::Iron;
use postgres::{Connection, SslMode};
use rustless::{Application, Api, Nesting, Versioning};
use rustc_serialize::json;
use std::borrow::Borrow;
    use std::str::FromStr;


mod db;

#[derive(RustcDecodable, RustcEncodable)]
struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    // GeoLocation common.Location
    distance: f64,
}


fn main() {
    let (port, db_url) = match fetch_env() {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };
    
    let db = match db::setup(db_url.borrow(), 1) {
        Some(result) => result,
        None => return,
    };

    // let stmt = db.prepare(
    //     r#"SELECT canteens.id, canteens.name, city_id, cities.name, 
    //     CAST(-1 AS float8) AS "distance"
    //     FROM canteens
    //     INNER JOIN cities ON cities.id=city_id
    //     WHERE canteens.id=2"#).unwrap();

    // let mut vec = std::vec::Vec::new();
    // for row in stmt.query(&[]).unwrap() {
    //     let canteen = Canteen {
    //         id: row.get(0),
    //         name: row.get(1),
    //         city_id: row.get(2),
    //         city_name: row.get(3),
    //         distance: row.get(4),
    //     };
    //     vec.push(canteen);
    // }
    // println!("{}", json::encode(&vec).unwrap());

    let api = Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::Path);
        // api.prefix("api");

        api.get("info/:id", |endpoint| {
            endpoint.handle(|client, params| {
                let db = client.app.db();

                let stmt = db.prepare(
                    r#"SELECT canteens.id, canteens.name, city_id, cities.name, 
                    CAST(-1 AS float8) AS "distance"
                    FROM canteens
                    INNER JOIN cities ON cities.id=city_id
                    WHERE canteens.id=$1"#).unwrap();

                let mut vec = std::vec::Vec::new();
                let id : i32 = FromStr::from_str(
                    params.find("id").unwrap().as_string().unwrap()).unwrap();
                for row in stmt.query(&[&id]).unwrap() {
                    let canteen = Canteen {
                        id: row.get(0),
                        name: row.get(1),
                        city_id: row.get(2),
                        city_name: row.get(3),
                        distance: row.get(4),
                    };
                    vec.push(canteen);
                }
                client.text(json::encode(&vec).unwrap())
                // client.text(params.find("id")
                //     .unwrap().as_string().unwrap().to_string())
            })
        });
       
    });

    let mut app = Application::new(api);
    app.ext.insert::<db::AppDB>(db);

    println!("Listening on port {}", port);
    Iron::new(app).http(("localhost", port)).unwrap();
}


// Fetches all needed environment variables
fn fetch_env() -> Result<(u16, String), String> {
    use std::str::FromStr;
    use std::env;

    // Try to get 'PORT' and convert it to u16
    let port = match env::var("PORT") {
        Err(_) => return Err("Env-Var 'PORT' is not set!".to_string()),
        Ok(val) => match FromStr::from_str(val.borrow()) {
            Err(_) => return Err("Env-Var 'PORT' is not an integer!"
                .to_string()),
            Ok(p) => p,
        },
    };

    // Try to get the database connection string
    let db_url = match env::var("DATABASE_URL") {
        Err(_) => return Err("Env-Var 'DATABASE_URL' is not set!".to_string()),
        Ok(val) => val,
    };

    Ok((port, db_url))
}