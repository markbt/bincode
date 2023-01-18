
use crate::error::EncodeError;
use crate::error::DecodeError;
use crate::enc::Encoder;
use crate::enc::Encode;
use crate::enc::write::Writer;
use crate::de::Decode;
use crate::de::BorrowDecode;
use crate::de::BorrowDecoder;
use crate::de::Decoder;
use crate::de::read::Reader;
use crate::size::EncodedSize;
use crate::config::Config;
use crate::config::InternalArrayLengthConfig;

impl<T> Encode for [T]
where
    T: Encode,
{
    default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl Encode for [u8] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        encoder.writer().write(self)?;
        Ok(())
    }
}

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if !E::C::SKIP_FIXED_ARRAY_LENGTH {
            crate::enc::encode_slice_len(encoder, N)?;
        }
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if !E::C::SKIP_FIXED_ARRAY_LENGTH {
            crate::enc::encode_slice_len(encoder, N)?;
        }
        encoder.writer().write(self.as_slice())?;
        Ok(())
    }
}

impl<T> EncodedSize for [T]
where
    T: EncodedSize,
{
    default fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = crate::size::size_slice_len::<C>(self.len())?;
        for item in self {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl EncodedSize for [u8] {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let size = crate::size::size_slice_len::<C>(self.len())?;
        Ok(size + self.len())
    }
}

impl<T, const N: usize> EncodedSize for [T; N]
where
    T: EncodedSize,
{
    default fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = 0;
        if !C::SKIP_FIXED_ARRAY_LENGTH {
            size += crate::size::size_slice_len::<C>(N)?;
        }
        for item in self.iter() {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl<const N: usize> EncodedSize for [u8; N] {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = N;
        if !C::SKIP_FIXED_ARRAY_LENGTH {
            size += crate::size::size_slice_len::<C>(N)?;
        }
        Ok(size)
    }
}

impl<T, const N: usize> Decode for [T; N]
where
    T: Decode,
{
    default fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        if !D::C::SKIP_FIXED_ARRAY_LENGTH {
            let length = crate::de::decode_slice_len(decoder)?;
            if length != N {
                return Err(DecodeError::ArrayLengthMismatch {
                    found: length,
                    required: N,
                });
            }
        }

        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;

        let result = crate::de::impl_core::collect_into_array(&mut (0..N).map(|_| {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            T::decode(decoder)
        }));

        // result is only None if N does not match the values of `(0..N)`, which it always should
        // So this unwrap should always work
        result.unwrap()
    }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        if !D::C::SKIP_FIXED_ARRAY_LENGTH {
            let length = crate::de::decode_slice_len(decoder)?;
            if length != N {
                return Err(DecodeError::ArrayLengthMismatch {
                    found: length,
                    required: N,
                });
            }
        }

        decoder.claim_bytes_read(N)?;

        let mut buf = [0u8; N];
        decoder.reader().read(&mut buf)?;
        Ok(buf)
    }
}

impl<'de, T, const N: usize> BorrowDecode<'de> for [T; N]
where
    T: BorrowDecode<'de>,
{
    default fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        if !D::C::SKIP_FIXED_ARRAY_LENGTH {
            let length = crate::de::decode_slice_len(decoder)?;
            if length != N {
                return Err(DecodeError::ArrayLengthMismatch {
                    found: length,
                    required: N,
                });
            }
        }

        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;

        let result = crate::de::impl_core::collect_into_array(&mut (0..N).map(|_| {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            T::borrow_decode(decoder)
        }));

        // result is only None if N does not match the values of `(0..N)`, which it always should
        // So this unwrap should always work
        result.unwrap()
    }
}

impl<'de, const N: usize> BorrowDecode<'de> for [u8; N] {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        if !D::C::SKIP_FIXED_ARRAY_LENGTH {
            let length = crate::de::decode_slice_len(decoder)?;
            if length != N {
                return Err(DecodeError::ArrayLengthMismatch {
                    found: length,
                    required: N,
                });
            }
        }

        decoder.claim_bytes_read(N)?;

        let mut buf = [0u8; N];
        decoder.reader().read(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use alloc::vec::Vec;
    use super::*;

    impl<T> Encode for Vec<T>
    where
        T: Encode,
    {
        default fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
            crate::enc::encode_slice_len(encoder, self.len())?;
            for item in self.iter() {
                item.encode(encoder)?;
            }
            Ok(())
        }
    }

    impl Encode for Vec<u8> {
        fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
            crate::enc::encode_slice_len(encoder, self.len())?;
            encoder.writer().write(self.as_slice())?;
            Ok(())
        }
    }

    impl<T> Decode for Vec<T>
    where
        T: Decode,
    {
        default fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                // See the documentation on `unclaim_bytes_read` as to why we're doing this here
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());

                vec.push(T::decode(decoder)?);
            }
            Ok(vec)
        }
    }

    impl Decode for Vec<u8> {
        fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_bytes_read(len)?;

            let mut vec = Vec::with_capacity(len);
            // Safety: we are writing into the unitialized capacity.
            // Replace with MaybeUninit::write_slice once stabilized.
            let buffer: &mut [u8] = unsafe { &mut *(vec.spare_capacity_mut() as *mut[_] as *mut [u8]) };
            decoder.reader().read(buffer)?;
            // Safety: we just initialize `len` bytes by reading.
            unsafe { vec.set_len(len); }
            Ok(vec)
        }
    }

    impl<'de, T> BorrowDecode<'de> for Vec<T>
    where
        T: BorrowDecode<'de>,
    {
        default fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_container_read::<T>(len)?;

            let mut vec = Vec::with_capacity(len);
            for _ in 0..len {
                // See the documentation on `unclaim_bytes_read` as to why we're doing this here
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());

                vec.push(T::borrow_decode(decoder)?);
            }
            Ok(vec)
        }
    }

    impl<'de> BorrowDecode<'de> for Vec<u8> {
        fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
            let len = crate::de::decode_slice_len(decoder)?;
            decoder.claim_bytes_read(len)?;

            let mut vec = Vec::with_capacity(len);
            // Safety: we are writing into the unitialized capacity.
            // Replace with MaybeUninit::write_slice once stabilized.
            let buffer: &mut [u8] = unsafe { &mut *(vec.spare_capacity_mut() as *mut[_] as *mut [u8]) };
            decoder.reader().read(buffer)?;
            // Safety: we just initialize `len` bytes by reading.
            unsafe { vec.set_len(len); }
            Ok(vec)
        }
    }

    impl<T> EncodedSize for Vec<T>
    where
        T: EncodedSize,
    {
        default fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
            let mut size = crate::size::size_slice_len::<C>(self.len())?;
            for item in self.iter() {
                size += item.encoded_size::<C>()?;
            }
            Ok(size)
        }
    }

    impl EncodedSize for Vec<u8> {
        fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
            let size = crate::size::size_slice_len::<C>(self.len())?;
            Ok(size + self.len())
        }
    }
}
