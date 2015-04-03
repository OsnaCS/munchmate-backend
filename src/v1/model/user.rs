use v1::ApiError;
use v1::db::PooledDBConn;
use hyper::status::StatusCode;
use std::error::Error;


#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
    id: i64,
    username: String,
    // creation_time: ,
    // avatar_url: String,
    // last_action: ,
    // avatar_thumbnail
}


impl User {
    pub fn get_by_id(db: PooledDBConn, id: i64) -> Result<User, ApiError> {
        // Prepare and execute query and check for errors.
        let stmt = db.prepare(r#"
            SELECT users.id, users.username
            FROM users
            WHERE users.id=$1"#).unwrap();

        let rows = match stmt.query(&[&id]) {
            Err(e) => return Err(ApiError::detailed(
                StatusCode::InternalServerError,
                "Query failed!".to_string(),
                e.description().to_string())),
            Ok(rows) => rows
        };

        // Try to fetch the only user with that id or return 404.
        match rows.iter().next() {
            Some(row) => Ok(User {
                            id: row.get(0),
                            username: row.get(1),
                        }),
            None => Err(ApiError::new(StatusCode::NotFound,
                format!("No user found with id '{}'", id)))
        }
    }
}
