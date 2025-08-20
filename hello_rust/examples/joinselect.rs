use std::time::{Duration, Instant};
use tokio::{join, select};

async fn make(val: &'static str, dt: u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(dt)).await;
    val
}

#[tokio::main]
async fn main() {
    // 1) join!: tunggu semua selesai
    println!("Starting join! example...");
    let t0 = Instant::now();

    let (res1, res2, res3) = join!(
        make("coffee", 100),
        make("green tea", 50),
        make("lemonade", 20)
    );

    println!("join! completed in: {:?}", t0.elapsed());
    println!("join: res1 = {:?}", res1);
    println!("join: res2 = {:?}", res2);
    println!("join: res3 = {:?}", res3);

    // 2) select!: ambil yang pertama selesai, batalkan sisanya
    println!("\nStarting select! example...");
    let t1 = Instant::now();

    let res = select! {
        val = make("coffee", 100) => {
            println!("select!: 'coffee' (100ms) finished first");
            val
        },
        val = make("green tea", 50) => {
            println!("select!: 'green tea' (50ms) finished first");
            val
        },
        val = make("lemonade", 20) => {
            println!("select!: 'lemonade' (20ms) finished first");
            val
        },
    };

    println!("select! completed in: {:?}", t1.elapsed());
    println!("select: res = {:?}", res);

    // 3) select! dengan waktu sama (tie-breaking bisa berbeda tiap run)
    println!("\nStarting select! with equal times...");
    let t2 = Instant::now();

    let res_equal = select! {
        val = make("coffee", 20) => {
            println!("select! (equal): 'coffee' finished first");
            val
        },
        val = make("green tea", 20) => {
            println!("select! (equal): 'green tea' finished first");
            val
        },
        val = make("lemonade", 20) => {
            println!("select! (equal): 'lemonade' finished first");
            val
        },
    };

    println!("select! (equal) completed in: {:?}", t2.elapsed());
    println!("select! (equal): res = {:?}", res_equal);

    // 4) Pola umum: timeout dengan select! (race pekerjaan vs timer)
    println!("\nStarting select! with timeout...");
    let t3 = Instant::now();
    let timeout = tokio::time::sleep(Duration::from_millis(30));

    let res_timeout = select! {
        val = make("slow drink", 100) => {
            format!("ok: {val}")
        },
        _ = timeout => {
            "timeout!".to_string()
        },
    };

    println!("select! (timeout) finished in: {:?}", t3.elapsed());
    println!("select! (timeout): res = {:?}", res_timeout);
}
