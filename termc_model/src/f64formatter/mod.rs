use std::fmt;
use num::traits::{Zero, One, Num, ParseFloatError};
use std::ops::{Add, Div, Mul, Sub, Rem};
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone, Debug, Default, PartialOrd)]
pub struct F64Formatter(pub f64);

impl Num for F64Formatter {
    type FromStrRadixErr = ParseFloatError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(F64Formatter(f64::from_str_radix(str, radix)?))
    }
}

impl Zero for F64Formatter {
    fn zero() -> Self {
        F64Formatter(f64::zero())
    }
    fn is_zero(&self) -> bool {
        return self.0 == f64::zero()
    }
}

impl One for F64Formatter {
    fn one() -> Self {
        F64Formatter(f64::one())
    }
}

impl Add for F64Formatter {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        F64Formatter(self.0 + rhs.0)
    }
}

impl Sub for F64Formatter {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        F64Formatter(self.0 - rhs.0)
    }
}

impl Mul for F64Formatter {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        F64Formatter(self.0 * rhs.0)
    }
}

impl Div for F64Formatter {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        F64Formatter(self.0 / rhs.0)
    }
}

impl Rem for F64Formatter {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        F64Formatter(self.0 % rhs.0)
    }
}

impl F64Formatter {
    fn lookup_binary(val: f64) -> char {
        lazy_static!{
            static ref BINARY_MAP : HashMap<u64, char> = {
                let mut m = HashMap::new();
                m.insert(0, '0');
                m.insert(1, '1');
                m
            };
        }
        let pre = val as u64;
        BINARY_MAP.get(&pre).unwrap().clone()
    }

    fn lookup_octal(val: f64) -> char {
        lazy_static!{
            static ref OCTAL_MAP : HashMap<u64, char> = {
                let mut m = HashMap::new();
                m.insert(0, '0');
                m.insert(1, '1');
                m.insert(2, '2');
                m.insert(3, '3');
                m.insert(4, '4');
                m.insert(5, '5');
                m.insert(6, '6');
                m.insert(7, '7');
                m
            };
        }
        let pre = val as u64;
        OCTAL_MAP.get(&pre).unwrap().clone()
    }

    fn lookup_lower_hex(val: f64) -> char {
        lazy_static!{
            static ref HEX_MAP : HashMap<u64, char> = {
                let mut m = HashMap::new();
                m.insert(0, '0');
                m.insert(1, '1');
                m.insert(2, '2');
                m.insert(3, '3');
                m.insert(4, '4');
                m.insert(5, '5');
                m.insert(6, '6');
                m.insert(7, '7');
                m.insert(8, '8');
                m.insert(9, '9');
                m.insert(10, 'a');
                m.insert(11, 'b');
                m.insert(12, 'c');
                m.insert(13, 'd');
                m.insert(14, 'e');
                m.insert(15, 'f');
                m
            };
        }
        let pre = val as u64;
        HEX_MAP.get(&pre).unwrap().clone()
    }

    fn lookup_upper_hex(val: f64) -> char {
        F64Formatter::lookup_lower_hex(val).to_uppercase().to_string().pop().unwrap().clone()
    }
}

macro_rules! format_pre_dp {
    ($f:ident, $val:ident, $typ:tt) => {{
        let abs = $val.abs();
        let pre_dp : u64 = abs as u64;
        if $f.alternate() {
            format!(concat!("{0:#", $typ, "}"), pre_dp)
        }
        else {
            format!(concat!("{0:", $typ, "}"), pre_dp)
        }
    }}
}

macro_rules! format_post_dp {
    ($val:ident, $base:tt, $lookup:ident, $prec:tt) => {{
        let abs = $val.abs();
        let mut post_dp : f64 = abs - ((abs as u64) as f64);
        let mut repr = String::new();

        let mut n = 0;
        while n < $prec {
            if post_dp == 0.0_f64 {
                break;
            }

            post_dp *= $base;
            repr.push(F64Formatter::$lookup(post_dp));
            post_dp -= (post_dp as u64) as f64;
            n += 1;
        }

        repr
    }}
}

macro_rules! write_formatter {
    ($f:ident, $formatter:ident, $fmt_type:tt, $base:tt, $lookup:ident) => {{
        let abs = $formatter.0.abs();
        let pre_repr = format_pre_dp!($f, abs, $fmt_type);
        let post_repr = if let Some(prec) = $f.precision() {
            format_post_dp!(abs, $base, $lookup, prec)
        }
        else {
            format_post_dp!(abs, $base, $lookup, 10)
        };
        //if post_repr != "0" {
        if post_repr != "" {
            write!($f, "{0}.{1}", pre_repr, post_repr)
        }
        else {
            write!($f, "{0}", pre_repr)
        }
    }}
}

impl fmt::Binary for F64Formatter {

    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        write_formatter!(f, self, "b", 2.0_f64, lookup_binary)
    }
}

impl fmt::Octal for F64Formatter {

    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        write_formatter!(f, self, "o", 8.0_f64, lookup_octal)
    }
}

impl fmt::LowerHex for F64Formatter {

    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        write_formatter!(f, self, "x", 16.0_f64, lookup_lower_hex)
    }
}

impl fmt::UpperHex for F64Formatter {

    fn fmt(& self, f: & mut fmt::Formatter) -> fmt::Result {

        write_formatter!(f, self, "X", 16.0_f64, lookup_upper_hex)
    }
}

impl From<f64> for F64Formatter {
    fn from(val: f64) -> Self {
        F64Formatter(val)
    }
}