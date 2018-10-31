extern crate sigar_rs;

use sigar_rs::cpu;

fn main() {
    #[cfg(target_arch = "x86")]
    println!("on x86");
    #[cfg(target_arch = "x86_64")]
    println!("on x86_64");

    let cpulsit = cpu::list().unwrap();

    println!("{:?}", cpulsit);
}
