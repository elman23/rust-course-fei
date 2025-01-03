//! Run this file with `cargo test --test 04_luhn_algorithm`.

// TODO: Implement the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm),
// which is used to check the validity of e.g. bank or credit card numbers.

/**
* function isValid(cardNumber[1..length])
   sum := 0
   parity := length mod 2
   for i from 1 to length do
       if i mod 2 != parity then
           sum := sum + cardNumber[i]
       elseif cardNumber[i] > 4 then
           sum := sum + 2 * cardNumber[i] - 9
       else
           sum := sum + 2 * cardNumber[i]
       end if
   end for
   return cardNumber[length] == ((10 - (sum mod 10)) mod 10)
   end function
*/

fn luhn_algorithm(n: i64) -> bool {
    let number_as_string = n.to_string();
    let l = number_as_string.len();
    let parity = l % 2;
    let mut sum: i64 = 0;

    let chars: Vec<i64> = number_as_string
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    for i in 0..l - 1 {
        if i % 2 != parity {
            sum += chars[i];
        } else if chars[i] > 4 {
            sum += 2 * chars[i] - 9;
        } else {
            sum += 2 * chars[i];
        }
    }
    chars[l - 1] == (10 - (sum % 10)) % 10
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17893729974));
        assert!(luhn_algorithm(79927398713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17893729975));
        assert!(!luhn_algorithm(17893729976));
        assert!(!luhn_algorithm(17893729977));
        assert!(!luhn_algorithm(123456));
    }
}
