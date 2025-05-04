use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::geo::{self, LatLon};

struct Tower {
    addr: u16,
    phys_addr: IpAddr,
    port: u16,
    location: geo::LatLon,
}

impl Tower {
    fn new() -> Tower {
        Tower {
            addr: 0xc1ac,
            phys_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 10,
            location: LatLon::new(10.1, 1.0)
        }
    }

    fn stand_up() -> Result<bool, String> {
        
        
        Err("Something went wrong.".to_string())
    }
}
