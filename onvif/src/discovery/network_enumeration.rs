use std::net::Ipv4Addr;

#[inline]
fn octets_to_u32(octets: [u8; 4]) -> u32 {
    (octets[0] as u32) << (3 * 8)
        | (octets[1] as u32) << (2 * 8)
        | (octets[2] as u32) << 8
        | (octets[3] as u32)
}

/// Enumerate the list of IPs on the network given the network address and the mask.
pub fn enumerate_network_v4(network: Ipv4Addr, mask: Ipv4Addr) -> Vec<Ipv4Addr> {
    let network = octets_to_u32(network.octets());
    let mask = octets_to_u32(mask.octets());

    let mask = !mask;

    let mut ips = Vec::with_capacity(mask as usize);

    for value in 1..mask {
        let addr = network | value;
        ips.push(Ipv4Addr::from(addr))
    }

    ips
}

/// Tests the enumeration method. See http://jodies.de/ipcalc for examples.
#[cfg(test)]
mod test_enumerate_v4 {
    use super::*;

    #[test]
    pub fn test_basic_home_network() {
        let home_net = Ipv4Addr::new(192, 168, 0, 0);
        let net_mask = Ipv4Addr::new(255, 255, 255, 0);

        let ips = enumerate_network_v4(home_net, net_mask);

        assert_eq!(254, ips.len())
    }

    #[test]
    pub fn test_more_complex_net() {
        let home_net = Ipv4Addr::new(192, 168, 0, 0);
        let net_mask = Ipv4Addr::new(255, 255, 254, 0);

        let ips = enumerate_network_v4(home_net, net_mask);

        dbg!(&ips);

        assert_eq!(510, ips.len())
    }
}
