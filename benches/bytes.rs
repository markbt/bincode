use bincode::config;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bytes_encodings(c: &mut Criterion) {
    let config = config::standard().with_limit::<100000>();
    let data = b"Hello, world!\n".repeat(1000);
    let mut buffer = [0u8; 16384];

    c.bench_function("encode large bytes", |b| {
        b.iter(|| {
            let _ = black_box(bincode::encode_into_slice(black_box(&data), &mut buffer[..], config).unwrap());
        })
    });

    bincode::encode_into_slice(&data, &mut buffer[..], config).unwrap();
    c.bench_function("borrow decode large bytes", |b| {
        b.iter(|| {
            let _ : &[u8] = black_box(bincode::borrow_decode_from_slice(black_box(&buffer), config).unwrap().0);
        })
    });

    c.bench_function("decode large bytes", |b| {
        b.iter(|| {
            let _ : Vec<u8> = black_box(bincode::decode_from_slice(black_box(&buffer), config).unwrap().0);
        })
    });
}

criterion_group!(benches, bytes_encodings);
criterion_main!(benches);
