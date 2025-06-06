//! Shared data structures between the kernel and userspace.
//!
//! In order to make sure endianness is correct, we store everything in byte-arrays in _big-endian_ order.
//! This makes it easier to directly take the values from the network buffer and use them in these structs (and vice-versa).

#![cfg_attr(not(feature = "std"), no_std)]

use core::net::{Ipv4Addr, Ipv6Addr};

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ClientAndChannelV4 {
    ipv4_address: [u8; 4],
    port: [u8; 2],
    channel: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ClientAndChannelV6 {
    ipv6_address: [u8; 16],
    port: [u8; 2],
    channel: [u8; 2],
}

impl ClientAndChannelV4 {
    pub fn new(ipv4_address: Ipv4Addr, port: u16, channel: u16) -> Self {
        Self {
            ipv4_address: ipv4_address.octets(),
            port: port.to_be_bytes(),
            channel: channel.to_be_bytes(),
        }
    }

    pub fn from_socket(src: core::net::SocketAddrV4, channel: u16) -> Self {
        Self::new(*src.ip(), src.port(), channel)
    }

    pub fn client_ip(&self) -> Ipv4Addr {
        self.ipv4_address.into()
    }

    pub fn client_port(&self) -> u16 {
        u16::from_be_bytes(self.port)
    }

    pub fn channel(&self) -> u16 {
        u16::from_be_bytes(self.channel)
    }
}

impl ClientAndChannelV6 {
    pub fn new(ipv6_address: Ipv6Addr, port: u16, channel: u16) -> Self {
        Self {
            ipv6_address: ipv6_address.octets(),
            port: port.to_be_bytes(),
            channel: channel.to_be_bytes(),
        }
    }

    pub fn from_socket(src: core::net::SocketAddrV6, channel: u16) -> Self {
        Self::new(*src.ip(), src.port(), channel)
    }

    pub fn client_ip(&self) -> Ipv6Addr {
        self.ipv6_address.into()
    }

    pub fn client_port(&self) -> u16 {
        u16::from_be_bytes(self.port)
    }

    pub fn channel(&self) -> u16 {
        u16::from_be_bytes(self.channel)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PortAndPeerV4 {
    ipv4_address: [u8; 4],
    allocation_port: [u8; 2],
    peer_port: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct PortAndPeerV6 {
    ipv6_address: [u8; 16],

    allocation_port: [u8; 2],
    peer_port: [u8; 2],
}

impl PortAndPeerV4 {
    pub fn new(ipv4_address: Ipv4Addr, allocation_port: u16, peer_port: u16) -> Self {
        Self {
            ipv4_address: ipv4_address.octets(),
            allocation_port: allocation_port.to_be_bytes(),
            peer_port: peer_port.to_be_bytes(),
        }
    }

    pub fn from_socket(dst: core::net::SocketAddrV4, allocation_port: u16) -> Self {
        Self::new(*dst.ip(), allocation_port, dst.port())
    }

    pub fn peer_ip(&self) -> Ipv4Addr {
        self.ipv4_address.into()
    }

    pub fn allocation_port(&self) -> u16 {
        u16::from_be_bytes(self.allocation_port)
    }

    pub fn peer_port(&self) -> u16 {
        u16::from_be_bytes(self.peer_port)
    }
}

impl PortAndPeerV6 {
    pub fn new(ipv6_address: Ipv6Addr, allocation_port: u16, peer_port: u16) -> Self {
        Self {
            ipv6_address: ipv6_address.octets(),

            allocation_port: allocation_port.to_be_bytes(),
            peer_port: peer_port.to_be_bytes(),
        }
    }

    pub fn from_socket(dst: core::net::SocketAddrV6, allocation_port: u16) -> Self {
        Self::new(*dst.ip(), allocation_port, dst.port())
    }

    pub fn peer_ip(&self) -> Ipv6Addr {
        self.ipv6_address.into()
    }

    pub fn allocation_port(&self) -> u16 {
        u16::from_be_bytes(self.allocation_port)
    }

    pub fn peer_port(&self) -> u16 {
        u16::from_be_bytes(self.peer_port)
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Config {
    udp_checksum_enabled: bool,
    lowest_allocation_port: [u8; 2],
    highest_allocation_port: [u8; 2],
}

impl Config {
    pub fn udp_checksum_enabled(&self) -> bool {
        self.udp_checksum_enabled
    }

    pub fn with_udp_checksum(self, enabled: bool) -> Self {
        Self {
            udp_checksum_enabled: enabled,
            ..self
        }
    }

    pub fn lowest_allocation_port(&self) -> u16 {
        u16::from_be_bytes(self.lowest_allocation_port)
    }

    pub fn with_lowest_allocation_port(self, port: u16) -> Self {
        Self {
            lowest_allocation_port: port.to_be_bytes(),
            ..self
        }
    }

    pub fn highest_allocation_port(&self) -> u16 {
        u16::from_be_bytes(self.highest_allocation_port)
    }

    pub fn with_highest_allocation_port(self, port: u16) -> Self {
        Self {
            highest_allocation_port: port.to_be_bytes(),
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            udp_checksum_enabled: true,
            lowest_allocation_port: 49152_u16.to_be_bytes(),
            highest_allocation_port: 65535_u16.to_be_bytes(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct StatsEvent {
    pub relayed_data: u64,
}

impl StatsEvent {
    #[cfg(feature = "std")]
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let (chunk, _) = bytes.split_first_chunk()?;
        let relayed_data = u64::from_ne_bytes(*chunk);

        Some(Self { relayed_data })
    }
}

#[cfg(all(feature = "std", target_os = "linux"))]
mod userspace {
    use super::*;

    unsafe impl aya::Pod for ClientAndChannelV4 {}

    unsafe impl aya::Pod for PortAndPeerV4 {}

    unsafe impl aya::Pod for ClientAndChannelV6 {}

    unsafe impl aya::Pod for PortAndPeerV6 {}

    unsafe impl aya::Pod for Config {}
}
