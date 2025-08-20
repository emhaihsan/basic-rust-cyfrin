#![allow(unused)]

#[derive(Debug)]
struct Solidity {
    version: String,
}

#[derive(Debug)]
struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;

    // default method
    fn help(&self) -> String {
        "No specific help available. Good luck!".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {}", file_path)
    }
}

// Polymorphic fn using a trait bound with impl Trait
fn compile_contract(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8.20".to_string(),
    };
    let vy = Vyper {
        version: "0.3.7".to_string(),
    };

    // Direct calls via trait methods
    println!("Direct - Solidity: {}", sol.compile("example.sol"));
    println!("Direct - Vyper:    {}", vy.compile("example.vy"));

    // Generic function using trait bound
    println!(
        "Generic - Solidity: {}",
        compile_contract(&sol, "example.sol")
    );
    println!(
        "Generic - Vyper:    {}",
        compile_contract(&vy, "example.vy")
    );

    // Default method from trait
    println!("Solidity help: {}", sol.help());
    println!("Vyper help:    {}", vy.help());
}
