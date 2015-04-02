use api::ApiError;
use db::PooledDBConn;
use hyper::status::StatusCode;
use std::error::Error;
use model::Location;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Canteen {
    id: i32,
    name: String,
    city_id: i32,
    city_name: String,
    location: Location,
    distance: Option<f64>,
}

impl Canteen {
    pub fn get_by_id(db: PooledDBConn, id: i32) -> Result<Canteen, ApiError> {
        // Prepare and execute query and check for errors.
        let stmt = db.prepare(r#"
            SELECT canteens.id, canteens.name, city_id, cities.name, location
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

        // Try to fetch the only canteen with that id or return 404.
        match rows.iter().next() {
            Some(row) => Ok(Canteen {
                            id: row.get(0),
                            name: row.get(1),
                            city_id: row.get(2),
                            city_name: row.get(3),
                            location: row.get(4),
                            distance: None,
                        }),
            None => Err(ApiError::new(StatusCode::NotFound,
                format!("No canteen found with id '{}'", id)))
        }
    }

    pub fn get_nearest(db: PooledDBConn, pos: Location)
        -> Result<Vec<Canteen>, ApiError> {
        // Prepare query: Ordered by the distance to the given `pos`.
        let stmt = db.prepare(r#"
            SELECT canteens.id, canteens.name, city_id, cities.name, location,
                (point($1, $2) <@> location)*1.609344 as "distance"
            FROM canteens
            INNER JOIN cities ON cities.id=city_id
            ORDER BY distance
            LIMIT 5"#).unwrap();

        // Execute query and check for any error.
        let rows = match stmt.query(&[&pos.lat, &pos.lng]) {
            Err(e) => return Err(ApiError::detailed(
                StatusCode::InternalServerError,
                "Query failed!".to_string(),
                e.description().to_string())),
            Ok(rows) => rows
        };

        // Fetch all canteens and return.
        let mut data = Vec::new();

        for row in rows {
            data.push(Canteen {
                id: row.get(0),
                name: row.get(1),
                city_id: row.get(2),
                city_name: row.get(3),
                location: row.get(4),
                distance: row.get(5),
            });
        }

        Ok(data)
    }
}
