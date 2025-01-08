//! Run this file with `cargo test --test 05_srl_validator`.

// TODO: Implement a SRL (Simple Resource Locator) validator.
// A SRL consists of two parts, an optional protocol (string) and an address (string).
// The format of the SRL looks like this: `[<protocol>://]<address>`
// The protocol and the address have to contain only lowercase English characters.
// Protocol must not be empty if :// is present in the SRL.
// Address must not be empty.
//
// As an example, these are valid SRLs:
// - `http://foo`
// - `bar://baz`
// - `foobar`
//
// And these are invalid SRLs:
// - `http://foo1` (invalid character in address)
// - `asd://bar://` (invalid character in address)
// - `://baz` (empty protocol)
// - `01://baz` (invalid character in protocol)
//
// Create a struct `SRL` in a module named `srl`. Expose functions for parsing a SRL and getting
// its individual parts, but do not allow modifying the fields of `SRL` outside its module.
// Do not use regular expressions, SRLs can be easily parsed with a big of parsing logic.
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `SRL` and `SRLValidationError`,
// so that asserts in tests work.

mod srl {
    #[derive(Debug, Eq, PartialEq)]
    pub enum SRLValidationError {
        EmptyAddress,
        EmptyProtocol,
        InvalidCharacterInProtocol(char),
        InvalidCharacterInAddress(char),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct SRL {
        protocol: Option<String>,
        address: String,
    }

    impl SRL {
        pub fn new(srl: &str) -> Result<Self, SRLValidationError> {
            if srl.is_empty() {
                return Err(SRLValidationError::EmptyAddress);
            }
            if srl.starts_with("://") {
                return Err(SRLValidationError::EmptyProtocol);
            }
            let parts: Vec<&str> = srl.split("://").collect();
            if parts.len() > 2 {
                return Err(SRLValidationError::InvalidCharacterInAddress(':'));
            }
            let address = if parts.len() > 1 {
                parts[1].to_string()
            } else {
                parts[0].to_string()
            };
            let protocol = if parts.len() > 1 {
                Some(parts[0].to_string())
            } else {
                None
            };
            if let Some(p) = protocol.clone() {
                for c in p.chars() {
                    if !c.is_ascii_lowercase() {
                        return Err(SRLValidationError::InvalidCharacterInProtocol(c));
                    }
                }
            }
            for c in address.chars() {
                if !c.is_ascii_lowercase() {
                    return Err(SRLValidationError::InvalidCharacterInAddress(c));
                }
            }

            Ok(Self { protocol, address })
        }

        pub fn get_protocol(&self) -> Option<&str> {
            match &self.protocol {
                Some(t) => Some(t),
                None => None,
            }
        }

        pub fn get_address(&self) -> &str {
            &self.address
        }
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::srl::{SRLValidationError, SRL};

    #[test]
    fn empty_address() {
        assert_eq!(
            //SRL::new("".to_ascii_lowercase()),
            SRL::new(""),
            Err(SRLValidationError::EmptyAddress)
        );
    }

    #[test]
    fn only_separator() {
        assert_eq!(SRL::new("://"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn empty_protocol() {
        assert_eq!(SRL::new("://foo"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn multiple_protocols() {
        assert_eq!(
            SRL::new("ab://bc://foo"),
            Err(SRLValidationError::InvalidCharacterInAddress(':'))
        );
    }

    #[test]
    fn invalid_protocol() {
        assert_eq!(
            SRL::new("bAc://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('A'))
        );
        assert_eq!(
            SRL::new("a02://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('0'))
        );
    }

    #[test]
    fn invalid_address_with_protocol() {
        assert_eq!(
            SRL::new("abc://fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            SRL::new("bar://fooBZcX"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_address_without_protocol() {
        assert_eq!(
            SRL::new("fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            SRL::new("fooBAc"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_protocol_and_address() {
        assert_eq!(
            SRL::new("bZcA://fo2o"),
            Err(SRLValidationError::InvalidCharacterInProtocol('Z'))
        );
        assert_eq!(
            SRL::new("a20://barBAZ"),
            Err(SRLValidationError::InvalidCharacterInProtocol('2'))
        );
    }

    #[test]
    fn invalid_char_emoji() {
        assert_eq!(
            SRL::new("asd://fo🙃o"),
            Err(SRLValidationError::InvalidCharacterInAddress('🙃'))
        );
    }

    #[test]
    fn no_protocol() {
        let srl = SRL::new("foobar").unwrap();
        assert_eq!(srl.get_protocol(), None);
        assert_eq!(srl.get_address(), "foobar");
    }

    #[test]
    fn protocol_and_scheme() {
        let srl = SRL::new("bar://foobar").unwrap();
        assert_eq!(srl.get_protocol(), Some("bar"));
        assert_eq!(srl.get_address(), "foobar");
    }
}
