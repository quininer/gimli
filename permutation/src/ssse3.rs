use core::simd::u32x4;
use core::simd::{ FromBits, IntoBits };
use core::arch::x86_64::{
    _mm_shuffle_epi8, _mm_shuffle_epi32,
    _mm_set_epi8,
    _mm_srli_epi32, _mm_slli_epi32
};
use ::S;


macro_rules! shuffle {
    ( $fp3:expr, $fp2:expr, $fp1:expr, $fp0:expr ) => {
        ($fp3 << 6) | ($fp2 << 4) | ($fp1 << 2) | $fp0
    }
}

macro_rules! shift {
    ( right $a:expr, $imm8:expr ) => {
        u32x4::from_bits(_mm_srli_epi32($a.into_bits(), $imm8))
    };
    ( left $a:expr, $imm8:expr ) => {
        u32x4::from_bits(_mm_slli_epi32($a.into_bits(), $imm8))
    }
}

const COEFFS: [u32x4; 6] = [
    u32x4::new(0x9e37_7904, 0, 0, 0), u32x4::new(0x9e37_7908, 0, 0, 0), u32x4::new(0x9e37_790c, 0, 0, 0),
    u32x4::new(0x9e37_7910, 0, 0, 0), u32x4::new(0x9e37_7914, 0, 0, 0), u32x4::new(0x9e37_7918, 0, 0, 0)
];

#[target_feature(enable = "ssse3")]
pub unsafe fn gimli(state: &mut [u32; S]) {
    let mut x = u32x4::load_unaligned(&state[0..]);
    let mut y = u32x4::load_unaligned(&state[4..]);
    let mut z = u32x4::load_unaligned(&state[8..]);

    macro_rules! round {
        () => {
            x = _mm_shuffle_epi8(
                x.into_bits(),
                _mm_set_epi8(12, 15, 14, 13, 8, 11, 10, 9, 4, 7, 6, 5, 0, 3, 2, 1).into_bits()
            ).into_bits();
            y = shift!(left y, 9) | shift!(right y, 32 - 9);
            let newz = x ^ shift!(left z, 1) ^ shift!(left y & z, 2);
            let newy = y ^ x                ^ shift!(left x | z, 1);
            x =        z ^ y                ^ shift!(left x & y, 3);
            y = newy;
            z = newz;
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = _mm_shuffle_epi32(x.into_bits(), shuffle!(2, 3, 0, 1)).into_bits();
        x ^= round;

        round!();
        round!();

        x = _mm_shuffle_epi32(x.into_bits(), shuffle!(1, 0, 3, 2)).into_bits();

        round!();
    }

    x.store_unaligned(&mut state[0..]);
    y.store_unaligned(&mut state[4..]);
    z.store_unaligned(&mut state[8..]);
}
