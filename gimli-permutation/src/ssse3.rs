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

pub unsafe fn gimli(state: &mut [u32; BLOCK_LENGTH]) {
    let mut x = u32x4::load(state, 0);
    let mut y = u32x4::load(state, 4);
    let mut z = u32x4::load(state, 8);

    macro_rules! round {
        () => {
            x = _mm_shuffle_epi8(
                x.into(),
                _mm_set_epi8(12, 15, 14, 13, 8, 11, 10, 9, 4, 7, 6, 5, 0, 3, 2, 1).into()
            ).into();
            y = (shift(y.into(), 9) | _mm_srli_epi32(y.into(), 32 - 9)).into();
            let newz = x.as_i32x4() ^ shift(z.into(), 1)    ^ shift((y & z).into(), 2);
            let newy = y            ^ x                     ^ shift((x | z).into(), 1).into();
            x =        z            ^ y                     ^ shift((x & y).into(), 3).into();
            y = newy;
            z = newz.into();
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = _mm_shuffle_epi32(x.into(), shuffle(2, 3, 0, 1)).into();
        x ^= round;

        round!();
        round!();

        x = _mm_shuffle_epi32(x.into(), shuffle(1, 0, 3, 2)).into();

        round!();
    }

    x.store(state, 0);
    y.store(state, 4);
    z.store(state, 8);
}


#[inline]
fn shuffle(fp3: i32, fp2: i32, fp1: i32, fp0: i32) -> i32 {
    (fp3 << 6) | (fp2 << 4) | (fp1 << 2) | fp0
}
