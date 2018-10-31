extern crate sigar_rs;

use sigar_rs::load;

fn main() {
    let loadavg = load::get().unwrap();

    println!(
        "loadavg: {:?}, {:?}, {:?}",
        loadavg.avg_1m, loadavg.avg_5m, loadavg.avg_15m
    );
}
