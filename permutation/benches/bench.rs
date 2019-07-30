#![feature(test)]

extern crate test;
extern crate gimli_permutation;

use test::{ Bencher, black_box };
use gimli_permutation::S;


#[bench]
fn bench_gimli(b: &mut Bencher) {
    use gimli_permutation::gimli;

    let mut data = black_box([40; S]);

    b.iter(|| {
        gimli(&mut data);
    });
}

#[bench]
fn bench_gimli_portable(b: &mut Bencher) {
    use gimli_permutation::portable;

    let mut data = black_box([41; S]);

    b.iter(|| {
        portable::gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "ssse3")]
#[bench]
fn bench_gimli_ssse3(b: &mut Bencher) {
    use gimli_permutation::ssse3;

    let mut data = black_box([42; S]);

    b.iter(|| unsafe {
        ssse3::gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "avx2")]
#[bench]
fn bench_gimli_avx2(b: &mut Bencher) {
    use gimli_permutation::avx2;

    let mut data = black_box([43; S]);
    let mut data2 = black_box([44; S]);

    b.iter(|| unsafe {
        avx2::gimli_x2(&mut data, &mut data2);
    });
}
