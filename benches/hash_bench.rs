use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use hashing::{hash_string, hash_bytes, Algorithm};

fn bench_string_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_hashing");
    
    let long_string = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(10);
    let inputs = vec![
        ("short", "hello"),
        ("medium", "The quick brown fox jumps over the lazy dog. Pack my box with five dozen liquor jugs."),
        ("long", long_string.as_str()),
    ];

    for (name, input) in inputs {
        group.throughput(Throughput::Bytes(input.len() as u64));
        
        group.bench_with_input(BenchmarkId::new("sha256", name), &input, |b, i| {
            b.iter(|| hash_string(black_box(i), Algorithm::Sha256))
        });
        
        group.bench_with_input(BenchmarkId::new("blake3", name), &input, |b, i| {
            b.iter(|| hash_string(black_box(i), Algorithm::Blake3))
        });
        
        group.bench_with_input(BenchmarkId::new("md5", name), &input, |b, i| {
            b.iter(|| hash_string(black_box(i), Algorithm::Md5))
        });
    }
    
    group.finish();
}

fn bench_bytes_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes_hashing");
    
    let sizes = vec![
        ("1KB", 1024),
        ("10KB", 10 * 1024),
        ("100KB", 100 * 1024),
    ];

    for (name, size) in sizes {
        let data = vec![0xAB; size];
        group.throughput(Throughput::Bytes(size as u64));
        
        group.bench_with_input(BenchmarkId::new("sha256", name), &data, |b, d| {
            b.iter(|| hash_bytes(black_box(d), Algorithm::Sha256))
        });
        
        group.bench_with_input(BenchmarkId::new("blake3", name), &data, |b, d| {
            b.iter(|| hash_bytes(black_box(d), Algorithm::Blake3))
        });
        
        group.bench_with_input(BenchmarkId::new("sha512", name), &data, |b, d| {
            b.iter(|| hash_bytes(black_box(d), Algorithm::Sha512))
        });
    }
    
    group.finish();
}

fn bench_all_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_algorithms");
    let data = b"benchmark test data";
    
    for algorithm in Algorithm::all() {
        group.bench_with_input(
            BenchmarkId::from_parameter(algorithm.name()),
            &data,
            |b, d| {
                b.iter(|| hash_bytes(black_box(*d), algorithm))
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_string_hashing, bench_bytes_hashing, bench_all_algorithms);
criterion_main!(benches);
