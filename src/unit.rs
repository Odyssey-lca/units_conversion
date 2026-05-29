use std::fmt;

use serde::{Deserialize, Serialize};

use crate::dimension::*;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Unit {
    pub dimension: Dimension,
    pub scale_to_si: f64,
}

impl Unit {

    /// Converts the current unit into another target unit, if their physical dimensions match.
    ///
    /// # Examples
    /// 
    /// ```
    /// use units_conversion::parser::parse_unit;
    /// 
    /// let hour = parse_unit("h").unwrap();
    /// let second = parse_unit("s").unwrap();
    /// 
    /// // Successful conversion (same dimensions)
    /// if let Some(converted) = hour.convert(&second) {
    ///     assert_eq!(converted.scale_to_si, 3600.0);
    /// }
    /// 
    /// // Failed conversion (different dimensions: time to length)
    /// let meter = parse_unit("m").unwrap();
    /// assert!(hour.convert(&meter).is_none());
    /// ```
    pub fn convert(self, to: &Unit) -> Option<Unit> {
        if self.dimension == to.dimension {
            let scale_to_si = self.scale_to_si / to.scale_to_si;
            let dimension = self.dimension;
            Some(Unit { dimension, scale_to_si })
        } else {
            None
        }
    }

    pub fn pow(&self, exp: i32) -> Unit {
        let scale_to_si = self.scale_to_si.powi(exp);
        let dimension = self.dimension.pow(exp);
        Unit { dimension, scale_to_si }
    }
}

impl std::ops::Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            scale_to_si: self.scale_to_si * rhs.scale_to_si,
            dimension: self.dimension * rhs.dimension,
        }
    }
}

impl std::ops::Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            scale_to_si: self.scale_to_si / rhs.scale_to_si,
            dimension: self.dimension / rhs.dimension,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{};{}",
            self.dimension.time,
            self.dimension.mass,
            self.dimension.length,
            self.dimension.temperature,
            self.dimension.current,
            self.dimension.amount,
        )
    }
}
