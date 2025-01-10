//! Run this file with `cargo test --test 03_interleave`.

// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.
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

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::interleave;

    #[test]
    fn interleave_empty() {
        assert_eq!(interleave("", ""), "");
    }

    #[test]
    fn interleave_only_left() {
        assert_eq!(interleave("a", ""), "a");
        assert_eq!(interleave("zxjas", ""), "zxjas");
    }

    #[test]
    fn interleave_only_right() {
        assert_eq!(interleave("", "z"), "z");
        assert_eq!(interleave("", "foobar"), "foobar");
    }

    #[test]
    fn interleave_same_length() {
        assert_eq!(interleave("abcdef", "012345"), "a0b1c2d3e4f5");
    }

    #[test]
    fn interleave_first_longer() {
        assert_eq!(
            interleave("Programming Rust", "O'Reilly"),
            "POr'oRgerialmlmying Rust"
        );
    }

    #[test]
    fn interleave_second_longer() {
        assert_eq!(
            interleave("ahoj, jak se máš?", "díky za optání, mám se dobře"),
            "adhíokjy,  zjaa ko psteá nmíá,š ?mám se dobře"
        );
    }

    #[test]
    fn interleave_second_longer_b() {
        let interleaved = interleave("máš?", "í, mám");
        assert_eq!(interleaved, "míá,š ?mám");
    }
}
