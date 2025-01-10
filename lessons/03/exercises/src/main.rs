//! You can use this file for experiments.

fn main() {
    println!("Hello Rust!");

    let s1 = "ahoj, jak se máš?";
    let s2 = "díky za optání, mám se dobře";

    char_idx(s1, s2);

    println!("Chars in s1: {}", s1.chars().count());
    println!("Len of s1: {}", s1.len());
    println!("Chars in s2: {}", s2.chars().count());
    println!("Len of s2: {}", s2.len());

    let s = interleave(s1, s2);
    println!("{s}");

    let l = "máš".len();
    println!("{l}");
}

fn char_idx(s1: &str, s2: &str) {
    for (i, c) in s1.char_indices() {
        println!("{i}, {c}");
    }
    for (i, c) in s2.char_indices() {
        println!("{i}, {c}");
    }
}

fn interleave(s1: &str, s2: &str) -> String {
    let mut interleaved = String::new();
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        //println!("{:?}", x);
        interleaved.push(c1);
        interleaved.push(c2);
    }
    if s1.chars().count() > s2.chars().count() {
        let s1_char_vec: Vec<char> = s1.chars().collect();
        let s1_final_part: String = s1_char_vec[s2.chars().count()..].iter().collect();
        interleaved.push_str(&s1_final_part);
    }
    if s2.chars().count() > s1.chars().count() {
        let s2_char_vec: Vec<char> = s2.chars().collect();
        let s2_final_part: String = s2_char_vec[s1.chars().count()..].iter().collect();
        interleaved.push_str(&s2_final_part);
    }
    interleaved
}
