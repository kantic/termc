
use std::fmt;

#[derive(Debug, Clone)]
pub struct ExpectedErrorTemplate {
    input: String,
    expected: String,
    found: Option<String>,
    pos: usize
}

impl ExpectedErrorTemplate {
    pub fn new<S1, S2>(input: S1, expected: S2, found: Option<String>,
               pos: usize) -> ExpectedErrorTemplate where S1: Into<String>, S2: Into<String> {

        ExpectedErrorTemplate {input: input.into(), expected: expected.into(),
            found: found, pos: pos}
    }
}

impl fmt::Display for ExpectedErrorTemplate {

    /// Returns the formatted error message.
    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        let location_part = create_location_string(self.input.clone(), self.pos);

        let mut found_part = String::new();
        if self.found.is_some() {
            found_part.push_str(" Found: ");
            found_part.push_str(self.found.as_ref().unwrap());
        }

        write!(f, "Error: Expected {}.\n{}{}", self.expected, location_part, found_part)
    }
}

/// Creates a string that sets a marker at the specified position. The result is the input string with the marker set.
pub fn create_location_string<S>(input: S, pos: usize) -> String where S: Into<String> {
    let mut res = input.into();
    res.push('\n');
    for _ in 0..pos {
        res.push(' ');
    }
    res.push_str("^~~~");

    res
}
