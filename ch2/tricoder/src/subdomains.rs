use crate::{
    model::{CrtShEntry, Subdomain},
    Error,
};
use reqwest::blocking::Client;
use std::{collections::HashSet, time::Duration};
/*
The Resolver is responsible for performing recursive queries to lookup domain names.

This is a 100% in process DNS resolver. It does not use the Host OS’ resolver. If what is desired is to use the Host OS’ resolver, generally in the system’s libc, then the std::net::ToSocketAddrs variant over &str should be used.

This uses the default configuration, which sets the Google Public DNS as the upstream resolvers. Please see their privacy statement for important information about what they track, many ISP’s track similar information in DNS.

https://docs.rs/trust-dns-resolver/latest/trust_dns_resolver/
 */
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};

pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;

    //clean and dedup results
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains('*'))
        .collect();
    subdomains.insert(target.to_string());

    let subdomains: Vec<Subdomain> = subdomains
        .into_iter()
        .map(|domain| Subdomain {
            domain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect();
    Ok(subdomains)
}

pub fn resolves(domain: &Subdomain) -> bool {
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(4);

    let dns_resolver = Resolver::new(ResolverConfig::default(), opts)
        .expect("subdomain resolver: building DNS client");
    dns_resolver.lookup_ip(domain.domain.as_str()).is_ok()
}