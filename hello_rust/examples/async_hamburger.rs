use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn grill_patty() -> &'static str {
    println!("Grilling patty...");
    sleep(Duration::from_millis(800)).await;
    "patty"
}

async fn toast_bun() -> &'static str {
    println!("Toasting bun...");
    sleep(Duration::from_millis(400)).await;
    "bun"
}

async fn slice_veggies() -> &'static str {
    println!("Slicing veggies...");
    sleep(Duration::from_millis(500)).await;
    "veggies"
}

fn assemble(parts: [&str; 3]) -> String {
    format!("{} with {} and {}", parts[0], parts[1], parts[2])
}

#[tokio::main]
async fn main() {
    println!("=== Async Hamburger Demo ===");

    // Sequential (await one after another)
    let t0 = Instant::now();
    let p = grill_patty().await;
    let b = toast_bun().await;
    let v = slice_veggies().await;
    let burger_seq = assemble([b, p, v]);
    println!("Sequential: {burger_seq} in {:?}\n", t0.elapsed());

    // Concurrent with join!: start all futures, await together
    let t1 = Instant::now();
    let (p2, b2, v2) = tokio::join!(grill_patty(), toast_bun(), slice_veggies());
    let burger_conc = assemble([b2, p2, v2]);
    println!("Concurrent (join!): {burger_conc} in {:?}\n", t1.elapsed());

    // Concurrent with spawned tasks
    let t2 = Instant::now();
    let h_patty = tokio::spawn(grill_patty());
    let h_bun = tokio::spawn(toast_bun());
    let h_veg = tokio::spawn(slice_veggies());

    let p3 = h_patty.await.expect("task panicked");
    let b3 = h_bun.await.expect("task panicked");
    let v3 = h_veg.await.expect("task panicked");
    let burger_spawn = assemble([b3, p3, v3]);
    println!("Concurrent (spawn): {burger_spawn} in {:?}\n", t2.elapsed());

    println!("Tip: Futures are lazy — they do nothing until awaited or polled by the runtime.");
}
