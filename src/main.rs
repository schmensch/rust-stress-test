use std::thread;

fn main() {
    let threads = num_cpus::get();
    println!("Threads in this system: {}", threads);
    println!("Using {} threads for stress test", threads);
    for i in 1..threads {
        println!("Spawning thread number {}", i);
        thread::spawn (|| {
            let mut x = 0;
            loop {
                x = x + 1;
                x = x - 1;
            }
        });
    }
    let mut x = 0;
    loop {
        x = x + 1;
        x = x - 1;
    }
}
