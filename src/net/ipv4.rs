use std::net::{Ipv4Addr, SocketAddrV4};

/// Checks if a string is a valid IPv4 address with a port.
#[inline]
pub fn is_valid(s: &str) -> bool {
    s.parse::<SocketAddrV4>().is_ok()
}

/// Converts a string to a SocketAddrV4 if it's a valid IPv4 address with a port.
#[inline]
pub fn convert(s: &str) -> Option<SocketAddrV4> {
    s.parse::<SocketAddrV4>().ok()
}

/// Creates a SocketAddrV4 from an IP address string and a port.
#[inline]
pub fn create_socket(ip: &str, port: u16) -> Option<SocketAddrV4> {
    let ip_addr = ip.parse::<Ipv4Addr>().ok()?;
    Some(SocketAddrV4::new(ip_addr, port))
}
