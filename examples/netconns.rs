extern crate sigar_rs;

use sigar_rs::net;

fn main() {
    let conns = net::connection_list(0).unwrap();

    println!("{:?}", conns);
}
