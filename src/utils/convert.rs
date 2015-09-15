
const FEET_PER_METER: f64 = 3.28084;
const MILE_PER_KM: f64 = 0.621371;

pub fn m_to_ft(m: f64) -> f64 {
  m * FEET_PER_METER
}

pub fn km_to_mi(km: f64) -> f64 {
  km * MILE_PER_KM
}

#[cfg(test)]
mod tests {
  use super::{FEET_PER_METER, MILE_PER_KM, m_to_ft, km_to_mi};

  #[test]
  fn test_m_to_ft() {
    assert_eq!(m_to_ft(1.0), FEET_PER_METER);
  }

  #[test]
  fn test_km_to_mi() {
    assert_eq!(km_to_mi(1.0), MILE_PER_KM);
  }
}