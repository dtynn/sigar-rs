//! Provides methods for gathering net informations,
//!

use super::{result::*, util::*};
use sigar_sys::*;
use std::error::Error as stdError;
use std::ffi::{CStr, CString};
use std::net;
use std::os::raw::{c_int, c_ulong};

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
        const MASK_U16: u32 = !(0u32) >> 16;

        unsafe {
            let in6: [u32; 4] = [
                u32_reverse(raw.in6[0]),
                u32_reverse(raw.in6[1]),
                u32_reverse(raw.in6[2]),
                u32_reverse(raw.in6[3]),
            ];

            Address {
                inet4: net::Ipv4Addr::from(u32_reverse(raw.in_)),
                inet6: net::Ipv6Addr::new(
                    (in6[0] >> 16) as u16,
                    (in6[0] & MASK_U16) as u16,
                    (in6[1] >> 16) as u16,
                    (in6[1] & MASK_U16) as u16,
                    (in6[2] >> 16) as u16,
                    (in6[2] & MASK_U16) as u16,
                    (in6[3] >> 16) as u16,
                    (in6[3] & MASK_U16) as u16,
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
/// Returns config for primary interface
pub fn interface_config_primary() -> SigarResult<InterfaceConfig> {
    let raw = ffi_wrap!(
        sigar_net_interface_config_primary_get,
        sigar_net_interface_config_t
    )?;

    Ok(InterfaceConfig::from_raw(&raw))
}

// C: sigar_net_interface_stat_get
#[derive(Debug)]
pub struct InterfaceStat {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
    pub rx_overruns: u64,
    pub rx_frame: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
    pub tx_overruns: u64,
    pub tx_collisions: u64,
    pub tx_carrier: u64,
    pub speed: u64,
}

/// Returns interface stat for give name
pub fn interface_stat(name: &str) -> SigarResult<InterfaceStat> {
    let name_ptr = CString::new(name).map_err(|e| Error::from_str(e.description()))?;

    let raw = ffi_wrap!(
        sigar_net_interface_stat_get,
        (name_ptr.as_ptr()),
        sigar_net_interface_stat_t
    )?;

    Ok(value_convert!(
        InterfaceStat,
        raw,
        rx_packets,
        rx_bytes,
        rx_errors,
        rx_dropped,
        rx_overruns,
        rx_frame,
        tx_packets,
        tx_bytes,
        tx_errors,
        tx_dropped,
        tx_overruns,
        tx_collisions,
        tx_carrier,
        speed,
    ))
}

// C: sigar_net_interface_list_get

/// Returns interface names
pub fn interface_list() -> SigarResult<Vec<CString>> {
    ffi_wrap_destroy!(
        sigar_net_interface_list_get,
        sigar_net_interface_list_destroy,
        sigar_net_interface_list_t,
        (|list_ptr: &sigar_net_interface_list_t| ffi_extract_list!(
            list_ptr,
            (|one: &*mut ::std::os::raw::c_char| CStr::from_ptr(*one).to_owned())
        ))
    )
}

// C: sigar_net_connection_list_get
// C: sigar_net_connection_list_destroy
#[derive(Debug)]
pub struct Conn {
    pub local_port: u64,
    pub local_address: NetAddress,
    pub remote_port: u64,
    pub remote_address: NetAddress,
    pub uid: u32,
    pub inode: u64,
    pub type_: ConnType,
    pub state: ConnSate,
    pub send_queue: u64,
    pub receive_queue: u64,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ConnType {
    TCP,
    UDP,
    RAW,
    UNIX,
    UNKNOWN,
}

impl ConnType {
    fn from_raw(raw: c_int) -> Self {
        match raw as u32 {
            SIGAR_NETCONN_TCP => ConnType::TCP,
            SIGAR_NETCONN_UDP => ConnType::UDP,
            SIGAR_NETCONN_RAW => ConnType::RAW,
            SIGAR_NETCONN_UNIX => ConnType::UNIX,
            _ => ConnType::UNKNOWN,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ConnSate {
    TCP_ESTABLISHED,
    TCP_SYN_SENT,
    TCP_SYN_RECV,
    TCP_FIN_WAIT1,
    TCP_FIN_WAIT2,
    TCP_TIME_WAIT,
    TCP_CLOSE,
    TCP_CLOSE_WAIT,
    TCP_LAST_ACK,
    TCP_LISTEN,
    TCP_CLOSING,
    TCP_IDLE,
    TCP_BOUND,
    TCP_UNKNOWN,
}

impl ConnSate {
    fn from_raw(raw: c_int) -> Self {
        match raw as u32 {
            SIGAR_TCP_ESTABLISHED => ConnSate::TCP_ESTABLISHED,
            SIGAR_TCP_SYN_SENT => ConnSate::TCP_SYN_SENT,
            SIGAR_TCP_SYN_RECV => ConnSate::TCP_SYN_RECV,
            SIGAR_TCP_FIN_WAIT1 => ConnSate::TCP_FIN_WAIT1,
            SIGAR_TCP_FIN_WAIT2 => ConnSate::TCP_FIN_WAIT2,
            SIGAR_TCP_TIME_WAIT => ConnSate::TCP_TIME_WAIT,
            SIGAR_TCP_CLOSE => ConnSate::TCP_CLOSE,
            SIGAR_TCP_CLOSE_WAIT => ConnSate::TCP_CLOSE_WAIT,
            SIGAR_TCP_LAST_ACK => ConnSate::TCP_LAST_ACK,
            SIGAR_TCP_LISTEN => ConnSate::TCP_LISTEN,
            SIGAR_TCP_CLOSING => ConnSate::TCP_CLOSING,
            SIGAR_TCP_IDLE => ConnSate::TCP_IDLE,
            SIGAR_TCP_BOUND => ConnSate::TCP_BOUND,
            SIGAR_TCP_UNKNOWN => ConnSate::TCP_UNKNOWN,
            _ => ConnSate::TCP_UNKNOWN,
        }
    }
}

impl Conn {
    fn from_raw(raw: &sigar_net_connection_t) -> Self {
        value_convert!(
            Conn,
            raw,
            local_port,
            remote_port,
            uid,
            inode,
            send_queue,
            receive_queue,
            (local_address: NetAddress::from_raw(&raw.local_address)),
            (remote_address: NetAddress::from_raw(&raw.remote_address)),
            (state: ConnSate::from_raw(raw.state)),
            (type_: ConnType::from_raw(raw.type_)),
        )
    }
}

type Flag = u32;
pub const FLAG_NETCONN_CLIENT: Flag = SIGAR_NETCONN_CLIENT;
pub const FLAG_NETCONN_SERVER: Flag = SIGAR_NETCONN_SERVER;
pub const FLAG_NETCONN_TCP: Flag = SIGAR_NETCONN_TCP;
pub const FLAG_NETCONN_UDP: Flag = SIGAR_NETCONN_UDP;
pub const FLAG_NETCONN_RAW: Flag = SIGAR_NETCONN_RAW;
pub const FLAG_NETCONN_UNIX: Flag = SIGAR_NETCONN_UNIX;

/// Returns all connections for given flags
pub fn connection_list(flags: Flag) -> SigarResult<Vec<Conn>> {
    ffi_wrap_destroy!(
        (|ptr: *mut sigar_t, connlist: *mut sigar_net_connection_list_t| {
            sigar_net_connection_list_get(ptr, connlist, flags as c_int)
        }),
        sigar_net_connection_list_destroy,
        sigar_net_connection_list_t,
        (|list_ptr: &sigar_net_connection_list_t| ffi_extract_list!(
            list_ptr,
            (|one: &sigar_net_connection_t| Conn::from_raw(one))
        ))
    )
}

// C: sigar_net_stat_get
#[derive(Debug)]
pub struct Stat {
    pub tcp_states: [i32; 14usize],
    pub tcp_inbound_total: u32,
    pub tcp_outbound_total: u32,
    pub all_inbound_total: u32,
    pub all_outbound_total: u32,
}

impl Stat {
    fn from_raw(raw: &sigar_net_stat_t) -> Self {
        let mut tcp_states: [i32; 14usize] = [0; 14usize];
        let mut i = 0usize;
        while i < 14 {
            tcp_states[i] = raw.tcp_states[i] as i32;
            i += 1;
        }

        value_convert!(
            Stat,
            raw,
            tcp_inbound_total,
            tcp_outbound_total,
            all_inbound_total,
            all_outbound_total,
            (tcp_states: tcp_states),
        )
    }
}

/// Returns connection stat summary for given flags
pub fn stat_get(flags: Flag) -> SigarResult<Stat> {
    let raw = ffi_wrap!(
        (|sigar: *mut sigar_t, netstat: *mut sigar_net_stat_t| sigar_net_stat_get(
            sigar,
            netstat,
            flags as c_int
        )),
        sigar_net_stat_t
    )?;

    Ok(Stat::from_raw(&raw))
}

// C: sigar_net_listen_address_get
/// Returns the bind address for a given port
pub fn listen_address_get(port: u64) -> SigarResult<NetAddress> {
    let raw = ffi_wrap!(
        sigar_net_listen_address_get,
        (port as c_ulong),
        sigar_net_address_t
    )?;

    Ok(NetAddress::from_raw(&raw))
}

// TODO:
// C: sigar_net_connection_walk
// C: sigar_net_stat_port_get
// C: sigar_net_address_equals
// C: sigar_net_address_to_string
// C: sigar_net_scope_to_string
// C: sigar_net_address_hash
// C: sigar_net_connection_type_get
// C: sigar_net_connection_state_get
// C: sigar_net_interface_flags_to_string
// C: sigar_net_services_name_get
