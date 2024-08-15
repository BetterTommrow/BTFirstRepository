use mysql::prelude::*;
use mysql::*;

fn main() {
    let dsn: &str = "";
    let pool: Pool = Pool::new(dsn).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let ret: Option<String> = conn
        .query_first("select f1 from log_pull;")
        .unwrap();
    println!("{}", ret.unwrap());

    println!("{}", gcd(14, 100))
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

