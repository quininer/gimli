#[cfg(target_arch = "x86")]
use core::arch::x86::{
    _mm_set_epi8,
    _mm_loadu_si128, _mm_storeu_si128,
    _mm_shuffle_epi8, _mm_shuffle_epi32,
    _mm_srli_epi32, _mm_slli_epi32,
    _mm_and_si128, _mm_or_si128, _mm_xor_si128
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    _mm_set_epi8,
    _mm_loadu_si128, _mm_storeu_si128,
    _mm_shuffle_epi8, _mm_shuffle_epi32,
    _mm_srli_epi32, _mm_slli_epi32,
    _mm_and_si128, _mm_or_si128, _mm_xor_si128
};
use crate::S;


macro_rules! shuffle {
    ( $fp3:expr, $fp2:expr, $fp1:expr, $fp0:expr ) => {
        ($fp3 << 6) | ($fp2 << 4) | ($fp1 << 2) | $fp0
    }
}

macro_rules! shift {
    ( >> $a:expr, $imm8:expr ) => {
        _mm_srli_epi32($a, $imm8)
    };
    ( << $a:expr, $imm8:expr ) => {
        _mm_slli_epi32($a, $imm8)
    }
}

macro_rules! xor {
    ( $x:expr , $y:expr $( , $z:expr )* ) => {{
        let mut t = _mm_xor_si128($x, $y);
        $(
            t = _mm_xor_si128(t, $z);
        )*
        t
    }}
}

macro_rules! rotate {
    ( $y:expr, $bits:expr ) => {
        _mm_or_si128(shift!(<< $y, $bits), shift!(>> $y, 32 - $bits))
    }
}

macro_rules! rotate24 {
    ( $x:expr ) => {
        _mm_shuffle_epi8(
            $x,
            _mm_set_epi8(12, 15, 14, 13, 8, 11, 10, 9, 4, 7, 6, 5, 0, 3, 2, 1)
        )
    }
}

const COEFFS: [[u32; 4]; 6] = [
    [0x9e37_7904, 0, 0, 0],    [0x9e37_7908, 0, 0, 0],    [0x9e37_790c, 0, 0, 0],
    [0x9e37_7910, 0, 0, 0],    [0x9e37_7914, 0, 0, 0],    [0x9e37_7918, 0, 0, 0]
];


#[target_feature(enable = "ssse3")]
pub unsafe fn gimli(state: &mut [u32; S]) {
    let mut x = _mm_loadu_si128(state[0..].as_ptr() as *const _);
    let mut y = _mm_loadu_si128(state[4..].as_ptr() as *const _);
    let mut z = _mm_loadu_si128(state[8..].as_ptr() as *const _);

    macro_rules! round {
        () => {
            x = rotate24!(x);
            y = rotate!(y, 9);
            let newz = xor!(x, shift!(<< z, 1), shift!(<< _mm_and_si128(y, z), 2));
            let newy = xor!(y, x,               shift!(<<  _mm_or_si128(x, z), 1));
            x =        xor!(z, y,               shift!(<< _mm_and_si128(x, y), 3));
            y = newy;
            z = newz;
        }
    }

    for round in COEFFS.iter().rev() {
        round!();

        x = _mm_shuffle_epi32(x, shuffle!(2, 3, 0, 1));
        x = _mm_xor_si128(x, _mm_loadu_si128(round.as_ptr() as *const _));

        round!();
        round!();

        x = _mm_shuffle_epi32(x, shuffle!(1, 0, 3, 2));

        round!();
    }

    _mm_storeu_si128(state[0..].as_mut_ptr() as *mut _, x);
    _mm_storeu_si128(state[4..].as_mut_ptr() as *mut _, y);
    _mm_storeu_si128(state[8..].as_mut_ptr() as *mut _, z);
}
