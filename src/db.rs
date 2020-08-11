use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use rocket::{http::Status, request::FromRequest, Outcome, State};
use std::ops::Deref;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> PgPool {
    let database_url = dotenv!("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("failer to create db pool")
}

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();
    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
