use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    //use a custom threadpool to improve speed
    /*
        https://github.com/rayon-rs/rayon
        https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/
        Rayon is a data-parallelism library for Rust. It is extremely lightweight and makes it easy to convert a sequential computation into a parallel one. It also guarantees data-race freedom.

        use rayon::prelude::*;
        fn sum_of_squares(input: &[i32]) -> i32 {
            input.par_iter() // <-- just change that!
                .map(|&i| i * i)
                .sum()
    }
         */

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    // pool.install is required to use our custom threadpool, instead of rayon's default one

    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("     {}", port.port);
            }

            println!();
        }
    });

    Ok(())
}

/*
Output:

cargo run -- kerkour.com
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/tricoder kerkour.com`
social.kerkour.com:
     80
     443
     8080
     8443

academy.kerkour.com:
     80
     443
     8080
     8443

www.kerkour.com:
     80
     443
     8080
     8443

kerkour.com:
     80
     443
     8080
     8443

[16:59:11] [cost 40.970s] cargo run -- kerkour.com
 */
