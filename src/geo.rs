// This module contains any and all functionality related for handling the
// geolocation of towers in roundworld. This is used to determine the best
// network to route clacks messages through. We don't actually care about the
// real location of a tower, feel free to fake it.
use crate::maths;

pub struct LatLon {
    lat: f64,
    lon: f64
}

const EARTH_RADIUS: f64 = 6371.008; // kms

impl LatLon {
    pub fn new(latitude: f64, longitude: f64) -> LatLon {
        LatLon {
            lat: latitude,
            lon: longitude
        }
    }

    /// The haversine distance between two coordinates on a sphere as kilometres
    pub fn distance(self, b: LatLon) -> f64 {
        // TODO: make this calculate on a proper ellipsoid, this is good enough
        // for the most part now.
        let d: LatLon = LatLon {
            lat: (self.lat - b.lat).abs().to_radians(),
            lon: (self.lon - b.lon).abs().to_radians()
        };

        let t = maths::hav(d.lat) +
            self.lat.to_radians().cos() * b.lat.to_radians().cos() * maths::hav(d.lon);

        return 2. * EARTH_RADIUS * t.sqrt().asin();
    }
}
