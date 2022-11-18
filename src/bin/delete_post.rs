use std::env;
use std::env::args;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    use tunnel_manager::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_max_conn_str = env::var("DB_MAX_CONNECTION").unwrap_or_else(|_| "5".to_string());
    let db_max_conn = db_max_conn_str.parse::<u32>().unwrap();

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    // Create a connection pool
    let pool = Pool::builder()
        .test_on_check_out(true)
        .max_size(db_max_conn)
        .build(manager)
        .expect("Could not build connection pool");

    let conn = &mut pool.get().unwrap();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
