use core::mem::transmute as into;
use coresimd::simd::{ u32x4, u32x8, i32x8 };
use coresimd::vendor::{
    _mm256_loadu2_m128i, _mm256_storeu2_m128i,
    _mm256_shuffle_epi8, _mm256_set_epi8,
    _mm256_srli_epi32, _mm256_shuffle_epi32,

    _mm256_slli_epi32 as shift
};
use ::BLOCK_LENGTH;


const COEFFS: [u32x8; 6] = [
    u32x8::new(0x9e37_7904, 0, 0, 0, 0x9e37_7904, 0, 0, 0),
    u32x8::new(0x9e37_7908, 0, 0, 0, 0x9e37_7908, 0, 0, 0),
    u32x8::new(0x9e37_790c, 0, 0, 0, 0x9e37_790c, 0, 0, 0),
    u32x8::new(0x9e37_7910, 0, 0, 0, 0x9e37_7910, 0, 0, 0),
    u32x8::new(0x9e37_7914, 0, 0, 0, 0x9e37_7914, 0, 0, 0),
    u32x8::new(0x9e37_7918, 0, 0, 0, 0x9e37_7918, 0, 0, 0),
];

pub unsafe fn gimli(state: &mut [u32; BLOCK_LENGTH * 2]) {
    let (mut x1, mut x2) = (u32x4::load(state, 0), u32x4::load(state, 12));
    let (mut y1, mut y2) = (u32x4::load(state, 4), u32x4::load(state, 16));
    let (mut z1, mut z2) = (u32x4::load(state, 8), u32x4::load(state, 20));

    let mut x = _mm256_loadu2_m128i(into(&x1), into(&x2));
    let mut y = _mm256_loadu2_m128i(into(&y1), into(&y2));
    let mut z = _mm256_loadu2_m128i(into(&z1), into(&z2));

    macro_rules! round {
        () => {
            x = _mm256_shuffle_epi8(
                x.as_u8x32(),
                _mm256_set_epi8(
                    12, 15, 14, 13, 8,  11, 10, 9,  4,  7,  6,  5,  0,  3,  2,  1,
                    28, 31, 30, 29, 24, 27, 26, 25, 20, 23, 22, 21, 16, 19, 18, 17
                ).as_u8x32()
            ).as_i8x32();
            y = (shift(into(y), 9) | _mm256_srli_epi32(into(y), 32 - 9)).into();
            let newz = into::<_, i32x8>(x)  ^ shift(into(z), 1) ^ shift(into(y & z), 2);
            let newy = y                    ^ x                 ^ into(shift(into(x | z), 1));
            x =        z                    ^ y                 ^ into(shift(into(x & y), 3));
            y = newy;
            z = into(newz);
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = into(_mm256_shuffle_epi32(into(x), shuffle(2, 3, 0, 1)));
        x ^= into(round);

        round!();
        round!();

        x = into(_mm256_shuffle_epi32(into(x), shuffle(1, 0, 3, 2)));

        round!();
    }

    _mm256_storeu2_m128i(into(&mut x1), into(&mut x2), x);
    _mm256_storeu2_m128i(into(&mut y1), into(&mut y2), y);
    _mm256_storeu2_m128i(into(&mut z1), into(&mut z2), z);

    x1.store(state, 0);
    x2.store(state, 12);
    y1.store(state, 4);
    y2.store(state, 16);
    z1.store(state, 8);
    z2.store(state, 20);
}


#[inline]
fn shuffle(fp3: i32, fp2: i32, fp1: i32, fp0: i32) -> i32 {
    (fp3 << 6) | (fp2 << 4) | (fp1 << 2) | fp0
}
