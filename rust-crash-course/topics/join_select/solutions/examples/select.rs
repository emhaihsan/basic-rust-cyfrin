#![allow(unused)]

use std::time::Duration;
use tokio::{join, select};

// join and select macros

// join!
// - Polls multiple Futures concurrently
// - Waits for all of them to complete
// - Returns a tuple of their results

// select!
// - Polls multiple Futures concurrently
// - Returns as soon as one of them completes
// - Cancels the rest (drops them)

// Simulates an async task that completes after `dt` milliseconds
async fn make(val: &'static str, dt: u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(dt)).await;
    val
}

#[tokio::main]
async fn main() {
    let (res1, res2, res3) = join!(
        make("coffee", 10),
        make("green tea", 20),
        make("lemonade", 30)
    );
    println!("join: res1 = {:?}", res1);
    println!("join: res2 = {:?}", res2);
    println!("join: res3 = {:?}", res3);

    let res = select! {
        val = make("coffee", 1) => {
            println!("future 1 finished first");
            val
        }
        val = make("green tea", 1) => {
            println!("future 2 finished first");
            val
        }
        val = make("lemonade", 1) => {
            println!("future 3 finished first");
            val
        }
    };
    println!("select: res = {:?}", res);
}
