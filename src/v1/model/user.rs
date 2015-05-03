use v1::ApiError;
use v1::db::PooledDBConn;
use hyper::status::StatusCode;
use std::io::Read;
use std::error::Error;
use postgres::{self, Type, FromSql, ToSql};
use time::{self, Duration};

lazy_static! {
    static ref AUTH_TOKEN_TTL : Duration = Duration::seconds(10);
}

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

#[derive(RustcDecodable, RustcEncodable)]
pub struct LoginResponse {
    auth_token: String,
    valid_until: i64,
    me: User,
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

    pub fn login(db: PooledDBConn, username: String, auth_key: String)
        -> Result<LoginResponse, ApiError> {
        // Prepare query: We want to know if A: the user exists and B: the
        // auth_key is correct. We select the user with the given username
        // first (if there is no user with that username, we get an empty
        // result). We then try to left join the auth_key (if the auth_key
        // does not match, those columns will be null).
        let stmt = db.prepare(r#"
            SELECT users.id, auth_keys.device_desc
            FROM users
            LEFT JOIN auth_keys ON
                auth_keys.owner=users.id AND auth_keys.auth_string=$1
            WHERE users.username=$2"#).unwrap();

        // Execute query and check for any error.
        let rows = match stmt.query(&[&auth_key, &username]) {
            Err(e) => return Err(ApiError::detailed(
                StatusCode::InternalServerError,
                "Query failed!".to_string(),
                e.description().to_string())),
            Ok(rows) => rows
        };

        // Try to fetch the resulting dataset. If the result is empty the
        // username is invalid.
        let (user, valid) = match rows.iter().next() {
            Some(row) => ( User {
                            id: row.get(0),
                            username: username,
                            relation: None,
                        }, row.get_opt::<usize, String>(1).is_ok()),
            None => return Err(ApiError::new(StatusCode::NotFound,
                format!("No user found with username '{}'", username)))
        };

        if valid {
            // Create auth token and insert it into database.

            let now = time::now();
            let valid_until = (now + *AUTH_TOKEN_TTL).to_timespec();
            let token = "Cake".to_string();

            // let sql = r#"
            //     INSERT INTO auth_tokens(user_id, token, valid_until, created)
            //     VALUES ($1, $2, $3, $4);"#;
            // let args : &[&ToSql] = &[&user.id, &token, &valid_until, &now];
            // match db.execute(sql, args) {
            //     Err(e) => return Err(ApiError::detailed(
            //         StatusCode::InternalServerError,
            //         "Query failed!".to_string(),
            //         e.description().to_string())),
            //     Ok(affected) if (affected != 1) => return Err(ApiError::new(
            //         StatusCode::InternalServerError, format!(
            //             "Query succeded, but affected {} (instead of 1) rows!",
            //             affected)
            //         )),
            //     _ => {},
            // };

            Ok(LoginResponse {
                auth_token: token,
                valid_until: valid_until.sec,
                me: user,
            })
        } else {
            Err(ApiError::new(StatusCode::Forbidden,
                format!("Invalid auth_key!")))
        }
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
        // We only accept our custom type "user_relation".
        match *ty {
            Type::Other(ref o) if o.name() == "user_relation" => true,
            _ => false,
        }
    }
}
