use anyhow::{bail, Context, Result};

use libp2p::core::multiaddr::{Multiaddr, Protocol};

use std::{net::IpAddr, str::FromStr};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

pub fn is_valid_ip(ip_addr: &str) -> bool {
    match IpAddr::from_str(ip_addr) {
        Ok(ip_addr) => {
            ip_addr.is_ipv4() || ip_addr.is_ipv6()
        }
        Err(_) => false,
    }
}

// parse_multiaddrs
pub fn parse_multiaddrs(domain: &str) -> Result<Vec<String>> {
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
        Err(_) => {}
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

    for (_, v) in result.iter().enumerate() {
        let dnses = match resolver.txt_lookup(&format!("_dnsaddr.{}", v)) {
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

/// Parses a `<character-string>` of a `dnsaddr` TXT record.
pub fn parse_dnsaddr_txt(txt: &[u8]) -> Result<Multiaddr> {
    let s = std::str::from_utf8(txt).with_context(|| "Error")?;
    match s.strip_prefix("dnsaddr=") {
        None => bail!("Missing `dnsaddr=` prefix."),
        Some(a) => Ok(Multiaddr::try_from(a).with_context(|| "Error")?),
    }
}
