use postgres;
use openssl::ssl::{SslContext, SslMethod};
use rustless;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;

// Some type aliases for nicer names
pub type PooledDB = r2d2::Pool<PostgresConnectionManager>;
pub type PooledDBConn<'a> =
    r2d2::PooledConnection<'a, PostgresConnectionManager>;


// Will try to connect to the database specified by url
pub fn setup(url: &str, pool_size: u32) -> Option<PooledDB> {
    // Try to initialize SSL context
    let ssl_mode = match SslContext::new(SslMethod::Sslv23) {
        Ok(val) => postgres::SslMode::Prefer(val),
        Err(e) => {
            println!("ERROR: Creating SSL-context failed!");
            println!("    => {}", e);
            return None;
        }
    };

    // Create manager
    let manager = match PostgresConnectionManager::new(url, ssl_mode) {
        Ok(m) => m,
        Err(e) => {
            println!("ERROR: Failed to initialize connection manager!");
            println!("    => {}", e);
            return None;
        }
    };

    // Configure, create and return pool
    let config = r2d2::Config::builder().pool_size(pool_size).build();
    let handler = Box::new(r2d2::NoopErrorHandler);

    match r2d2::Pool::new(config, manager, handler) {
        Ok(pool) => Some(pool),
        Err(e) => {
            println!("ERROR: Connection to database failed!");
            println!("    => {}", e);
            return None;
        }
    }
}

// Info: The following code is essentially from the rustless example.
// To have easy access to the postgres connection in our endpoints, we want to
// store the connection in the typemap `rustless::Application::ext`. In order
// to use the typemap, we have to setup a key-value pair where the value is our
// postgres connection type. We use the following empty struct as key type.
pub struct AppDB;

impl ::typemap::Key for AppDB {
    type Value = PooledDB;
}

// To make access even easier, we can add a method to rustless::Application.
// Endpoints just have to call `.db()` instead of `.get<AppDB>()`.
pub trait DatabaseExt: rustless::Extensible {
    fn db(&self) -> PooledDBConn;
}
impl DatabaseExt for rustless::Application {
    fn db(&self) -> PooledDBConn {
        // FIXME: Handle get() error
        self.ext.get::<AppDB>().unwrap().get().unwrap()
    }
}
