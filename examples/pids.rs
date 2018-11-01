extern crate sigar_rs;

use sigar_rs::process;

fn main() {
    let pids = process::list().unwrap();

    println!("pids: {:?}", pids);

    let stat = process::summary().unwrap();
    println!("stats: {:?}", stat);
}
