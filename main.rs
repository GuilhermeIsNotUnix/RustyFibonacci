use std::time::Duration;
use std::thread;

fn main(){
    let root = 5_f32.sqrt();
    let phi = (1.0 + root) / 2.0;
    let mut n = 0;

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{}", (phi.powi(n) / root).round());
        n = n + 1;
    }
}
