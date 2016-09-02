
use std::fmt;

#[derive(Debug, Clone)]
pub struct ExpectedErrorTemplate {
    input: String,
    expected: String,
    found: Option<String>,
    pos: u32
}

impl ExpectedErrorTemplate {
    pub fn new<S1, S2>(input: S1, expected: S2, found: Option<String>,
               pos: u32) -> ExpectedErrorTemplate where S1: Into<String>, S2: Into<String> {

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

pub fn create_location_string<S>(input: S, pos: u32) -> String where S: Into<String> {
    let mut res = input.into();
    res.push('\n');
    for _ in 0..pos {
        res.push(' ');
    }
    res.push_str("^~~~");

    res
}