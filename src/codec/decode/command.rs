use crate::{
    error::{Error, ResponseError},
    frame::{
        command_reply::{ListIdentityReply, RegisterSessionReply},
        CommonPacketFormat, EncapsulationPacket,
    },
    objects::identity::IdentityObject,
};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use std::{convert::TryFrom, io};

impl TryFrom<EncapsulationPacket<Bytes>> for RegisterSessionReply {
    type Error = Error;
    #[inline]
    fn try_from(src: EncapsulationPacket<Bytes>) -> Result<Self, Self::Error> {
        if src.hdr.command != 0x65 {
            return Err(
                io::Error::new(io::ErrorKind::Other, "RegisterSession: unexpected reply").into(),
            );
        }
        let session_handle = src.hdr.session_handle;
        let reply_data = src.data;

        //RegisterSession
        if reply_data.len() < 4 {
            return Err(Error::Response(ResponseError::InvalidData));
        }
        debug_assert_eq!(reply_data.len(), 4);
        //TODO: validate sender context
        let protocol_version = LittleEndian::read_u16(&reply_data[0..2]);
        debug_assert_eq!(protocol_version, 1);
        let session_options = LittleEndian::read_u16(&reply_data[2..4]);
        debug_assert_eq!(session_options, 0);
        Ok(Self {
            session_handle: session_handle,
            protocol_version,
        })
    }
}

impl TryFrom<EncapsulationPacket<Bytes>> for ListIdentityReply {
    type Error = Error;
    #[inline]
    fn try_from(src: EncapsulationPacket<Bytes>) -> Result<Self, Self::Error> {
        if src.hdr.command != 0x63 {
            return Err(
                io::Error::new(io::ErrorKind::Other, "ListIdentity: unexpected reply").into(),
            );
        }
        let cpf = CommonPacketFormat::try_from(src.data)?;
        if cpf.len() != 1 {
            return Err(Error::Response(ResponseError::InvalidData));
        }
        // ListIdentity
        let res: Result<Vec<_>, _> = cpf
            .into_vec()
            .into_iter()
            .map(|item| {
                if item.type_code != 0x0C {
                    return Err(Error::Response(ResponseError::InvalidData));
                }
                let item_data = item.data.unwrap();
                IdentityObject::try_from(item_data)
            })
            .collect();
        Ok(Self(res?))
    }
}