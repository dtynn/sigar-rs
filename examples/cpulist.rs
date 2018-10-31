extern crate sigar_rs;

use sigar_rs::cpu;
use std::{thread::sleep, time::Duration};

fn main() {
    #[cfg(target_arch = "x86")]
    println!("on x86");
    #[cfg(target_arch = "x86_64")]
    println!("on x86_64");

    let cpulist = cpu::list().unwrap();

    println!("cpu usage: \n{:?}", cpulist);

    let cpuinfos = cpu::info_list().unwrap();
    println!("cpu info: \n{:?}", cpuinfos);

    let prev = cpu::get().unwrap();

    sleep(Duration::from_secs(1));

    let curr = cpu::get().unwrap();

    let perc = cpu::calc_percent(&prev, &curr);

    println!("percent: \n{:?}", perc);
}
