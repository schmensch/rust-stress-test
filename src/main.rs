use std::thread;

fn main() {
    let num_threads = num_cpus::get();
    println!("Threads in this system: {}", num_threads);
    println!("Using {} threads for stress test", num_threads);
    for i in 0..num_threads {
        println!("Spawning thread number {}", i);
        worker();
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
