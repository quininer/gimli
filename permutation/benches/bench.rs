#![feature(test)]

extern crate test;

use test::{ Bencher, black_box };
use gimli_permutation::SIZE;


#[bench]
fn bench_gimli(b: &mut Bencher) {
    use gimli_permutation::gimli;

    let mut data = black_box([40; SIZE]);

    b.iter(|| {
        gimli(&mut data);
    });
}

#[bench]
fn bench_gimli_x2(b: &mut Bencher) {
    use gimli_permutation::gimli_x2;

    let mut data = black_box([40; SIZE]);
    let mut data2 = black_box([41; SIZE]);

    b.iter(|| {
        gimli_x2(&mut data, &mut data2);
    });
}

#[bench]
fn bench_gimli_portable(b: &mut Bencher) {
    use gimli_permutation::portable;

    let mut data = black_box([41; SIZE]);

    b.iter(|| {
        portable::gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "ssse3")]
#[bench]
fn bench_gimli_ssse3(b: &mut Bencher) {
    use gimli_permutation::test;

    let mut data = black_box([42; SIZE]);

    b.iter(|| unsafe {
        test::ssse3_gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "avx2")]
#[bench]
fn bench_gimli_avx2(b: &mut Bencher) {
    use gimli_permutation::test;

    let mut data = black_box([43; SIZE]);
    let mut data2 = black_box([44; SIZE]);

    b.iter(|| unsafe {
        test::avx2_gimli_x2(&mut data, &mut data2);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "neon")]
#[bench]
fn bench_gimli_neon(b: &mut Bencher) {
    use gimli_permutation::test;

    let mut data = black_box([42; SIZE]);

    b.iter(|| unsafe {
        test::neon_gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[bench]
fn bench_gimli_simd128(b: &mut Bencher) {
    use gimli_permutation::test;

    let mut data = black_box([43; SIZE]);

    b.iter(|| {
        test::simd128_gimli::<u32>(&mut data);
    });
}

#[cfg(feature = "simd")]
#[bench]
fn bench_gimli_simd256(b: &mut Bencher) {
    use gimli_permutation::test;

    let mut data = black_box([45; SIZE]);
    let mut data2 = black_box([46; SIZE]);

    b.iter(|| {
        test::simd256_gimli_x2::<u32>(&mut data, &mut data2);
    });
}
