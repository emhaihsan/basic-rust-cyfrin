fn print_message(s: &str) {
    println!("{}", s);
}

fn greet(name: &str) -> String {
    format!("Halo {}", name)
}

fn lens(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}

fn first_word(s: &str) -> &str {
    let idx = s.find(' ');

    &s[0..idx.unwrap_or(s.len())]
}

fn main() {
    // Membuat string
    let s1: String = String::from("Hello Rust");
    let s2: String = "Hello World".to_string();

    // Panjang dalam byte (UTF-8)
    println!("len(s1) bytes = {}", s1.len());
    let crab = "🦀"; // 1 karakter, tapi 4 byte
    println!(
        "'🦀' bytes = {}, chars = {}",
        crab.len(),
        crab.chars().count()
    );

    // &str dari literal dan dari String
    let lit: &str = "literal";
    let view_all: &str = &s1; // &String -> &str (deref coercion)
    let hello: &str = &s1[0..5]; // "Hello" (hati-hati: harus pada batas byte UTF-8)
    println!("lit={}, view_all={}, hello={}", lit, view_all, hello);

    // Konversi &str -> String
    let owned1: String = lit.to_string();
    let owned2: String = String::from(lit);
    println!("owned1={}, owned2={}", owned1, owned2);

    // Aman saat slicing: gunakan get(..) untuk menghindari panic pada batas UTF-8
    if let Some(sub) = s1.get(0..5) {
        println!("safe slice: {}", sub);
    } else {
        println!("invalid utf-8 boundary");
    }

    // Modifikasi String
    let mut msg = String::from("Hello");
    msg.push(' ');
    msg.push_str("Rust");
    println!("msg = {}", msg);

    // Operator + (memindahkan kiri)
    let a = String::from("Hi ");
    let b = String::from("Rust");
    let c = a + &b; // a pindah (move), b dipinjam sebagai &str
    println!("c = {}", c);

    // format! membangun String baru (tidak memindahkan argumen)
    let name = "Rust";
    let ver = 1.76f32;
    let s = format!("Learning {} version {} is fun! 🦀", name, ver);
    println!("{}", s);

    // Parameter &str fleksibel
    print_message(&s2); // &String -> &str
    print_message("from literal"); // sudah &str

    let lens = lens(&s1);
    println!("len(s1) bytes = {}, chars = {}", lens.0, lens.1);
    let first = first_word(&s1);
    println!("first word: {}", first);
    let greet = greet("Rust");
    println!("greet = {}", greet);
}
