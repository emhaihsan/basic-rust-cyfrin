use std::thread;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const N: usize = 1000; // jumlah tugas per skenario
const WORK_MS: u64 = 100; // durasi kerja/sleep per tugas

fn blocking_work() {
    // Simulasi kerja I/O-blocking di thread biasa
    thread::sleep(Duration::from_millis(WORK_MS));
}

async fn async_work() {
    // Simulasi kerja I/O-bound non-blocking di dunia async
    sleep(Duration::from_millis(WORK_MS)).await;
}

#[tokio::main]
async fn main() {
    println!("=== Threads vs Async/Await (Tokio) ===");
    println!("Tugas per skenario: {N}, durasi per tugas: {WORK_MS}ms\n");

    // 1) Native threads (blocking)
    let t0 = Instant::now();
    let mut handles = Vec::with_capacity(N);
    for _ in 0..N {
        handles.push(thread::spawn(|| blocking_work()));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Threads (blocking): {:?}", t0.elapsed());

    // 2) Async good: non-blocking sleep di runtime Tokio
    let t1 = Instant::now();
    let mut tasks = Vec::with_capacity(N);
    for _ in 0..N {
        tasks.push(tokio::spawn(async_work()));
    }
    for t in tasks {
        t.await.unwrap();
    }
    println!("Async (non-blocking sleep): {:?}", t1.elapsed());

    // 3) Async bad: memanggil blocking_work di task async (menghambat worker threads)
    let t2 = Instant::now();
    let mut bad_tasks = Vec::with_capacity(N);
    for _ in 0..N {
        bad_tasks.push(tokio::spawn(async move {
            // Anti-pattern: memblokir thread runtime
            blocking_work();
        }));
    }
    for t in bad_tasks {
        t.await.unwrap();
    }
    println!(
        "Async (blocking di task) [anti-pattern]: {:?}",
        t2.elapsed()
    );

    // 4) Async + spawn_blocking: offload kerja blocking ke pool khusus
    let t3 = Instant::now();
    let mut offload_tasks = Vec::with_capacity(N);
    for _ in 0..N {
        offload_tasks.push(tokio::task::spawn_blocking(|| blocking_work()));
    }
    for t in offload_tasks {
        t.await.unwrap();
    }
    println!("Async (spawn_blocking - offload): {:?}", t3.elapsed());

    println!("\nCatatan:");
    println!("- Threads cocok untuk CPU-bound; jumlah thread sebaiknya dibatasi sesuai core.");
    println!("- Async cocok untuk I/O-bound; gunakan operasi non-blocking (sleep, I/O async).");
    println!("- Jika perlu operasi blocking di dunia async, gunakan spawn_blocking untuk offload.");
}
