use criterion::{ criterion_main, criterion_group, Criterion, black_box };
use gimli_hash::GimliHash;



fn bench_gimli_hash(c: &mut Criterion) {
    c.bench_function("16", |b| {
        let data = black_box([0x25; 16]);

        b.iter(|| {
            let mut res = [0; 32];
            let mut hasher = GimliHash::default();
            hasher.update(&data);
            hasher.finalize(&mut res);
        });
    });

    c.bench_function("256", |b| {
        let data = black_box([0x26; 256]);

        b.iter(|| {
            let mut res = [0; 32];
            let mut hasher = GimliHash::default();
            hasher.update(&data);
            hasher.finalize(&mut res);
        });
    });

    c.bench_function("4096", |b| {
        let data = black_box([0x28; 4096]);

        b.iter(|| {
            let mut res = [0; 32];
            let mut hasher = GimliHash::default();
            hasher.update(&data);
            hasher.finalize(&mut res);
        });
    });

    c.bench_function("16k", |b| {
        let data = black_box(vec![0x28; 16 * 1024]);

        b.iter(|| {
            let mut res = [0; 32];
            let mut hasher = GimliHash::default();
            hasher.update(&data);
            hasher.finalize(&mut res);
        });
    });
}

criterion_group!(hash, bench_gimli_hash);
criterion_main!(hash);
