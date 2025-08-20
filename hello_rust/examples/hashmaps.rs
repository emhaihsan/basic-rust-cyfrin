use std::collections::HashMap;

fn main() {
    // 1) Inisialisasi kosong (mutable)
    let mut scores: HashMap<String, u32> = HashMap::new();

    // 2) Insert key-value
    scores.insert("red".to_string(), 100);
    scores.insert("blue".to_string(), 200);

    // 3) Cetak (debug / pretty)
    println!("scores (debug): {:?}", scores);
    println!("scores (pretty): {:#?}", scores);

    // 4) Get nilai (Option<&V>)
    let red = scores.get("red");
    println!("Red score: {:?}", red); // Some(100)
    let green = scores.get("green");
    println!("Green score: {:?}", green); // None

    // 5) entry().or_insert() → &mut V
    // Kasus A: key baru → disisipkan default 0, lalu ditambah 100
    let s_black: &mut u32 = scores.entry("black".to_string()).or_insert(0);
    *s_black += 100;
    println!("Black score now: {:?}", scores.get("black")); // Some(100)

    // Kasus B: key sudah ada → tidak menimpa, langsung dapat &mut nilai lama
    let s_blue: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *s_blue += 100; // 200 → 300
    println!("Blue score now: {:?}", scores.get("blue")); // Some(300)
}
