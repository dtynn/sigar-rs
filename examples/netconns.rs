extern crate sigar_rs;

use sigar_rs::net;

fn main() {
    let conns = net::connection_list(net::FLAG_NETCONN_CLIENT | net::FLAG_NETCONN_TCP).unwrap();

    let size = if conns.len() > 10 { 10 } else { conns.len() };

    let mut i = 0usize;
    while i < size {
        println!("#{:?} {:?}", i + 1, conns[i]);
        i += 1;
    }
}
