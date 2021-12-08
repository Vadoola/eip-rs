// rseip
//
// rseip - EIP&CIP in pure Rust.
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

#![allow(non_snake_case)]

use super::*;
use bytes::{BufMut, Bytes, BytesMut};
use core::marker::PhantomData;
use smallvec::SmallVec;

impl<A: Encoder> Encoder for &mut A {
    type Error = A::Error;

    fn encode_bool(&mut self, item: bool, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_bool(item, buf)
    }

    fn encode_i8(&mut self, item: i8, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_i8(item, buf)
    }

    fn encode_u8(&mut self, item: u8, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_u8(item, buf)
    }

    fn encode_i16(&mut self, item: i16, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_i16(item, buf)
    }

    fn encode_u16(&mut self, item: u16, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_u16(item, buf)
    }

    fn encode_i32(&mut self, item: i32, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_i32(item, buf)
    }

    fn encode_u32(&mut self, item: u32, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_u32(item, buf)
    }

    fn encode_i64(&mut self, item: i64, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_i64(item, buf)
    }

    fn encode_u64(&mut self, item: u64, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_u64(item, buf)
    }

    fn encode_f32(&mut self, item: f32, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_f32(item, buf)
    }

    fn encode_f64(&mut self, item: f64, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_f64(item, buf)
    }

    fn encode_i128(&mut self, item: i128, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_i128(item, buf)
    }

    fn encode_u128(&mut self, item: u128, buf: &mut BytesMut) -> Result<(), Self::Error> {
        (**self).encode_u128(item, buf)
    }
}

macro_rules! impl_primitive {
    ($ty:ident, $m:tt, $s:tt) => {
        impl Encode for $ty {
            #[inline]
            fn encode<A: Encoder>(self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
            where
                Self: Sized,
            {
                encoder.$m(self, buf)
            }

            #[inline]
            fn encode_by_ref<A: Encoder>(
                &self,
                buf: &mut BytesMut,
                encoder: &mut A,
            ) -> Result<(), A::Error> {
                encoder.$m(*self, buf)
            }

            #[inline(always)]
            fn bytes_count(&self) -> usize {
                $s
            }
        }
    };
}

impl_primitive!(bool, encode_bool, 1);
impl_primitive!(i8, encode_i8, 1);
impl_primitive!(u8, encode_u8, 1);
impl_primitive!(i16, encode_i16, 2);
impl_primitive!(u16, encode_u16, 2);
impl_primitive!(i32, encode_i32, 4);
impl_primitive!(u32, encode_u32, 4);
impl_primitive!(i64, encode_i64, 8);
impl_primitive!(u64, encode_u64, 8);
impl_primitive!(i128, encode_i128, 16);
impl_primitive!(u128, encode_u128, 16);

impl<T: Encode> Encode for &T {
    #[inline]
    fn encode_by_ref<A: Encoder>(&self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        (**self).encode_by_ref(buf, encoder)
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        (**self).bytes_count()
    }
}

impl Encode for () {
    #[inline]
    fn encode_by_ref<A: Encoder>(
        &self,
        _buf: &mut BytesMut,
        _encoder: &mut A,
    ) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        0
    }
}

impl<T: Encode> Encode for PhantomData<T> {
    #[inline]
    fn encode_by_ref<A: Encoder>(
        &self,
        _buf: &mut BytesMut,
        _encoder: &mut A,
    ) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        0
    }
}

