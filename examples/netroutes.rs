extern crate sigar_rs;

use sigar_rs::{net, strip_bytes};
use std::str;

fn main() {
    let routes = net::route_list().unwrap();
    for (i, route) in routes.iter().enumerate() {
        println!("route {}", i + 1);
        println!("\tdestination: {:?}", route.destination);
        println!("\tgateway: {:?}", route.gateway);
        println!("\tmask: {:?}", route.mask);
        println!(
            "\tifname: {:?}",
            str::from_utf8(strip_bytes(&route.ifname[..])).unwrap()
        );
        println!("");
    }
}
