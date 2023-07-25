use anyhow::Result;

use libp2p::core::multiaddr::{Multiaddr, Protocol};

use std::{net::IpAddr, str::FromStr};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

pub fn is_valid_ip(ip_addr: &str) -> bool {
    match IpAddr::from_str(ip_addr) {
        Ok(ip_addr) => {
            if ip_addr.is_ipv4() || ip_addr.is_ipv6() {
                return true;
            } else {
                return false;
            }
        }
        Err(_) => return false,
    }
}

// parse_multiaddrs
fn parse_multiaddrs(domain: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    // let mut real_dns = Vec::new();
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
        Err(_) => {}
    }

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;

    // Perform DNS TXT lookup
    let records = resolver.txt_lookup(domain)?;
    for record in records.iter() {
        for txt_data in record.iter() {
            let value: String = txt_data.iter().map(|c| *c as char ).collect();
            println!("{}", value);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::core::utils::ip::parse_multiaddrs;

    #[test]
    fn test_parse_multiaddrs() {
        // Test Multiaddr
        let domain = "/ip4/127.0.0.1/tcp/8080";
        let result = parse_multiaddrs(domain);
        println!("{:?}", result);

        // Test Domain name address
        let domain = "www.example.com";
        let result = parse_multiaddrs(domain);
        println!("{:?}", result);
    }
}
