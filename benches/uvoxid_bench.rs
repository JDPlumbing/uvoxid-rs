use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uvoxid::{UvoxId, Delta};

pub fn bench_serde(c: &mut Criterion) {
    let id = UvoxId::new(0, 6_371_000_000, 12_345_678, -98_765_432);

    c.bench_function("serialize json", |b| {
        b.iter(|| serde_json::to_string(black_box(&id)).unwrap())
    });

    let json = serde_json::to_string(&id).unwrap();
    c.bench_function("deserialize json", |b| {
        b.iter(|| serde_json::from_str::<UvoxId>(black_box(&json)).unwrap())
    });

    c.bench_function("serialize cbor", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            ciborium::ser::into_writer(black_box(&id), &mut buf).unwrap();
            buf
        })
    });

    let mut buf = Vec::new();
    ciborium::ser::into_writer(&id, &mut buf).unwrap();
    c.bench_function("deserialize cbor", |b| {
        b.iter(|| ciborium::de::from_reader::<UvoxId, _>(black_box(buf.as_slice())).unwrap())
    });
}
pub fn bench_uvoxid(c: &mut Criterion) {
    let id = UvoxId::new(0, 6_371_000_000, 0, 0);
    let delta = Delta { dr_um: 100, dlat: 50, dlon: -50 };

    c.bench_function("construct uvoxid", |b| {
        b.iter(|| UvoxId::new(0, black_box(6_371_000_000), 0, 0))
    });

    c.bench_function("apply delta", |b| {
        b.iter(|| {
            let id2 = id + delta;
            black_box(id2)
        })
    });
}

criterion_group!(benches, bench_serde, bench_uvoxid);
criterion_main!(benches);
