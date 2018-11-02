extern crate sigar_rs;

use sigar_rs::{net, strip_bytes};
use std::str;

fn main() {
    let interface = net::interface_config_primary().unwrap();
    println!("primary interface:");
    println!(
        "\tname: {:?}",
        str::from_utf8(strip_bytes(&interface.name[..])).unwrap()
    );

    println!(
        "\ttype_: {:?}",
        str::from_utf8(strip_bytes(&interface.type_[..])).unwrap()
    );

    println!(
        "\tdescription: {:?}",
        str::from_utf8(strip_bytes(&interface.description[..])).unwrap()
    );

    println!("\thwaddr: {:?}", interface.hwaddr);

    println!("\taddress: {:?}", interface.address);

    println!("\tdestination: {:?}", interface.destination);

    println!("\tbroadcast: {:?}", interface.broadcast);

    println!("\tnetmask: {:?}", interface.netmask);

    println!("\taddress6: {:?}", interface.address6);

    println!("\tflags: {:?}", interface.flags);

    println!("\tmtu: {:?}", interface.mtu);
}
