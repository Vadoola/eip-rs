// rseip
//
// rseip - EIP&CIP in pure Rust.
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

/// default port for EtherNet/IP over TCP/IP
pub const EIP_DEFAULT_PORT: u16 = 0xAF12;
/// default port for EtherNet/IP over TCP/IP class 0 and class 1
pub const EIP_DEFAULT_UDP_PORT: u16 = 0x08AE;

pub const ENCAPSULATION_HEADER_LEN: usize = 24;
pub const ENCAPSULATION_DATA_MAX_LEN: usize = u16::MAX as usize - ENCAPSULATION_HEADER_LEN;
pub const COMMON_PACKET_MAX_ITEM_COUNT: u16 = 4;

pub const EIP_COMMAND_NOP: u16 = 0x0000;
pub const EIP_COMMAND_LIST_IDENTITY: u16 = 0x0063;
pub const EIP_COMMAND_LIST_INTERFACES: u16 = 0x0064;
pub const EIP_COMMAND_LIST_SERVICE: u16 = 0x0004;
pub const EIP_COMMAND_REGISTER_SESSION: u16 = 0x0065;
pub const EIP_COMMAND_UNREGISTER_SESSION: u16 = 0x0066;
pub const EIP_COMMAND_SEND_RRDATA: u16 = 0x006F;
pub const EIP_COMMAND_SEND_UNIT_DATA: u16 = 0x0070;

pub const SERVICE_UNCONNECTED_SEND: u8 = 0x52;
pub const SERVICE_FORWARD_OPEN: u8 = 0x54;
pub const SERVICE_LARGE_FORWARD_OPEN: u8 = 0x5B;
pub const SERVICE_FORWARD_CLOSE: u8 = 0x4E;
