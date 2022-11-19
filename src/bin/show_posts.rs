use std::env;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;

use tunnel_manager::storage::users::*;

fn main() {
    dotenv().ok();
    use tunnel_manager::schema::posts::dsl::*;

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

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
