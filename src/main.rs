use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use rand::random;

fn squared(x: f64, y: f64) -> f64 {
    (x * x) + (y * y)
}

fn main() {
    let start = Instant::now();
    let (tx, rx) = mpsc::channel();
    for _ in 0..=4    {
        let tx = tx.clone();
        let radius = 1.0;
        let handle = thread::spawn(move || {
            let mut in_count = 0.0;
            let mut total_count = 0.0;
            for num in 0..=1000000000u64 {
                match num % 100000000 {
                    0 => {
                        println!("{} iterations completed", num);
                    },
                    _ => {}
                }
                let point_x = random::<f64>() * 2.0 - 1.0;
                let point_y = random::<f64>() * 2.0 - 1.0;
                //x^2 + y^2 <= r^2
                if squared(point_x, point_y) <= radius {
                    in_count += 1.0;
                }
                total_count += 1.0;
            }
            tx.send((in_count, total_count)).unwrap();
        });

    }
    drop(tx);
    let mut in_count = 0.0;
    let mut total_count = 0.0;
    for (in_count_thread, total_count_thread) in rx {
        in_count += in_count_thread;
        total_count += total_count_thread;
    }
    let pi = 4.0 * (in_count / total_count);
    println!("Pi is approximately: {}", pi);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}


