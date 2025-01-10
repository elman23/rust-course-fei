//! You can use this file for experiments.

#[derive(Debug)]
struct Fibonacci {
    current: u32,
    next: u32,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let current = self.current;
        self.current = self.next;
        self.next += current;
        Some(self.next)
    }
}

fn main() {
    println!("Hello Rust!");

    let mut f = Fibonacci::default();

    println!("{:?}", f);

    println!("{}", f.next().unwrap());
    println!("{}", f.next().unwrap());
    println!("{}", f.next().unwrap());
    println!("{}", f.next().unwrap());
    println!("{}", f.next().unwrap());
}
