use std::io;
use std::path::Path;
use std::string::String;
fn main() {
    println!("Server started!");
    let db_exists: bool = check_db_exists().unwrap_or(false);
    let mut db_path: String = String::new();
    let mut db_name: String = String::new();
    let mut db_user: String = String::new();
    let mut db_password: String = String::new();
    let mut db_port: u16 = 0;
    let mut db_host: String = String::new();
    let mut db_schema: String = String::new();
    let mut db_table: String = String::new();
    let mut db_column: String = String::new();
}
const DB_PATH: &str = "data/db";

fn check_db_exists() -> Result<bool, io::Error> {
    if Path::new(DB_PATH).exists() {
        return Ok(true);
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Database does not exist at {}", DB_PATH),
    ))
}
