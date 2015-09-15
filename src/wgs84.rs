
/// Provides RADIUS, FLATTENING, and POLAR_RADIUS constants from the reference 
/// ellipsoid. Inpired by [mapbox-wgs84](https://github.com/mapbox/wgs84)

pub const RADIUS: f64 = 6378137.0; // meters
pub const FLATTENING_DENOM: f64 = 298.257223563;
pub const FLATTENING: f64 = 1.0/FLATTENING_DENOM;
pub const POLAR_RADIUS: f64 = RADIUS * (1.0 - FLATTENING);