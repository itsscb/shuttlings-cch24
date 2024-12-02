#![allow(dead_code, clippy::unused_async)]

mod ipv4_dest;
mod ipv4_key;
mod ipv6_dest;
mod ipv6_key;

pub use ipv4_dest::ipv4_dest;
pub use ipv4_key::ipv4_key;
pub use ipv6_dest::ipv6_dest;
pub use ipv6_key::ipv6_key;

#[derive(serde::Deserialize)]
pub struct DestParams {
    from: String,
    key: String,
}

#[derive(serde::Deserialize)]
pub struct KeyParams {
    from: String,
    to: String,
}

#[cfg(test)]
mod test {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    use ipv4_dest::calculate_ipv4_dest;
    use ipv4_key::calculate_ipv4_key;
    use ipv6_dest::calculate_ipv6_dest;
    use ipv6_key::calculate_ipv6_key;

    use super::*;

    #[test]
    fn test_calculate_ipv4_dest() {
        let from = Ipv4Addr::from_str("10.0.0.0").unwrap();
        let key = Ipv4Addr::from_str("1.2.3.255").unwrap();
        assert_eq!(calculate_ipv4_dest(from, key), "11.2.3.255");

        let from = Ipv4Addr::from_str("128.128.33.0").unwrap();
        let key = Ipv4Addr::from_str("255.0.255.33").unwrap();
        assert_eq!(calculate_ipv4_dest(from, key), "127.128.32.33");
    }

    #[test]
    fn test_calculate_ipv4_key() {
        let from = Ipv4Addr::from_str("10.0.0.0").unwrap();
        let to = Ipv4Addr::from_str("11.2.3.255").unwrap();
        assert_eq!(calculate_ipv4_key(from, to), "1.2.3.255");

        let from = Ipv4Addr::from_str("128.128.33.0").unwrap();
        let to = Ipv4Addr::from_str("127.128.32.33").unwrap();
        assert_eq!(calculate_ipv4_key(from, to), "255.0.255.33");
    }

    #[test]
    fn test_calculate_ipv6_dest() {
        let from = Ipv6Addr::from_str("fe80::1").unwrap();
        let key = Ipv6Addr::from_str("5:6:7::3333").unwrap();
        assert_eq!(calculate_ipv6_dest(from, key), "fe85:6:7::3332");
    }

    #[test]
    fn test_calculate_ipv6_key() {
        let from = Ipv6Addr::from_str("aaaa::aaaa").unwrap();
        let to = Ipv6Addr::from_str("5555:ffff:c:0:0:c:1234:5555").unwrap();
        assert_eq!(calculate_ipv6_key(from, to), "ffff:ffff:c::c:1234:ffff");
    }
}
