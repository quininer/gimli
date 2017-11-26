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

    let mut x = _mm256_loadu2_m128i(&x1 as *const _ as _, &x2 as *const _ as _);
    let mut y = _mm256_loadu2_m128i(&y1 as *const _ as _, &y2 as *const _ as _);
    let mut z = _mm256_loadu2_m128i(&z1 as *const _ as _, &z2 as *const _ as _);

    macro_rules! round {
        () => {
            x = _mm256_shuffle_epi8(
                x.as_u8x32(),
                _mm256_set_epi8(
                    12, 15, 14, 13, 8,  11, 10, 9,  4,  7,  6,  5,  0,  3,  2,  1,
                    28, 31, 30, 29, 24, 27, 26, 25, 20, 23, 22, 21, 16, 19, 18, 17
                ).as_u8x32()
            ).as_i8x32();
            y = (shift(y.into(), 9) | _mm256_srli_epi32(y.into(), 32 - 9)).into();
            let newz = i32x8::from(x)   ^ shift(z.into(), 1)    ^ shift((y & z).into(), 2);
            let newy = y                ^ x                     ^ shift((x | z).into(), 1).into();
            x =        z                ^ y                     ^ shift((x & y).into(), 3).into();
            y = newy;
            z = newz.into();
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = _mm256_shuffle_epi32(x.into(), shuffle(2, 3, 0, 1)).into();
        x ^= round.into();

        round!();
        round!();

        x = _mm256_shuffle_epi32(x.into(), shuffle(1, 0, 3, 2)).into();

        round!();
    }

    _mm256_storeu2_m128i(&mut x1 as *mut _ as _, &mut x2 as *mut _ as _, x);
    _mm256_storeu2_m128i(&mut y1 as *mut _ as _, &mut y2 as *mut _ as _, y);
    _mm256_storeu2_m128i(&mut z1 as *mut _ as _, &mut z2 as *mut _ as _, z);

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
