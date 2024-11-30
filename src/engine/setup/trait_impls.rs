//! Contains implementations of traits for the `Value` and `Depth` types.
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::setup::depth::Depth;
use crate::setup::values::Value;

impl std::ops::Neg for Value {
    type Output = Self;
    fn neg(self) -> Self {
        Value(-self.0)
    }
}

impl Add<Value> for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Value(self.0 + rhs.0)
    }
}

impl Add<i16> for Value {
    type Output = Self;
    fn add(self, rhs: i16) -> Self {
        self + Value(rhs)
    }
}

impl Sub<i16> for Value {
    type Output = Self;
    fn sub(self, rhs: i16) -> Self {
        self - Value(rhs)
    }
}

impl Sub<Value> for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Value(self.0 - rhs.0)
    }
}

impl Mul<Value> for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Value(self.0 * rhs.0)
    }
}

impl AddAssign<Value> for Value {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i16> for Value {
    fn add_assign(&mut self, rhs: i16) {
        *self = *self + rhs;
    }
}

impl SubAssign<Value> for Value {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<i16> for Value {
    fn sub_assign(&mut self, rhs: i16) {
        *self = *self - rhs;
    }
}

impl Mul<i16> for Value {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self {
        Value(self.0 * rhs)
    }
}

impl std::ops::MulAssign<i16> for Value {
    fn mul_assign(&mut self, rhs: i16) {
        *self = *self * rhs;
    }
}

impl Mul<Value> for i16 {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value {
        Value(self * rhs.0)
    }
}

impl std::ops::Div<i16> for Value {
    type Output = Self;
    fn div(self, rhs: i16) -> Self {
        Value(self.0 / rhs)
    }
}

impl std::ops::DivAssign<i16> for Value {
    fn div_assign(&mut self, rhs: i16) {
        *self = *self / rhs;
    }
}

impl std::ops::Div<Value> for Value {
    type Output = i16;
    fn div(self, rhs: Self) -> i16 {
        self.0 / rhs.0
    }
}

/// Returns the score for when the engine believes it can checkmate within the
/// given depth
pub fn mate_in(ply: i16) -> Value {
    Value::MATE - ply
}

/// Returns the score for when the engine expects to be checkmated within the
/// given depth
pub fn mated_in(ply: i16) -> Value {
    -Value::MATE + ply
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value(v as i16)
    }
}

impl AddAssign for Depth {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub<u16> for Depth {
    type Output = Depth;

    fn sub(self, rhs: u16) -> Self::Output {
        Depth(self.0 - rhs)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value(v as i16)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value(v as i16)
    }
}

impl AddAssign<u32> for Value {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs as i16
    }
}

impl SubAssign<u32> for Value {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs as i16
    }
}

impl Sub<Value> for i32 {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        Value::from(self - rhs.0 as i32)
    }
}

impl Mul<u32> for Value {
    type Output = Value;

    fn mul(self, rhs: u32) -> Self::Output {
        Value(self.0 * rhs as i16)
    }
}

impl Mul<f64> for Value {
    type Output = Value;

    fn mul(self, rhs: f64) -> Self::Output {
        Value((self.0 as f64 * rhs) as i16)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value(value as i16)
    }
}
