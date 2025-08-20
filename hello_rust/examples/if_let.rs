#![allow(unused)]

fn main() {
    // 1) Option: match vs if let
    let x: Option<i32> = Some(9);

    match x {
        Some(val) => println!("(match) Option is {val}"),
        None => {}
    }

    if let Some(val) = x {
        println!("(if let) Option is {val}");
    }

    // 2) if let + else
    let y: Option<i32> = None;
    if let Some(v) = y {
        println!("Some: {v}");
    } else {
        println!("y is None");
    }
    // 3) Result
    let ok: Result<i32, String> = Ok(42);
    let err: Result<i32, String> = Err("fail".to_string());

    if let Ok(v) = ok {
        println!("ok value: {v}");
    }

    if let Err(e) = err {
        println!("err: {e}");
    }

    // 4) Enum custom + destrukturisasi
    enum Cmd {
        Move { x: i32, y: i32 },
        Quit,
    }
    let c = Cmd::Move { x: 3, y: 4 };
    if let Cmd::Move { x, y } = c {
        println!("move to ({x}, {y})");
    }
}
