extern crate sigar_rs;

use sigar_rs::mem;

fn main() {
    let meminfo = mem::get().unwrap();
    println!("mem: {:?}", meminfo);

    let swapinfo = mem::swap().unwrap();
    println!("swap: {:?}", swapinfo);
}
