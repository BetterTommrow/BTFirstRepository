mod db_config;

use mysql::prelude::*;
use mysql::*;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use crate::db_config::*; // 导入结构体


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut file = File::open("db/config/db_connection.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: DbConfig = serde_json::from_str(&contents)?;

    let db_config = &config.毛古哥服务器;
    // println!("Host: {}", db_config.host);
    // println!("Port: {}", db_config.port);
    // println!("User: {}", db_config.user);
    // println!("Password: {}", db_config.password);

    let dsn= format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.user,
        db_config.password,
        db_config.host,
        db_config.port,
        db_config.database
    );
    let dsn_slice: &str = dsn.as_str();
    let pool: Pool = Pool::new(dsn_slice).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let ret: Option<String> = conn
        .query_first("select f1 from log_pull;")
        .unwrap();
    println!("{}", ret.unwrap());

    println!("{}", gcd(14, 100));

    Ok(())
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

