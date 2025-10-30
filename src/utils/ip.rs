//! # IP and Networking Utilities
//!
//! Provides helper functions for validating IP addresses, resolving `dnsaddr` TXT records,
//! and parsing libp2p-style multiaddresses.
//!
//! This module enables the SDK to dynamically resolve network peers and endpoints
//! from domain names and DNS-based discovery mechanisms used in decentralized networks.
//!
//! ## Features
//! - Validate IPv4/IPv6 addresses
//! - Parse and verify libp2p `Multiaddr` strings
//! - Perform recursive DNS TXT lookups for `_dnsaddr` records
//! 

use crate::core::Error;
use libp2p::core::multiaddr::{Multiaddr, Protocol};
use std::{net::IpAddr, str::FromStr};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

/// Checks whether a string represents a valid IPv4 or IPv6 address.
///
/// # Arguments
/// * `ip_addr` - The input string to validate.
///
/// # Returns
/// `true` if the string is a valid IPv4 or IPv6 address, otherwise `false`.
///
/// # Example
/// ```
/// assert!(is_valid_ip("192.168.1.1"));
/// assert!(is_valid_ip("::1"));
/// assert!(!is_valid_ip("invalid_ip"));
/// ```
pub fn is_valid_ip(ip_addr: &str) -> bool {
    match IpAddr::from_str(ip_addr) {
        Ok(ip_addr) => ip_addr.is_ipv4() || ip_addr.is_ipv6(),
        Err(_) => false,
    }
}

/// Parses a domain or `Multiaddr` string into a list of valid libp2p multiaddresses.
///
/// Supports both direct IP-based multiaddresses (e.g., `/ip4/.../tcp/...`)
/// and DNS-based addresses resolved via TXT lookups on `_dnsaddr` records.
///
/// # Arguments
/// * `domain` - The domain or multiaddress string to parse.
///
/// # Returns
/// A vector of resolved multiaddress strings.
///
/// # Errors
/// Returns an [`Error`] if the input is not a valid multiaddress
/// or DNS resolution fails.
///
/// # Behavior
/// - If the input is already a valid `/ip4/.../tcp/...` multiaddress, it’s returned directly.
/// - Otherwise, a DNS TXT lookup is performed to resolve `dnsaddr=` entries.
/// - Nested `_dnsaddr.` lookups are also performed for chained DNS records.
///
/// # Example
/// ```ignore
/// let addrs = parse_multiaddrs("example.com")?;
/// for addr in addrs {
///     println!("Resolved multiaddr: {}", addr);
/// }
/// ```
pub fn parse_multiaddrs(domain: &str) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    let mut real_dns = Vec::new();
    match Multiaddr::from_str(domain) {
        Ok(addr) => {
            let mut protocols = addr.iter();

            if let Some(Protocol::Ip4(_)) = protocols.next() {
                if let Some(Protocol::Tcp(_)) = protocols.next() {
                    result.push(domain.to_string());
                    return Ok(result);
                }
            }
        }
        Err(_) => {
            return Err(format!("Failed to parse '{}' as a valid Multiaddr", domain).into());
        }
    }

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;

    // Perform DNS TXT lookup
    let records = resolver.txt_lookup(domain)?;
    for record in records.iter() {
        for dnsnames in record.iter() {
            let v: String = dnsnames.iter().map(|c| *c as char).collect();
            if v.contains("ip4") && v.contains("tcp") && v.matches('=').count() == 1 {
                result.push(v.trim_start_matches("dnsaddr=").to_string());
            }
        }
    }

    for v in result.iter() {
        let dnses = match resolver.txt_lookup(format!("_dnsaddr.{}", v)) {
            Ok(dnses) => dnses,
            Err(_) => continue,
        };
        for dns in dnses.iter() {
            for dnames in dns.iter() {
                let dns_str: String = dnames.iter().map(|c| *c as char).collect();
                if dns_str.contains("ip4")
                    && dns_str.contains("tcp")
                    && dns_str.matches('=').count() == 1
                {
                    let multiaddr = dns_str.trim_start_matches("dnsaddr=").to_string();
                    real_dns.push(multiaddr);
                }
            }
        }
    }

    result.extend(real_dns);
    Ok(result)
}

/// Parses a raw DNS TXT record containing a `dnsaddr=` entry into a [`Multiaddr`].
///
/// # Arguments
/// * `txt` - Byte slice representing the DNS TXT record contents.
///
/// # Returns
/// A valid [`Multiaddr`] if parsing succeeds.
///
/// # Errors
/// Returns an [`Error`] if the record does not begin with `dnsaddr=`
/// or contains invalid multiaddress syntax.
///
/// # Example
/// ```
/// let txt_record = b"dnsaddr=/ip4/127.0.0.1/tcp/4001";
/// let addr = parse_dnsaddr_txt(txt_record)?;
/// assert_eq!(addr.to_string(), "/ip4/127.0.0.1/tcp/4001");
/// ```
pub fn parse_dnsaddr_txt(txt: &[u8]) -> Result<Multiaddr, Error> {
    let s = std::str::from_utf8(txt).map_err(|_| "Error")?;
    match s.strip_prefix("dnsaddr=") {
        None => Err("Missing `dnsaddr=` prefix.".into()),
        Some(a) => Ok(Multiaddr::try_from(a).map_err(|_| "Error")?),
    }
}
