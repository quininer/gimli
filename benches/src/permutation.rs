use criterion::{ criterion_main, criterion_group, Criterion, black_box };
use gimli_permutation::SIZE;



fn bench_gimli(c: &mut Criterion) {
    c.bench_function("gimli", |b| {
        use gimli_permutation::gimli;

        let mut data = black_box([40; SIZE]);

        b.iter(|| {
            gimli(&mut data);
        });
    });

    c.bench_function("portable", |b| {
        use gimli_permutation::portable;

        let mut data = black_box([40; SIZE]);

        b.iter(|| {
            portable::gimli(&mut data);
        });
    });

    #[cfg(gimli_test)]
    #[cfg(feature = "simd")]
    #[cfg(target_feature = "ssse3")]
    c.bench_function("ssse3", |b| {
        use gimli_permutation::test;

        let mut data = black_box([40; SIZE]);

        b.iter(|| {
            test::ssse3_gimli(&mut data);
        });
    });

    #[cfg(gimli_test)]
    #[cfg(feature = "simd")]
    #[cfg(target_feature = "neon")]
    c.bench_function("neon", |b| {
        use gimli_permutation::test;

        let mut data = black_box([40; SIZE]);

        b.iter(|| {
            test::neon_gimli(&mut data);
        });
    });

    #[cfg(gimli_test)]
    #[cfg(feature = "simd")]
    c.bench_function("simd128", |b| {
        use gimli_permutation::test;

        let mut data = black_box([40; SIZE]);

        b.iter(|| {
            test::simd128_gimli::<()>(&mut data);
        });
    });
}

fn bench_gimli_x2(c: &mut Criterion) {
    c.bench_function("gimli_x2", |b| {
        use gimli_permutation::gimli_x2;

        let mut data = black_box([40; SIZE]);
        let mut data2 = black_box([41; SIZE]);

        b.iter(|| {
            gimli_x2(&mut data, &mut data2);
        });
    });

    #[cfg(gimli_test)]
    #[cfg(feature = "simd")]
    #[cfg(target_feature = "avx2")]
    c.bench_function("avx2", |b| {
        use gimli_permutation::test;

        let mut data = black_box([40; SIZE]);
        let mut data2 = black_box([41; SIZE]);

        b.iter(|| {
            test::avx2_gimli_x2(&mut data, &mut data2);
        });
    });

    #[cfg(gimli_test)]
    #[cfg(feature = "simd")]
    c.bench_function("simd256", |b| {
        use gimli_permutation::test;

        let mut data = black_box([40; SIZE]);
        let mut data2 = black_box([41; SIZE]);

        b.iter(|| {
            test::simd256_gimli_x2::<()>(&mut data, &mut data2);
        });
    });
}

criterion_group!(permutation, bench_gimli, bench_gimli_x2);
criterion_main!(permutation);
