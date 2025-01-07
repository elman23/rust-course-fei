//! You can use this file for experiments.

fn main() {
    println!("Hello Rust!");
    if 'a' < 'A' {
        println!("'A' is bigger.");
    } else {
        println!("'A' is smaller.");
    }
    let l = 'B'.to_ascii_lowercase();
    println!("Lowercase: {l}.");
    if l < 'A' {
        println!("'A' is bigger.");
    } else {
        println!("'A' is smaller.");
    }
}
