use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;
use std::env;

// An alias to the type for a pool of Diesel Postgres Connection
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initialize the database pool.
pub fn connect() -> PgPool {
    let db_user= env::var("DB_USER").unwrap_or(String::from("postgres"));
    let db_pass= env::var("DB_PASS").unwrap_or(String::from("password"));
    let db_host= env::var("DB_HOST").unwrap_or(String::from("localhost"));
    let db_name= env::var("DB_NAME").unwrap_or(String::from("workouts"));
    let manager = ConnectionManager::<PgConnection>::new(
        format!(
            "postgres://{}:{}@{}/{}",
            db_user,
            db_pass,
            db_host,
            db_name,
        )
        .as_str(),
    );
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &Connection as an &PgConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
