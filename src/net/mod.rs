//! Provides methods for gathering net informations,
//!

use super::{result::*, util::*};
use sigar_sys::*;
use std::error::Error as stdError;
use std::ffi::CString;
use std::net;

// C: sigar_net_info_get
/// net info
#[derive(Debug)]
pub struct Info {
    pub default_gateway: Vec<u8>,
    pub default_gateway_interface: Vec<u8>,
    pub host_name: Vec<u8>,
    pub domain_name: Vec<u8>,
    pub primary_dns: Vec<u8>,
    pub secondary_dns: Vec<u8>,
}

/// Returns net info
pub fn info() -> SigarResult<Info> {
    let raw = ffi_wrap!(sigar_net_info_get, sigar_net_info_t)?;

    Ok(Info {
        default_gateway: chars_to_bytes(&raw.default_gateway[..]),
        default_gateway_interface: chars_to_bytes(&raw.default_gateway_interface[..]),
        host_name: chars_to_bytes(&raw.host_name[..]),
        domain_name: chars_to_bytes(&raw.domain_name[..]),
        primary_dns: chars_to_bytes(&raw.primary_dns[..]),
        secondary_dns: chars_to_bytes(&raw.secondary_dns[..]),
    })
}
// C: sigar_net_route_list_get
#[derive(Debug)]
pub enum AFFamily {
    UNSPEC,
    INET,
    INET6,
    LINK,
}

impl AFFamily {
    #[allow(non_upper_case_globals)]
    fn from_raw(raw: sigar_net_address_t__bindgen_ty_1) -> Self {
        match raw {
            sigar_net_address_t_SIGAR_AF_INET => AFFamily::INET,
            sigar_net_address_t_SIGAR_AF_INET6 => AFFamily::INET6,
            sigar_net_address_t_SIGAR_AF_LINK => AFFamily::LINK,
            _ => AFFamily::UNSPEC,
        }
    }
}

#[derive(Debug)]
pub struct Address {
    inet4: net::Ipv4Addr,
    inet6: net::Ipv6Addr,
    mac: [u8; 8usize],
}

impl Address {
    fn from_raw(raw: &sigar_net_address_t__bindgen_ty_2) -> Address {
        const MASK: u32 = !(0u32) << 16;
        unsafe {
            Address {
                inet4: net::Ipv4Addr::from(raw.in_),
                inet6: net::Ipv6Addr::new(
                    (raw.in6[0] >> 16) as u16,
                    (raw.in6[0] & MASK) as u16,
                    (raw.in6[1] >> 16) as u16,
                    (raw.in6[1] & MASK) as u16,
                    (raw.in6[2] >> 16) as u16,
                    (raw.in6[2] & MASK) as u16,
                    (raw.in6[3] >> 16) as u16,
                    (raw.in6[3] & MASK) as u16,
                ),
                mac: raw.mac.clone(),
            }
        }
    }
}

#[derive(Debug)]
pub struct NetAddress {
    pub family: AFFamily,
    pub address: Address,
}

impl NetAddress {
    fn from_raw(raw: &sigar_net_address_t) -> Self {
        NetAddress {
            family: AFFamily::from_raw(raw.family),
            address: Address::from_raw(&raw.addr),
        }
    }
}

#[derive(Debug)]
pub struct Route {
    pub destination: NetAddress,
    pub gateway: NetAddress,
    pub mask: NetAddress,
    pub flags: u64,
    pub refcnt: u64,
    pub use_: u64,
    pub metric: u64,
    pub mtu: u64,
    pub window: u64,
    pub irtt: u64,
    pub ifname: Vec<u8>,
}

impl Route {
    fn from_raw(raw: &sigar_net_route_t) -> Self {
        value_convert!(
            Route,
            raw,
            flags,
            refcnt,
            use_,
            metric,
            mtu,
            window,
            irtt,
            (destination: NetAddress::from_raw(&raw.destination)),
            (gateway: NetAddress::from_raw(&raw.gateway)),
            (mask: NetAddress::from_raw(&raw.mask)),
            (ifname: chars_to_bytes(&raw.ifname[..])),
        )
    }
}

pub fn route_list() -> SigarResult<Vec<Route>> {
    ffi_wrap_destroy!(
        sigar_net_route_list_get,
        sigar_net_route_list_destroy,
        sigar_net_route_list_t,
        (|list_t: &sigar_net_route_list_t| ffi_extract_list!(
            list_t,
            (|one: &sigar_net_route_t| Route::from_raw(one))
        ))
    )
}

// C: sigar_net_interface_config_get
#[derive(Debug)]
pub struct InterfaceConfig {
    pub name: Vec<u8>,
    pub type_: Vec<u8>,
    pub description: Vec<u8>,
    pub hwaddr: NetAddress,
    pub address: NetAddress,
    pub destination: NetAddress,
    pub broadcast: NetAddress,
    pub netmask: NetAddress,
    pub address6: NetAddress,
    pub prefix6_length: i32,
    pub scope6: i32,
    pub flags: u64,
    pub mtu: u64,
    pub metric: u64,
    pub tx_queue_len: i32,
}

impl InterfaceConfig {
    fn from_raw(raw: &sigar_net_interface_config_t) -> Self {
        value_convert!(
            InterfaceConfig,
            raw,
            prefix6_length,
            scope6,
            flags,
            mtu,
            metric,
            tx_queue_len,
            (name: chars_to_bytes(&raw.name[..])),
            (type_: chars_to_bytes(&raw.type_[..])),
            (description: chars_to_bytes(&raw.description[..])),
            (hwaddr: NetAddress::from_raw(&raw.hwaddr)),
            (address: NetAddress::from_raw(&raw.address)),
            (destination: NetAddress::from_raw(&raw.destination)),
            (broadcast: NetAddress::from_raw(&raw.broadcast)),
            (netmask: NetAddress::from_raw(&raw.netmask)),
            (address6: NetAddress::from_raw(&raw.address6)),
        )
    }
}

/// Returns interface config for given name
pub fn interface_config(name: &str) -> SigarResult<InterfaceConfig> {
    let name_ptr = CString::new(name).map_err(|e| Error::from_str(e.description()))?;
    let raw = ffi_wrap!(
        sigar_net_interface_config_get,
        (name_ptr.as_ptr()),
        sigar_net_interface_config_t
    )?;

    Ok(InterfaceConfig::from_raw(&raw))
}

// C: sigar_net_interface_config_primary_get

pub fn interface_config_primary() -> SigarResult<InterfaceConfig> {
    let raw = ffi_wrap!(
        sigar_net_interface_config_primary_get,
        sigar_net_interface_config_t
    )?;

    Ok(InterfaceConfig::from_raw(&raw))
}
// C: sigar_net_interface_stat_get
// C: sigar_net_interface_list_get
// C: sigar_net_interface_list_destroy
// C: sigar_net_connection_list_get
// C: sigar_net_connection_list_destroy
// C: sigar_net_connection_walk
// C: sigar_net_stat_get
// C: sigar_net_stat_port_get
// C: sigar_net_listen_address_get
// C: sigar_net_address_equals
// C: sigar_net_address_to_string
// C: sigar_net_scope_to_string
// C: sigar_net_address_hash
// C: sigar_net_connection_type_get
// C: sigar_net_connection_state_get
// C: sigar_net_interface_flags_to_string
// C: sigar_net_services_name_get
