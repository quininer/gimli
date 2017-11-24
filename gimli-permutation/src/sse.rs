use core::mem::transmute as into;
use coresimd::simd::u32x4;
use coresimd::vendor::{
    _mm_shuffle_epi8, _mm_shuffle_epi32,
    _mm_set_epi8,
    _mm_srli_epi32,

    _mm_slli_epi32 as shift
};
use ::BLOCK_LENGTH;



const COEFFS: [u32x4; 6] = [
    u32x4::new(0x9e37_7904, 0, 0, 0), u32x4::new(0x9e37_7908, 0, 0, 0), u32x4::new(0x9e37_790c, 0, 0, 0),
    u32x4::new(0x9e37_7910, 0, 0, 0), u32x4::new(0x9e37_7914, 0, 0, 0), u32x4::new(0x9e37_7918, 0, 0, 0)
];

#[allow(unused_assignments)] // false positive
pub unsafe fn gimli(state: &mut [u32; BLOCK_LENGTH]) {
    let mut x = u32x4::load(&state[..4], 0);
    let mut y = u32x4::load(&state[4..][..8], 0);
    let mut z = u32x4::load(&state[8..], 0);

    macro_rules! round {
        () => {
            x = into(_mm_shuffle_epi8(
                into(x),
                _mm_set_epi8(12, 15, 14, 13, 8, 11, 10, 9, 4, 7, 6, 5, 0, 3, 2, 1).into()
            ));
            y = (shift(into(y), 9) | _mm_srli_epi32(into(y), 32 - 9)).into();
            let newz = x.as_i32x4() ^ shift(into(z), 1)    ^ shift(into(y & z), 2);
            let newy = y            ^ x                     ^ into(shift(into(x | z), 1));
            x =        z            ^ y                     ^ into(shift(into(x & y), 3));
            y = newy;
            z = into(newz);
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = into(_mm_shuffle_epi32(into(x), shuffle(2, 3, 0, 1)));
        x ^= round;

        round!();
        round!();

        x = into(_mm_shuffle_epi32(into(x), shuffle(1, 0, 3, 2)));

        round!();
    }

    x.store(&mut state[..4], 0);
    y.store(&mut state[4..][..8], 0);
    z.store(&mut state[8..], 0);
}


#[inline]
fn shuffle(fp3: i32, fp2: i32, fp1: i32, fp0: i32) -> i32 {
    (fp3 << 6) | (fp2 << 4) | (fp1 << 2) | fp0
}
