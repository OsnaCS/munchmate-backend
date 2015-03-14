#![allow(unused_imports)]
use db::DatabaseExt;
use rustc_serialize::json::{self, Json};
use rustless;
use rustless::{Client, HandleResult, Nesting};
use iron::mime;
use std;
use util::{SmartEndpoint};

#[derive(RustcDecodable, RustcEncodable)]
struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    // GeoLocation common.Location
    distance: f64,
}

// struct SmartParam {
//     bla: i32
// }

// fn smart<F: 'static>(func: F)
//     where F: for<'a> Fn(Client<'a>, SmartParam) -> HandleResult<Client<'a>> {
// }

pub fn route(ns: &mut rustless::Namespace) {

    ns.get("info/:id", |endpoint| {
        endpoint.handle_smart(|client, params| {
            // let db = client.app.db();

            // let stmt = db.prepare(
            //     r#"SELECT canteens.id, canteens.name, city_id, cities.name, 
            //     CAST(-1 AS float8) AS "distance"
            //     FROM canteens
            //     INNER JOIN cities ON cities.id=city_id
            //     WHERE canteens.id=$1"#).unwrap();

            // let mut vec = std::vec::Vec::new();
            let id : i32 = try!(params.get("id"));
            // let id : i32 = 
            // for row in stmt.query(&[&id]).unwrap() {
            //     let canteen = Canteen {
            //         id: row.get(0),
            //         name: row.get(1),
            //         city_id: row.get(2),
            //         city_name: row.get(3),
            //         distance: row.get(4),
            //     };
            //     vec.push(canteen);
            // }
            // client.set_content_type(mime::Mime(
            //     mime::TopLevel::Application, 
            //     mime::SubLevel::Json, 
            //     vec![(mime::Attr::Charset, mime::Value::Utf8)]
            // ));
            Ok(id)
            // Ok(client.text(json::encode(&id).unwrap()))
            // client.text(json::encode(&vec).unwrap())
        })
    });
}