use std::net::{Ipv6Addr, SocketAddrV6};

/// Checks if a string is a valid IPv6 address with a specified port.
#[inline]
pub fn is_valid(s: &str) -> bool {
    s.parse::<SocketAddrV6>().is_ok()
}

/// Converts a string to a SocketAddrV6 if it's a valid IPv6 address with a port.
#[inline]
pub fn convert(s: &str) -> Option<SocketAddrV6> {
    s.parse::<SocketAddrV6>().ok()
}

/// Creates a SocketAddrV6 from an IP address string, port, flow info, and scope ID.
#[inline]
pub fn create_socket(ip: &str, port: u16, flowinfo: u32, scope_id: u32) -> Option<SocketAddrV6> {
    let ip_addr = ip.parse::<Ipv6Addr>().ok()?;
    Some(SocketAddrV6::new(ip_addr, port, flowinfo, scope_id))
}
