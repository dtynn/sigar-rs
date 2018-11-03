extern crate sigar_rs;

use sigar_rs::{net, strip_bytes};
use std::str;

fn main() {
    let ifaces = net::interface_list().unwrap();
    println!("{:?} interfaces:", ifaces.len());
    for cs in ifaces {
        println!("\t{:?}", cs.into_string().unwrap());
    }

    let interface = net::interface_config_primary().unwrap();
    println!("primary interface:");

    let name = str::from_utf8(strip_bytes(&interface.name[..])).unwrap();

    println!("\tname: {:?}", name);

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

    println!("\tprefix6_length: {:?}", interface.prefix6_length);

    println!("\tscope6: {:?}", interface.scope6);

    println!("\tflags: {:?}", interface.flags);

    println!("\tmtu: {:?}", interface.mtu);

    let stat = net::interface_stat(name).unwrap();
    println!("interface stat for {:?}: {:?}", name, stat);
}
