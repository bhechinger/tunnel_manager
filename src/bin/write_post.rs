use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;
use std::io::{stdin, Read};
use tunnel_manager::storage::users::*;

fn main() {
    dotenv().ok();
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
    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(conn, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
