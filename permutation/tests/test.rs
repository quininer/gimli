#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#![cfg_attr(feature = "simd", feature(cfg_target_feature))]

extern crate gimli_permutation;

use gimli_permutation::BLOCK_LENGTH;


const INPUT: [u32; BLOCK_LENGTH] = [
    0x00000000, 0x9e3779ba, 0x3c6ef37a, 0xdaa66d46,
    0x78dde724, 0x1715611a, 0xb54cdb2e, 0x53845566,
    0xf1bbcfc8, 0x8ff34a5a, 0x2e2ac522, 0xcc624026,
];
const OUTPUT: [u32; BLOCK_LENGTH] = [
    0xba11c85a, 0x91bad119, 0x380ce880, 0xd24c2c68,
    0x3eceffea, 0x277a921c, 0x4f73a0bd, 0xda5a9cd8,
    0x84b673f0, 0x34e52ff7, 0x9e2bef49, 0xf41bb8d6,
];

#[test]
fn test_gimli() {
    let mut data = [0; BLOCK_LENGTH];
    data.copy_from_slice(&INPUT);

    gimli_permutation::portable::gimli(&mut data);
    assert_eq!(data, OUTPUT);


    let mut data = [0; BLOCK_LENGTH];
    data.copy_from_slice(&INPUT);

    gimli_permutation::gimli(&mut data);
    assert_eq!(data, OUTPUT);
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "ssse3")]
#[test]
fn test_gimli_ssse3() {
    let mut data = [0; BLOCK_LENGTH];
    data.copy_from_slice(&INPUT);

    unsafe {
        gimli_permutation::ssse3::gimli(&mut data);
    }
    assert_eq!(data, OUTPUT);
}

#[cfg(feature = "simd")]
#[cfg(target_feature = "avx2")]
#[test]
fn test_gimli_avx2() {
    let mut data = [0; BLOCK_LENGTH];
    let mut data2 = [0; BLOCK_LENGTH];
    data.copy_from_slice(&INPUT);

    unsafe {
        gimli_permutation::avx2::gimli(&mut data, &mut data2);
    }
    assert_eq!(data, OUTPUT);


    let mut data = [0; BLOCK_LENGTH];
    let mut data2 = [0; BLOCK_LENGTH];
    data2.copy_from_slice(&INPUT);

    unsafe {
        gimli_permutation::avx2::gimli(&mut data, &mut data2);
    }
    assert_eq!(data2, OUTPUT);


    let mut data = [0; BLOCK_LENGTH];
    let mut data2 = [0; BLOCK_LENGTH];
    data.copy_from_slice(&INPUT);
    data2.copy_from_slice(&INPUT);

    unsafe {
        gimli_permutation::avx2::gimli(&mut data, &mut data2);
    }
    assert_eq!(data, OUTPUT);
    assert_eq!(data2, OUTPUT);
}
