use alpha_core::{Zero, One, HalfBakedPID, Field};

// Rational numbers
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Rational {
    numerator: u64,
    denumerator: u64,
    sign: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub struct C64(f64, f64);

// Big unsigned integers
#[derive(PartialEq, Eq, Clone)]
pub struct Ubig( Vec<u64> );

// Big unsigned integers
#[derive(PartialEq, Eq, Clone)]
pub struct Ibig( Vec<i64> );

#[macro_export]
macro_rules! rational {
    ($a:expr; $b:expr) => { {
        Rational::new($a as i64, $b as i64)
    } };

    ($a:expr) => {{
        Rational::new($a,1)
    }};
}

mod rational_impl;

use alpha_core::{pid_impl_for_fields, field_impl};
use alpha_core::UniqueFactorization;
use omega_core::IdentityAlgorithm;
pid_impl_for_fields!{ Rational }
field_impl!{ Rational }

mod integral;