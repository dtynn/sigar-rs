extern crate sigar_rs;

use sigar_rs::process;

fn main() {
    let pids = process::list().unwrap();

    println!("pids: {:?}", pids);

    let stat = process::stat().unwrap();
    println!("stats: {:?}", stat);

    if pids.len() > 0 {
        let memusage = process::mem(pids[0]).unwrap();
        println!("mem usage for {:?}: {:?}", pids[0], memusage);
    }
}
