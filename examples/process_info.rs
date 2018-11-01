extern crate sigar_rs;

use sigar_rs::process;

fn main() {
    let pid = std::process::id();
    println!("process information for {}", pid);

    println!("mem usage: {:?}", process::mem(pid).unwrap());

    #[cfg(target_os = "linux")]
    println!("disk io: {:?}", process::disk_io(pid).unwrap());

    #[cfg(target_os = "linux")]
    println!("cum disk io: {:?}", process::cum_disk_io(pid).unwrap());

    println!("cred: {:?}", process::cred(pid).unwrap());

    println!("cred name: {:?}", process::cred_name(pid).unwrap());
}
