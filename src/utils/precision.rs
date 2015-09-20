
pub trait ToPrecision<T> {
  fn to_precision(&self, precision: i32) -> T;
}

impl ToPrecision<f32> for f32 {
  fn to_precision(&self, precision: i32) -> f32 {
    let rounder = 10_f32.powi(precision);
    (self * rounder).round() / rounder
  }
}

impl ToPrecision<f64> for f64 {
  fn to_precision(&self, precision: i32) -> f64 {
    let rounder = 10_f64.powi(precision);
    (self * rounder).round() / rounder
  }
}

#[cfg(test)]
mod tests {
  use super::ToPrecision;

  #[test]
  fn test_f32() {
    let x = 12.34567_f32;
    let y = x.to_precision(4);
    assert_eq!(12.34570, y);
  }

  #[test]
  fn test_f64() {
    let x = 12.345678910_f64;
    let y = x.to_precision(8);
    assert_eq!(12.34567891, y);
  }
}