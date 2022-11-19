use std::env;
use std::env::args;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;

use tunnel_manager::storage::users::Post;

fn main() {
    dotenv().ok();
    use tunnel_manager::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

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

    match diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
    {
        Ok(post) => println!("Published post {}", post.title),
        Err(err) => println!("Error: {}", err)
    }
}
