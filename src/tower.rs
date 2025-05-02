use std::net::{IpAddr};
use crate::geo;

struct Tower {
    addr: IpAddr,
    port: u16,
    location: geo::LatLon,
}


