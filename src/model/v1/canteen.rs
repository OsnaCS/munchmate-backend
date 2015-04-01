use api::ApiError;
use db::PooledDBConn;
use hyper::status::StatusCode;
use std::error::Error;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    // GeoLocation common.Location
    distance: f64,
}

impl Canteen {
    pub fn get_by_id(db: PooledDBConn, id: i32) -> Result<Canteen, ApiError> {
        let stmt = db.prepare(
            r#"SELECT canteens.id, canteens.name, city_id, cities.name,
            CAST(-1 AS float8) AS "distance"
            FROM canteens
            INNER JOIN cities ON cities.id=city_id
            WHERE canteens.id=$1"#).unwrap();

        let rows = match stmt.query(&[&id]) {
            Err(e) => return Err(ApiError::detailed(
                StatusCode::InternalServerError,
                "Query failed!".to_string(),
                e.description().to_string())),
            Ok(rows) => rows
        };

        match rows.iter().next() {
            Some(row) => Ok(Canteen {
                            id: row.get(0),
                            name: row.get(1),
                            city_id: row.get(2),
                            city_name: row.get(3),
                            distance: row.get(4),
                        }),
            None => Err(ApiError::new(StatusCode::BadRequest,
                format!("No canteen found with id '{}'", id)))
        }
    }
}
