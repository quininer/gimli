#[cfg(target_arch = "x86")]
use core::arch::x86::{
    _mm_loadu_si128, _mm_storeu_si128,
    _mm256_loadu2_m128i, _mm256_storeu2_m128i,
    _mm256_loadu_si256,
    _mm256_shuffle_epi8, _mm256_shuffle_epi32,
    _mm256_set_epi8,
    _mm256_srli_epi32, _mm256_slli_epi32,
    _mm256_and_si256, _mm256_or_si256, _mm256_xor_si256
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    _mm_loadu_si128, _mm_storeu_si128,
    _mm256_loadu2_m128i, _mm256_storeu2_m128i,
    _mm256_loadu_si256,
    _mm256_shuffle_epi8, _mm256_shuffle_epi32,
    _mm256_set_epi8,
    _mm256_srli_epi32, _mm256_slli_epi32,
    _mm256_and_si256, _mm256_or_si256, _mm256_xor_si256
};
use crate::S;


macro_rules! shuffle {
    ( $fp3:expr, $fp2:expr, $fp1:expr, $fp0:expr ) => {
        ($fp3 << 6) | ($fp2 << 4) | ($fp1 << 2) | $fp0
    }
}

macro_rules! shift {
    ( >> $a:expr, $imm8:expr ) => {
        _mm256_srli_epi32($a, $imm8)
    };
    ( << $a:expr, $imm8:expr ) => {
        _mm256_slli_epi32($a, $imm8)
    }
}

macro_rules! xor {
    ( $x:expr , $y:expr $( , $z:expr )* ) => {{
        let mut t = _mm256_xor_si256($x, $y);
        $(
            t = _mm256_xor_si256(t, $z);
        )*
        t
    }}
}

const COEFFS: [[u32; 8]; 6] = [
    [0x9e37_7904, 0, 0, 0, 0x9e37_7904, 0, 0, 0],
    [0x9e37_7908, 0, 0, 0, 0x9e37_7908, 0, 0, 0],
    [0x9e37_790c, 0, 0, 0, 0x9e37_790c, 0, 0, 0],
    [0x9e37_7910, 0, 0, 0, 0x9e37_7910, 0, 0, 0],
    [0x9e37_7914, 0, 0, 0, 0x9e37_7914, 0, 0, 0],
    [0x9e37_7918, 0, 0, 0, 0x9e37_7918, 0, 0, 0]
];


#[deprecated(since="0.1.1", note="please use `avx2::gimli_x2` instead")]
#[inline]
pub unsafe fn gimli(state: &mut [u32; S], state2: &mut [u32; S]) {
    gimli_x2(state, state2)
}

#[target_feature(enable = "avx2")]
pub unsafe fn gimli_x2(state: &mut [u32; S], state2: &mut [u32; S]) {
    let (mut x1, mut x2) =
        (_mm_loadu_si128(state[0..].as_ptr() as *const _), _mm_loadu_si128(state2[0..].as_ptr() as *const _));
    let (mut y1, mut y2) =
        (_mm_loadu_si128(state[4..].as_ptr() as *const _), _mm_loadu_si128(state2[4..].as_ptr() as *const _));
    let (mut z1, mut z2) =
        (_mm_loadu_si128(state[8..].as_ptr() as *const _), _mm_loadu_si128(state2[8..].as_ptr() as *const _));

    let mut x = _mm256_loadu2_m128i(&x1, &x2);
    let mut y = _mm256_loadu2_m128i(&y1, &y2);
    let mut z = _mm256_loadu2_m128i(&z1, &z2);

    macro_rules! round {
        () => {
            x = _mm256_shuffle_epi8(
                x,
                _mm256_set_epi8(
                    12, 15, 14, 13, 8,  11, 10, 9,  4,  7,  6,  5,  0,  3,  2,  1,
                    28, 31, 30, 29, 24, 27, 26, 25, 20, 23, 22, 21, 16, 19, 18, 17
                )
            );
            y = _mm256_or_si256(shift!(<< y, 9), shift!(>> y, 32 - 9));
            let newz = xor!(x, shift!(<< z, 1), shift!(<< _mm256_and_si256(y, z), 2));
            let newy = xor!(y, x, shift!(<< _mm256_or_si256(x, z), 1));
            x =        xor!(z, y, shift!(<< _mm256_and_si256(x, y), 3));
            y = newy;
            z = newz;
        }
    }

    for round in COEFFS.iter().rev() {
        round!();

        x = _mm256_shuffle_epi32(x, shuffle!(2, 3, 0, 1));
        x = _mm256_xor_si256(x, _mm256_loadu_si256(round.as_ptr() as *const _));

        round!();
        round!();

        x = _mm256_shuffle_epi32(x, shuffle!(1, 0, 3, 2));

        round!();
    }

    _mm256_storeu2_m128i(&mut x1, &mut x2, x);
    _mm256_storeu2_m128i(&mut y1, &mut y2, y);
    _mm256_storeu2_m128i(&mut z1, &mut z2, z);

    _mm_storeu_si128(state[0..].as_mut_ptr() as *mut _, x1);
    _mm_storeu_si128(state[4..].as_mut_ptr() as *mut _, y1);
    _mm_storeu_si128(state[8..].as_mut_ptr() as *mut _, z1);
    _mm_storeu_si128(state2[0..].as_mut_ptr() as *mut _, x2);
    _mm_storeu_si128(state2[4..].as_mut_ptr() as *mut _, y2);
    _mm_storeu_si128(state2[8..].as_mut_ptr() as *mut _, z2);
}
