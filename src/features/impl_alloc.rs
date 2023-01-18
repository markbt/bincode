use crate::{
    de::{BorrowDecoder, Decode, Decoder},
    enc::{self, Encode, Encoder},
    error::{DecodeError, EncodeError},
    impl_borrow_decode,
    size::EncodedSize,
    BorrowDecode, Config,
};
#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::*,
    rc::Rc,
    string::String,
    vec::Vec,
};

#[derive(Default)]
pub(crate) struct VecWriter {
    inner: Vec<u8>,
}

impl VecWriter {
    // May not be used in all feature combinations
    #[allow(dead_code)]
    pub(crate) fn collect(self) -> Vec<u8> {
        self.inner
    }
}

impl enc::write::Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }
}

/// Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode_to_vec<E: enc::Encode, C: Config>(val: E, config: C) -> Result<Vec<u8>, EncodeError> {
    let writer = VecWriter::default();
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().inner)
}

impl<T> Decode for BinaryHeap<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::decode(decoder)?;
            map.push(key);
        }
        Ok(map)
    }
}
impl<'de, T> BorrowDecode<'de> for BinaryHeap<T>
where
    T: BorrowDecode<'de> + Ord,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::borrow_decode(decoder)?;
            map.push(key);
        }
        Ok(map)
    }
}

impl<T> Encode for BinaryHeap<T>
where
    T: Encode + Ord,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for val in self.iter() {
            val.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> EncodedSize for BinaryHeap<T>
where
    T: EncodedSize + Ord,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = crate::size::size_slice_len::<C>(self.len())?;
        for val in self.iter() {
            size += val.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;

        let mut map = BTreeMap::new();
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());

            let key = K::decode(decoder)?;
            let value = V::decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}
impl<'de, K, V> BorrowDecode<'de> for BTreeMap<K, V>
where
    K: BorrowDecode<'de> + Ord,
    V: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;

        let mut map = BTreeMap::new();
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());

            let key = K::borrow_decode(decoder)?;
            let value = V::borrow_decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode + Ord,
    V: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for (key, val) in self.iter() {
            key.encode(encoder)?;
            val.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K, V> EncodedSize for BTreeMap<K, V>
where
    K: EncodedSize + Ord,
    V: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = crate::size::size_slice_len::<C>(self.len())?;
        for (key, val) in self.iter() {
            size += key.encoded_size::<C>()?;
            size += val.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = BTreeSet::new();
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}
impl<'de, T> BorrowDecode<'de> for BTreeSet<T>
where
    T: BorrowDecode<'de> + Ord,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = BTreeSet::new();
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::borrow_decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}

impl<T> Encode for BTreeSet<T>
where
    T: Encode + Ord,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> EncodedSize for BTreeSet<T>
where
    T: EncodedSize + Ord,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = crate::size::size_slice_len::<C>(self.len())?;
        for item in self.iter() {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl<T> Decode for VecDeque<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = VecDeque::with_capacity(len);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::decode(decoder)?;
            map.push_back(key);
        }
        Ok(map)
    }
}
impl<'de, T> BorrowDecode<'de> for VecDeque<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        let mut map = VecDeque::with_capacity(len);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());

            let key = T::borrow_decode(decoder)?;
            map.push_back(key);
        }
        Ok(map)
    }
}

impl<T> Encode for VecDeque<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> EncodedSize for VecDeque<T>
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = crate::size::size_slice_len::<C>(self.len())?;
        for item in self.iter() {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

#[cfg(not(feature = "use_min_specialization"))]
impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
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

#[cfg(not(feature = "use_min_specialization"))]
impl<'de, T> BorrowDecode<'de> for Vec<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
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

#[cfg(not(feature = "use_min_specialization"))]
impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

#[cfg(not(feature = "use_min_specialization"))]
impl<T> EncodedSize for Vec<T>
where
    T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        let mut size = crate::size::size_slice_len::<C>(self.len())?;
        for item in self.iter() {
            size += item.encoded_size::<C>()?;
        }
        Ok(size)
    }
}

impl Decode for String {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bytes = Vec::<u8>::decode(decoder)?;
        String::from_utf8(bytes).map_err(|e| DecodeError::Utf8 {
            inner: e.utf8_error(),
        })
    }
}
impl_borrow_decode!(String);

impl Decode for Box<str> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        String::decode(decoder).map(String::into_boxed_str)
    }
}
impl_borrow_decode!(Box<str>);

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl EncodedSize for String {
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.as_bytes().encoded_size::<C>()
    }
}

impl<T> Decode for Box<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Box::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for Box<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Box::new(t))
    }
}

impl<T> Encode for Box<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

impl<T> EncodedSize for Box<T>
where
    T: EncodedSize + ?Sized,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        T::encoded_size::<C>(self)
    }
}

impl<T> Decode for Box<[T]>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}

impl<'de, T> BorrowDecode<'de> for Box<[T]>
where
    T: BorrowDecode<'de> + 'de,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::borrow_decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}

impl<'cow, T> Decode for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = <T as ToOwned>::Owned::decode(decoder)?;
        Ok(Cow::Owned(t))
    }
}
impl<'cow, T> BorrowDecode<'cow> for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    &'cow T: BorrowDecode<'cow>,
{
    fn borrow_decode<D: BorrowDecoder<'cow>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = <&T>::borrow_decode(decoder)?;
        Ok(Cow::Borrowed(t))
    }
}

impl<'cow, T> Encode for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_ref().encode(encoder)
    }
}

impl<'cow, T> EncodedSize for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: EncodedSize,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        self.as_ref().encoded_size::<C>()
    }
}

impl<T> Decode for Rc<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Rc::new(t))
    }
}

impl<'de, T> BorrowDecode<'de> for Rc<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Rc::new(t))
    }
}

impl<T> Encode for Rc<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

impl<T> EncodedSize for Rc<T>
where
    T: EncodedSize + ?Sized,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        T::encoded_size::<C>(self)
    }
}

impl<T> Decode for Rc<[T]>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(vec.into())
    }
}

impl<'de, T> BorrowDecode<'de> for Rc<[T]>
where
    T: BorrowDecode<'de> + 'de,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::borrow_decode(decoder)?;
        Ok(vec.into())
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<T> Decode for Arc<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Arc::new(t))
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Decode for Arc<str> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let decoded = String::decode(decoder)?;
        Ok(decoded.into())
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<'de, T> BorrowDecode<'de> for Arc<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Arc::new(t))
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<'de> BorrowDecode<'de> for Arc<str> {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let decoded = String::decode(decoder)?;
        Ok(decoded.into())
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<T> Encode for Arc<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<T> EncodedSize for Arc<T>
where
    T: EncodedSize + ?Sized,
{
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError> {
        T::encoded_size::<C>(self)
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<T> Decode for Arc<[T]>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(vec.into())
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<'de, T> BorrowDecode<'de> for Arc<[T]>
where
    T: BorrowDecode<'de> + 'de,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::<T>::borrow_decode(decoder)?;
        Ok(vec.into())
    }
}
