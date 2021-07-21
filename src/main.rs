use std::thread;
use std::time::Duration;

fn main() {
    let num_threads = num_cpus::get();
    println!("Threads in this system: {}", num_threads + 1);
    println!("Using {} threads for stress test", num_threads + 1);
    for i in 0..num_threads {
        println!("Spawning thread number {}", i);
        worker();
    }
    loop {
        thread::sleep(Duration::from_millis(1))
    }
}

fn worker() {
    thread::spawn (|| {
        let mut _x = 0;
        loop {
            _x += 1;
            _x -= 1;
        }
    });
}
