use float_cmp::approx_eq;
use num_complex::Complex64;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Ket {
    first: Complex64,
    second: Complex64,
}

// Compare if one ket is equal to another.
// Complex64 also implements PartialEq for itself
// so we can use the == operator
impl PartialEq for Ket {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first
        && self.second == other.second
    }
}
impl Eq for Ket {}

impl Add for Ket {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            first: self.first + other.first,
            second: self.second + other.second,
        }
    }
}

pub const COMPLEX_ZERO: Complex64 =
    Complex64{ re: 0.0, im: 0.0 };
pub const COMPLEX_ONE: Complex64 =
    Complex64{ re: 1.0, im: 0.0 };
pub const KET_ZERO: Ket = Ket {
    first: COMPLEX_ONE,
    second: COMPLEX_ZERO,
};
pub const KET_ONE: Ket = Ket {
    first: COMPLEX_ZERO,
    second: COMPLEX_ONE,
};

#[test]
fn ket_zero_equal_to_itself() {
    assert!(KET_ZERO == KET_ZERO)
}

#[test]
fn ket_one_equal_to_itself() {
    assert!(KET_ONE == KET_ONE)
}

#[test]
fn ket_zero_not_equal_to_ket_one() {
    assert!(KET_ZERO != KET_ONE)
}

#[test]
fn ket_zero_add_ket_one() {
    let sum = KET_ZERO + KET_ONE;
    assert!(
        sum == Ket {
            first: COMPLEX_ONE,
            second: COMPLEX_ONE,
        },
    )
}