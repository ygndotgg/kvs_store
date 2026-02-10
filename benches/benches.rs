use criterion::{Criterion, criterion_group, criterion_main};
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use tempfile::TempDir;

fn kvs_write(c: &mut Criterion) {
    c.bench_function("kvs_write", |b| {
        b.iter(|| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();

            for i in 0..100 {
                store
                    .set(format!("key{}", i), format!("value{}", i))
                    .unwrap();
            }
        });
    });
}

fn sled_write(c: &mut Criterion) {
    c.bench_function("sled_function", |b| {
        b.iter(|| {
            let temp = TempDir::new().unwrap();
            let mut store = SledKvsEngine::open(temp.path()).unwrap();
            for i in 0..100 {
                store
                    .set(format!("key{}", i), format!("value{}", i))
                    .unwrap();
            }
        });
    });
}

fn kvs_read(c: &mut Criterion) {
    let temp = TempDir::new().unwrap();
    let mut store = KvStore::open(temp.path()).unwrap();
    for i in 0..1000 {
        store
            .set(format!("key{}", i), format!("value{}", i))
            .unwrap();
    }
    c.bench_function("kv_read_bench", |b| {
        b.iter(|| {
            for i in 0..1000 {
                store.get(format!("key{}", i)).unwrap();
            }
        });
    });
}

fn sled_read(c: &mut Criterion) {
    let temp = TempDir::new().unwrap();
    let mut store = SledKvsEngine::open(temp.path()).unwrap();
    for i in 0..1000 {
        store
            .set(format!("key{}", i), format!("value{}", i))
            .unwrap();
    }
    c.bench_function("sled_read_bench", |b| {
        b.iter(|| {
            for i in 0..1000 {
                store.get(format!("key{}", i)).unwrap();
            }
        });
    });
}

criterion_group!(benches, kvs_write, sled_write, kvs_read, sled_read);
criterion_main!(benches);
