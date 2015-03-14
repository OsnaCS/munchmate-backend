use db::DatabaseExt;
use rustless::{Nesting, Namespace};
use std;
// use util::{SmartEndpoint};
use super::util::handle;

#[derive(RustcDecodable, RustcEncodable)]
struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    // GeoLocation common.Location
    distance: f64,
}


pub fn route(ns: &mut Namespace) {

    ns.get("info/:id", |endpoint| {
        handle(endpoint, |client, params| {
            let db = client.app.db();

            let stmt = db.prepare(
                r#"SELECT canteens.id, canteens.name, city_id, cities.name, 
                CAST(-1 AS float8) AS "distance"
                FROM canteens
                INNER JOIN cities ON cities.id=city_id
                WHERE canteens.id=$1"#).unwrap();

            let mut vec = std::vec::Vec::new();
            let id : i32 = try!(params.get("id"));
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

            Ok(vec)
        })
    });
}