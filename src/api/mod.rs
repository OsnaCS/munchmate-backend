use db::DatabaseExt;
use rustc_serialize::json;
use rustless;
use rustless::{Api, Nesting, Versioning};
use iron::mime;
use std;
use std::str::FromStr;


#[derive(RustcDecodable, RustcEncodable)]
struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    // GeoLocation common.Location
    distance: f64,
}




pub fn root() -> rustless::Api {
    Api::build(|api| {
        // Specify API version
        api.version("v1", Versioning::Path);
        // api.prefix("api");

        api.get("info/:id", |endpoint| {
            endpoint.handle(|mut client: _, params| {
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
                client.set_content_type(mime::Mime(
                    mime::TopLevel::Application, 
                    mime::SubLevel::Json, 
                    vec![(mime::Attr::Charset, mime::Value::Utf8)]
                ));
                client.text(json::encode(&vec).unwrap())
            })
        });
       
    })
}