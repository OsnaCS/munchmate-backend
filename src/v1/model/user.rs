use v1::ApiError;
use v1::db::PooledDBConn;
use hyper::status::StatusCode;
use std::io::Read;
use std::error::Error;
use postgres::{self, Type, FromSql};



#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
    id: i64,
    username: String,
    relation: Option<Relation>,
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
                            relation: None,
                        }),
            None => Err(ApiError::new(StatusCode::NotFound,
                format!("No user found with id '{}'", id)))
        }
    }

    pub fn contacts(db: PooledDBConn, user: i64)
        -> Result<Vec<User>, ApiError> {
        // Prepare query: Ordered by the distance to the given `pos`.
        let stmt = db.prepare(r#"
            SELECT users.id, users.username, type
            FROM usera_knows_userb
            INNER JOIN users ON users.id=userb
            WHERE usera=$1
            ORDER BY type ASC"#).unwrap();

        // Execute query and check for any error.
        let rows = match stmt.query(&[&user]) {
            Err(e) => return Err(ApiError::detailed(
                StatusCode::InternalServerError,
                "Query failed!".to_string(),
                e.description().to_string())),
            Ok(rows) => rows
        };

        // Fetch all users with some relationship to `user` and return.
        let mut data = Vec::new();

        for row in rows {
            data.push(User {
                id: row.get(0),
                username: row.get(1),
                relation: Some(row.get(2)),
            });
        }

        Ok(data)
    }
}



#[derive(RustcDecodable, RustcEncodable)]
enum Relation {
    Blocked,
    Contact,
    Fav,
}

impl FromSql for Relation {
    fn from_sql<R: Read>(ty: &Type, raw: &mut R) -> postgres::Result<Self> {
        // Postgres will send the enum value names as string... so we need to
        // read all data into a string and then match all possibilities.
        let mut s = String::new();
        if raw.read_to_string(&mut s).is_err() {
            return Err(postgres::Error::WrongType(ty.clone()));
        }

        match &*s {
            "fav" => Ok(Relation::Fav),
            "contact" => Ok(Relation::Contact),
            "blocked" => Ok(Relation::Blocked),
            _ => Err(postgres::Error::WrongType(ty.clone()))
        }
    }

    fn accepts(ty: &Type) -> bool {
        // We only accept the type Point.
        match *ty {
            Type::Other(ref o) if o.name() == "user_relation" => true,
            _ => false,
        }
    }
}