impl<T: Encode> Encode for Option<T> {
    #[inline]
    fn encode<A: Encoder>(self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        if let Some(item) = self {
            item.encode(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn encode_by_ref<A: Encoder>(&self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        if let Some(item) = self {
            item.encode_by_ref(buf, encoder)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        self.as_ref().map(|v| v.bytes_count()).unwrap_or_default()
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    #[inline]
    fn encode<A: Encoder>(self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        for item in self {
            item.encode(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn encode_by_ref<A: Encoder>(&self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        for item in self {
            item.encode_by_ref(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn bytes_count(&self) -> usize {
        self.iter().map(|v| v.bytes_count()).sum()
    }
}

impl<T> Encode for SmallVec<T>
where
    T: smallvec::Array,
    T::Item: Encode,
{
    #[inline]
    fn encode<A: Encoder>(self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        for item in self {
            item.encode(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn encode_by_ref<A: Encoder>(&self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        for item in self {
            item.encode_by_ref(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn bytes_count(&self) -> usize {
        self.iter().map(|v| v.bytes_count()).sum()
    }
}

impl<T: Encode> Encode for Vec<T> {
    #[inline]
    fn encode<A: Encoder>(self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        for item in self {
            item.encode(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn encode_by_ref<A: Encoder>(&self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
    where
        Self: Sized,
    {
        for item in self {
            item.encode_by_ref(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn bytes_count(&self) -> usize {
        self.iter().map(|v| v.bytes_count()).sum()
    }
}

impl Encode for Bytes {
    #[inline]
    fn encode_by_ref<A: Encoder>(
        &self,
        buf: &mut BytesMut,
        _encoder: &mut A,
    ) -> Result<(), A::Error> {
        buf.put_slice(self);
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        self.len()
    }
}

impl Encode for &[u8] {
    #[inline]
    fn encode_by_ref<A: Encoder>(
        &self,
        buf: &mut BytesMut,
        _encoder: &mut A,
    ) -> Result<(), A::Error> {
        buf.put_slice(self);
        Ok(())
    }

    #[inline(always)]
    fn bytes_count(&self) -> usize {
        self.len()
    }
}

pub struct SliceContainer<'a, T> {
    inner: &'a [T],
    bytes_count: Option<usize>,
}

impl<'a, T> SliceContainer<'a, T> {
    #[inline]
    pub fn new(inner: &'a [T]) -> Self {
        Self {
            inner,
            bytes_count: None,
        }
    }

    /// fast path to compute number of bytes
    #[inline]
    pub fn with_bytes_count(mut self, size: usize) -> Self {
        self.bytes_count = Some(size);
        self
    }
}

impl<'a, T> Encode for SliceContainer<'a, T>
where
    T: Encode,
{
    #[inline]
    fn encode_by_ref<A: Encoder>(
        &self,
        buf: &mut BytesMut,
        encoder: &mut A,
    ) -> Result<(), A::Error> {
        for item in self.inner.iter() {
            item.encode_by_ref(buf, encoder)?;
        }
        Ok(())
    }

    #[inline]
    fn bytes_count(&self) -> usize {
        if let Some(v) = self.bytes_count {
            v
        } else {
            self.inner.iter().map(|v| v.bytes_count()).sum()
        }
    }
}

macro_rules! impl_tuple {
    ($($n:tt $name:ident)+) => {
        impl<$($name,)+> Encode for ($($name,)+)
        where
            $($name:Encode,)+
        {
            #[inline]
            fn encode<A: Encoder>(self, buf: &mut BytesMut, encoder: &mut A) -> Result<(), A::Error>
            where
                Self: Sized,
            {
                $(
                    self.$n.encode(buf, encoder)?;
                )+
                Ok(())
            }

            #[inline]
            fn encode_by_ref<A: Encoder>(
                &self,
                buf: &mut BytesMut,
                encoder: &mut A,
            ) -> Result<(), A::Error>
            where
                Self: Sized,
            {
                $(
                    self.$n.encode_by_ref(buf, encoder)?;
                )+
                Ok(())
            }

            #[inline]
            fn bytes_count(&self) -> usize {
                let mut count = 0;
                $(
                    count += self.$n.bytes_count();
                )+
                count
            }
        }
    };
}

impl_tuple!(0 T0);
impl_tuple!(0 T0 1 T1);
impl_tuple!(0 T0 1 T1 2 T2);
impl_tuple!(0 T0 1 T1 2 T2 3 T3);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14);
impl_tuple!(0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15);