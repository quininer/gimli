#![feature(test)]

extern crate test;
extern crate gimli_hash;

use test::{ Bencher, black_box };
use gimli_hash::GimliHash;


#[bench]
fn bench_gimlihash_16(b: &mut Bencher) {
    let data = black_box([254; 16]);
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res = [0; 32];
        let mut hasher = GimliHash::default();
        hasher.update(&data);
        hasher.finalize(&mut res);
    });
}

#[bench]
fn bench_gimlihash_256(b: &mut Bencher) {
    let data = black_box([254; 256]);
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res = [0; 32];
        let mut hasher = GimliHash::default();
        hasher.update(&data);
        hasher.finalize(&mut res);
    });
}

#[bench]
fn bench_gimlihash_4096(b: &mut Bencher) {
    let data = black_box([254; 4096]);
    b.bytes = data.len() as u64;

    b.iter(|| {
        let mut res = [0; 32];
        let mut hasher = GimliHash::default();
        hasher.update(&data);
        hasher.finalize(&mut res);
    });
}
