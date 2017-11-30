#![feature(test)]
#![cfg_attr(feature = "simd", feature(cfg_target_feature))]

extern crate test;
extern crate gimli_permutation;

use test::Bencher;
use gimli_permutation::BLOCK_LENGTH;


#[bench]
fn bench_gimli_portable(b: &mut Bencher) {
    use gimli_permutation::portable;

    let mut data = [42; BLOCK_LENGTH];

    b.iter(|| {
        portable::gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "ssse3")]
#[bench]
fn bench_gimli_ssse3(b: &mut Bencher) {
    use gimli_permutation::ssse3;

    let mut data = [42; BLOCK_LENGTH];

    b.iter(|| unsafe {
        ssse3::gimli(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "avx2")]
#[bench]
fn bench_gimli_avx2(b: &mut Bencher) {
    use gimli_permutation::avx2;

    let mut data = [42; BLOCK_LENGTH];
    let mut data2 = [42; BLOCK_LENGTH];

    b.iter(|| unsafe {
        avx2::gimli(&mut data, &mut data2);
    });
}
