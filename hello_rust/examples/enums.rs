#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 },
}

fn main() {
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("cmd0 == cmd1: {}", cmd0 == cmd1); // Output: cmd0 == cmd1: false
    let cmd_play1: Command = Command::Play;
    let cmd_play2: Command = Command::Play;
    println!("cmd_play1 == cmd_play2: {}", cmd_play1 == cmd_play2); // Output: cmd_play1 == cmd_play2: true
    let cmd_skip1: Command = Command::Skip(10);
    let cmd_skip2: Command = Command::Skip(10);
    println!("cmd_skip1 == cmd_skip2: {}", cmd_skip1 == cmd_skip2); // Output: cmd_skip1 == cmd_skip2: true
    let cmd_skip3: Command = Command::Skip(20);
    println!("cmd_skip1 == cmd_skip3: {}", cmd_skip1 == cmd_skip3); // Output: cmd_skip1 == cmd_skip3: false
}
