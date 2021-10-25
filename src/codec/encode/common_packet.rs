use crate::frame::CommonPacketItem;
use crate::Result;
use crate::{codec::Encodable, frame::CommonPacketFormat};
use bytes::{BufMut, BytesMut};

impl Encodable for CommonPacketFormat {
    #[inline(always)]
    fn encode(self: CommonPacketFormat, dst: &mut BytesMut) -> Result<()> {
        debug_assert!(self.len() > 0 && self.len() <= 4);
        dst.put_u16_le(self.len() as u16);
        for item in self.into_vec() {
            item.encode(dst)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        let count: usize = self.iter().map(|v| v.bytes_count()).sum();
        count + 2
    }
}

impl Encodable for CommonPacketItem {
    #[inline(always)]
    fn encode(self: CommonPacketItem, dst: &mut BytesMut) -> Result<()> {
        let bytes_count = self.bytes_count();
        dst.reserve(bytes_count);
        dst.put_u16_le(self.type_code);
        if let Some(data) = self.data {
            debug_assert!(data.len() <= u16::MAX as usize);
            dst.put_u16_le(data.len() as u16);
            dst.put_slice(&data);
        } else {
            dst.put_u16_le(0);
        }
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        4 + self.data.as_ref().map(|v| v.len()).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn test_common_packet_item() {
        let item = CommonPacketItem {
            type_code: 0x00,
            data: Some(Bytes::from_static(&[0, 0])),
        };
        assert_eq!(item.bytes_count(), 6);
        let buf = item.try_into_bytes().unwrap();
        assert_eq!(&buf[..], &[0x00, 0x00, 0x02, 0x00, 0x00, 0x00,]);
    }

    #[test]
    fn test_common_packet() {
        let null_addr = CommonPacketItem {
            type_code: 0x00,
            data: Some(Bytes::from_static(&[0, 0])),
        };
        let data_item = CommonPacketItem {
            type_code: 0xB2,
            data: Some(Bytes::from_static(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
            ])),
        };
        let cpf = CommonPacketFormat::from(vec![null_addr, data_item]);
        assert_eq!(cpf.bytes_count(), 2 + 4 + 2 + 4 + 9);
        let buf = cpf.try_into_bytes().unwrap();
        assert_eq!(
            &buf[..],
            &[
                0x02, 0x00, // item count
                0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // addr item
                0xB2, 0x00, 0x09, 0x00, 1, 2, 3, 4, 5, 6, 7, 8, 9, //data item
            ]
        );
    }
}