use criterion::{criterion_group, criterion_main, Criterion};
use uvoxid::{encode_uvoxid, decode_uvoxid};
use uvoxid::utils::distance::{linear_distance, haversine_distance};

fn bench_encode(c: &mut Criterion) {
    c.bench_function("encode_uvoxid", |b| {
        b.iter(|| {
            encode_uvoxid(6_371_000_000_000, 25_760_000, -80_190_000);
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let id = encode_uvoxid(6_371_000_000_000, 25_760_000, -80_190_000);
    c.bench_function("decode_uvoxid", |b| {
        b.iter(|| {
            decode_uvoxid(&id);
        })
    });
}

fn bench_linear_distance(c: &mut Criterion) {
    let id1 = encode_uvoxid(6_371_000_000_000, 25_760_000, -80_190_000);
    let id2 = encode_uvoxid(6_371_000_000_000, 40_710_000, -74_010_000);

    c.bench_function("linear_distance", |b| {
        b.iter(|| {
            linear_distance(&id1, &id2);
        })
    });
}

fn bench_haversine_distance(c: &mut Criterion) {
    let id1 = encode_uvoxid(6_371_000_000_000, 25_760_000, -80_190_000);
    let id2 = encode_uvoxid(6_371_000_000_000, 40_710_000, -74_010_000);

    c.bench_function("haversine_distance", |b| {
        b.iter(|| {
            haversine_distance(&id1, &id2);
        })
    });
}

// Register all benchmarks
criterion_group!(
    benches,
    bench_encode,
    bench_decode,
    bench_linear_distance,
    bench_haversine_distance
);
criterion_main!(benches);
