use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::str::FromStr;

// use decimal::d128;
use rust_decimal::Decimal;

// #[derive(Hash, Clone, Copy, Debug, PartialEq, PartialOrd)]
// pub struct D128(Decimal);

// impl Eq for D128 {}
// impl Ord for D128 {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         use std::cmp::Ordering;
//         if self.0 < other.0 {
//             Ordering::Less
//         } else if self.0 > other.0 {
//             Ordering::Greater
//         } else {
//             Ordering::Equal
//         }
//     }
// }
// impl Add for D128 {
//     type Output = D128;

//     fn add(self, rhs: Self) -> Self::Output {
//         D128(self.0 + rhs.0)
//     }
// }
// impl AddAssign for D128 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.0 += rhs.0;
//     }
// }
// impl Sub for D128 {
//     type Output = D128;

//     fn sub(self, rhs: Self) -> Self::Output {
//         D128(self.0 - rhs.0)
//     }
// }
// impl Mul for D128 {
//     type Output = D128;

//     fn mul(self, rhs: Self) -> Self::Output {
//         D128(self.0 * rhs.0)
//     }
// }
// impl Div for D128 {
//     type Output = D128;

//     fn div(self, rhs: Self) -> Self::Output {
//         D128(self.0 / rhs.0)
//     }
// }
// impl AsRef<Decimal> for D128 {
//     fn as_ref(&self) -> &Decimal {
//         &self.0
//     }
// }
// impl From<f64> for D128 {
//     fn from(value: f64) -> Self {
//         let v = d128::from_str(&value.to_string()).unwrap();
//         D128(v)
//     }
// }
// impl TryFrom<&str> for D128 {
//     type Error = ();

//     fn try_from(value: &str) -> Result<Self, ()> {
//         let v = d128::from_str(value)?;
//         Ok(D128(v))
//     }
// }
// impl D128 {
//     pub fn pow<O: AsRef<d128>>(self, exp: O) -> D128 {
//         D128(self.0.pow(exp.as_ref()))
//     }
// }

pub trait Data {
    fn y(&self, x: &Decimal) -> Option<Decimal>;
}
