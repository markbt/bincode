use super::{SIGNED_SINGLE_BYTE_MAX, SIGNED_SINGLE_BYTE_MIN};

macro_rules! range {
    ($t:ty : $min:expr => $max:expr) => {
        (($min as $t)..=($max as $t))
    };
}

pub fn varint_size_i16(val: i16) -> usize {
    if range!(i16: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX).contains(&val) {
        1
    } else {
        1 + std::mem::size_of::<u16>()
    }
}

pub fn varint_size_i32(val: i32) -> usize {
    if range!(i32: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX).contains(&val) {
        1
    } else if range!(i32: i16::MIN => i16::MAX).contains(&val) {
        1 + std::mem::size_of::<u16>()
    } else {
        1 + std::mem::size_of::<u32>()
    }
}

pub fn varint_size_i64(val: i64) -> usize {
    if range!(i64: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX).contains(&val) {
        1
    } else if range!(i64: i16::MIN => i16::MAX).contains(&val) {
        1 + std::mem::size_of::<u16>()
    } else if range!(i64: i32::MIN => i32::MAX).contains(&val) {
        1 + std::mem::size_of::<u32>()
    } else {
        1 + std::mem::size_of::<u64>()
    }
}

pub fn varint_size_i128(val: i128) -> usize {
    if range!(i128: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX).contains(&val) {
        1
    } else if range!(i128: i16::MIN => i16::MAX).contains(&val) {
        1 + std::mem::size_of::<u16>()
    } else if range!(i128: i32::MIN => i32::MAX).contains(&val) {
        1 + std::mem::size_of::<u32>()
    } else if range!(i128: i64::MIN => i64::MAX).contains(&val) {
        1 + std::mem::size_of::<u64>()
    } else {
        1 + std::mem::size_of::<u128>()
    }
}

pub fn varint_size_isize(val: isize) -> usize {
    // isize is being encoded as a i64
    varint_size_i64(val as i64)
}

#[test]
fn test_size_i16() {
    // these should all encode to a single byte
    for i in range!(i16: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX) {
        assert_eq!(varint_size_i16(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i16::MIN,
        -1000,
        -200,
        SIGNED_SINGLE_BYTE_MIN as i16 - 1,
        SIGNED_SINGLE_BYTE_MAX as i16 + 1,
        222,
        1234,
        i16::MAX,
    ] {
        assert_eq!(varint_size_i16(i), 3, "value: {}", i);
    }
}

#[test]
fn test_size_i32() {
    // these should all encode to a single byte
    for i in range!(i32: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX) {
        assert_eq!(varint_size_i32(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i16::MIN as i32,
        -1000,
        -200,
        SIGNED_SINGLE_BYTE_MIN as i32 - 1,
        SIGNED_SINGLE_BYTE_MAX as i32 + 1,
        222,
        1234,
        i16::MAX as i32,
    ] {
        assert_eq!(varint_size_i32(i), 3, "value: {}", i);
    }

    // these values should encode in 5 bytes (leading byte + 4 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i32::MIN,
        -1_000_000,
        i16::MIN as i32 - 1,
        i16::MAX as i32 + 1,
        100_000,
        1_000_000,
        i32::MAX,
    ] {
        assert_eq!(varint_size_i32(i), 5, "value: {}", i);
    }
}

#[test]
fn test_size_i64() {
    // these should all encode to a single byte
    for i in range!(i64: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX) {
        assert_eq!(varint_size_i64(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i16::MIN as i64,
        -1000,
        -200,
        SIGNED_SINGLE_BYTE_MIN as i64 - 1,
        SIGNED_SINGLE_BYTE_MAX as i64 + 1,
        222,
        1234,
        i16::MAX as i64,
    ] {
        assert_eq!(varint_size_i64(i), 3, "value: {}", i);
    }

    // these values should encode in 5 bytes (leading byte + 4 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i32::MIN as i64,
        -1_000_000,
        i16::MIN as i64 - 1,
        i16::MAX as i64 + 1,
        100_000,
        1_000_000,
        i32::MAX as i64,
    ] {
        assert_eq!(varint_size_i64(i), 5, "value: {}", i);
    }

    // these values should encode in 9 bytes (leading byte + 8 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i64::MIN,
        -6_000_000_000,
        i32::MIN as i64 - 1,
        i32::MAX as i64 + 1,
        5_000_000_000,
        i64::MAX,
    ] {
        assert_eq!(varint_size_i64(i), 9, "value: {}", i);
    }
}

#[test]
fn test_size_i128() {
    // these should all encode to a single byte
    for i in range!(i128: SIGNED_SINGLE_BYTE_MIN => SIGNED_SINGLE_BYTE_MAX) {
        assert_eq!(varint_size_i128(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i16::MIN as i128,
        -1000,
        -200,
        SIGNED_SINGLE_BYTE_MIN as i128 - 1,
        SIGNED_SINGLE_BYTE_MAX as i128 + 1,
        222,
        1234,
        i16::MAX as i128,
    ] {
        assert_eq!(varint_size_i128(i), 3, "value: {}", i);
    }

    // these values should encode in 5 bytes (leading byte + 4 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i32::MIN as i128,
        -1_000_000,
        i16::MIN as i128 - 1,
        i16::MAX as i128 + 1,
        100_000,
        1_000_000,
        i32::MAX as i128,
    ] {
        assert_eq!(varint_size_i128(i), 5, "value: {}", i);
    }

    // these values should encode in 9 bytes (leading byte + 8 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i64::MIN as i128,
        -6_000_000_000,
        i32::MIN as i128 - 1,
        i32::MAX as i128 + 1,
        5_000_000_000,
        i64::MAX as i128,
    ] {
        assert_eq!(varint_size_i128(i), 9, "value: {}", i);
    }

    // these values should encode in 17 bytes (leading byte + 16 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        i128::MIN,
        i64::MIN as i128 - 1,
        i64::MAX as i128 + 1,
        i128::MAX,
    ] {
        assert_eq!(varint_size_i128(i), 17, "value: {}", i);
    }
}
