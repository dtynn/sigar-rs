extern crate sigar_rs;

use sigar_rs::{net, strip_bytes};
use std::str;

fn main() {
    let netinfo = net::info().unwrap();
    println!("netinfo:");
    println!(
        "\tdefault_gateway: {:?}",
        str::from_utf8(strip_bytes(&netinfo.default_gateway[..])).unwrap()
    );

    println!(
        "\tdefault_gateway_interface: {:?}",
        str::from_utf8(strip_bytes(&netinfo.default_gateway_interface[..])).unwrap()
    );

    println!(
        "\thost_name: {:?}",
        str::from_utf8(strip_bytes(&netinfo.host_name[..])).unwrap()
    );

    println!(
        "\tdomain_name: {:?}",
        str::from_utf8(strip_bytes(&netinfo.domain_name[..])).unwrap()
    );

    println!(
        "\tprimary_dns: {:?}",
        str::from_utf8(strip_bytes(&netinfo.primary_dns[..])).unwrap()
    );

    println!(
        "\tsecondary_dns: {:?}",
        str::from_utf8(strip_bytes(&netinfo.secondary_dns[..])).unwrap()
    );
}
