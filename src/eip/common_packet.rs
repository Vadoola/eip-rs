// rseip
//
// rseip - EIP&CIP in pure Rust.
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

use crate::{eip::EipError, Error, Result};
use bytes::Bytes;
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};
#[derive(Default, Debug)]
pub struct CommonPacket(SmallVec<[CommonPacketItem; 2]>);

impl CommonPacket {
    #[inline(always)]
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[inline(always)]
    pub fn into_inner(self) -> SmallVec<[CommonPacketItem; 2]> {
        self.0
    }

    #[inline(always)]
    pub fn into_iter(self) -> impl IntoIterator<Item = CommonPacketItem> {
        self.0.into_iter()
    }

    #[inline(always)]
    pub fn push(&mut self, item: CommonPacketItem) {
        self.0.push(item);
    }
}

impl Deref for CommonPacket {
    type Target = [CommonPacketItem];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommonPacket {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<CommonPacketItem>> for CommonPacket {
    #[inline(always)]
    fn from(src: Vec<CommonPacketItem>) -> Self {
        Self(SmallVec::from_vec(src))
    }
}

// type_code:u16 | item_length:u16 | item_data

#[derive(Debug)]
pub struct CommonPacketItem {
    pub type_code: u16,
    pub data: Bytes,
}

impl CommonPacketItem {
    #[inline(always)]
    pub fn with_null_addr() -> Self {
        Self {
            type_code: 0,
            data: Bytes::from_static(&[0x00, 0x00]),
        }
    }

    #[inline(always)]
    pub fn with_unconnected_data(data: Bytes) -> Self {
        Self {
            type_code: 0xB2,
            data: data,
        }
    }

    #[inline(always)]
    pub fn with_connected_data(data: Bytes) -> Self {
        Self {
            type_code: 0xB1,
            data: data,
        }
    }

    #[inline(always)]
    pub fn is_null_addr(&self) -> bool {
        if self.type_code != 0 {
            return false;
        }
        self.data.len() == 0
    }

    #[inline(always)]
    pub fn ensure_type_code(&self, type_code: u16) -> Result<()> {
        if self.type_code != type_code {
            return Err(Error::Eip(EipError::InvalidData));
        }
        Ok(())
    }
}
