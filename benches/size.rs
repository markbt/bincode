use bincode::config;
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::distributions::Distribution;

use bincode::{Encode,config::Config,enc::EncoderImpl,error::EncodeError,enc::write::Writer};

/** Writer which only counts the bytes "written" to it */
struct SizeOnlyWriter<'a> {
    bytes_written: &'a mut usize
}

impl<'a> Writer for SizeOnlyWriter<'a> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        *self.bytes_written += bytes.len();
        Ok(())
    }
}

/** Return the serialized size of an `Encode` object. */
pub fn serialized_size<T:Encode,C:Config>(obj:&T, config:C) -> Result<usize, EncodeError> {
    let mut size = 0usize;
    let writer = SizeOnlyWriter { bytes_written: &mut size };
    let mut ei = EncoderImpl::new(writer, config);
    obj.encode(&mut ei)?;
    Ok(size)
}

macro_rules! size_bench {
    ($name:ident, $t:ident, $f:path) => {
        fn $name(c: &mut Criterion) {
            let mut rng = rand::thread_rng();
            let dist = rand::distributions::Uniform::from(0..$t::MAX);
            let input: Vec<$t> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
                .take(10_000)
                .collect();
            let config = config::standard();

            c.bench_function(stringify!($name), |b| {
                b.iter(|| {
                    let _ = black_box($f(&input, config).unwrap());
                })
            });
        }
    }
}

size_bench!(size_localsw_vec_u8, u8, serialized_size);
size_bench!(size_localsw_vec_u16, u16, serialized_size);
size_bench!(size_localsw_vec_u32, u32, serialized_size);
size_bench!(size_localsw_vec_u64, u64, serialized_size);
size_bench!(size_libsw_vec_u8, u8, bincode::serialized_size);
size_bench!(size_libsw_vec_u16, u16, bincode::serialized_size);
size_bench!(size_libsw_vec_u32, u32, bincode::serialized_size);
size_bench!(size_libsw_vec_u64, u64, bincode::serialized_size);
size_bench!(size_libtrait_vec_u8, u8, bincode::encoded_size);
size_bench!(size_libtrait_vec_u16, u16, bincode::encoded_size);
size_bench!(size_libtrait_vec_u32, u32, bincode::encoded_size);
size_bench!(size_libtrait_vec_u64, u64, bincode::encoded_size);

criterion_group!(
    size_benches,
    size_localsw_vec_u8,
    size_localsw_vec_u16,
    size_localsw_vec_u32,
    size_localsw_vec_u64,
    size_libsw_vec_u8,
    size_libsw_vec_u16,
    size_libsw_vec_u32,
    size_libsw_vec_u64,
    size_libtrait_vec_u8,
    size_libtrait_vec_u16,
    size_libtrait_vec_u32,
    size_libtrait_vec_u64,
);
criterion_main!(size_benches);
