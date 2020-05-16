use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type StatePool = actix_web::web::Data<Pool>;

pub fn create_pool() -> Pool {
    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL expected in .env");
    let manager = ConnectionManager::<PgConnection>::new(connspec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Fatal Error: Failed to create pool")
}
