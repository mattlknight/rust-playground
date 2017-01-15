extern crate mylib;

use mylib::traits::SqlSafe;

fn main() {
    // let username = "Robert'); DROP TABLE Students;--?";
    let username = "Bobby Tables";
    match username.is_sql_safe() {
        Ok(name) => println!("Username \"{}\" is safe", name),
        Err(err) => {
            println!("Error: Username \"{}\" is NOT SQL safe!", username);
            println!("Error: {}", err);
            println!("Error: {}", err.description());
        },
    };
}
