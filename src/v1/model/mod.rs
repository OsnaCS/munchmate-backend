use postgres::{Type, FromSql, Result, Error};
use std::io::Read;
pub use self::canteen::Canteen;
pub use self::user::User;


pub mod canteen;
pub mod user;



#[derive(RustcDecodable, RustcEncodable)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

impl FromSql for Location {
    fn from_sql<R: Read>(_: &Type, raw: &mut R) -> Result<Self> {
        // A point is just a pair of two f64. Fortunately we can pass just `raw`
        // in both function calls, because the first call will read the first 8
        // bytes "away".
        // FIXME: I guess try! will return the incorrect error
        // `(WrongType(Float8))` instead of `WrongType(Point)`.
        Ok(Location {
            lat: try!(f64::from_sql(&Type::Float8, raw)),
            lng: try!(f64::from_sql(&Type::Float8, raw)),
        })
    }

    fn accepts(ty: &Type) -> bool {
        // We only accept the type Point.
        match *ty {
            Type::Point => true,
            _ => false,
        }
    }
}
