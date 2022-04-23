use core::simd::{ u32x4, simd_swizzle };
use crate::SIZE;


const COEFFS: [u32x4; 6] = [
    u32x4::from_array([0x9e37_7904, 0, 0, 0]), u32x4::from_array([0x9e37_7908, 0, 0, 0]),
    u32x4::from_array([0x9e37_790c, 0, 0, 0]), u32x4::from_array([0x9e37_7910, 0, 0, 0]),
    u32x4::from_array([0x9e37_7914, 0, 0, 0]), u32x4::from_array([0x9e37_7918, 0, 0, 0])
];

#[inline]
pub fn gimli<T>(state: &mut [u32; SIZE]) {
    let mut x = u32x4::from_slice(&state[0..][..4]);
    let mut y = u32x4::from_slice(&state[4..][..4]);
    let mut z = u32x4::from_slice(&state[8..][..4]);

    macro_rules! round {
        () => {
            x = u32x4_rotate_left::<24>(x);
            y = u32x4_rotate_left::<9>(y);
            let newz = x ^ (z << u32x4::splat(1))   ^ ((y & z) << u32x4::splat(2));
            let newy = y ^ x                        ^ ((x | z) << u32x4::splat(1));
            x        = z ^ y                        ^ ((x & y) << u32x4::splat(3));
            y = newy;
            z = newz;
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = simd_swizzle!(x, [1, 0, 3, 2]);
        x ^= round;

        round!();
        round!();

        x = simd_swizzle!(x, [2, 3, 0, 1]);

        round!();
    }

    state[0..][..4].copy_from_slice(x.as_array());
    state[4..][..4].copy_from_slice(y.as_array());
    state[8..][..4].copy_from_slice(z.as_array());
}

#[inline]
fn u32x4_rotate_left<const OFFSET: u32>(value: u32x4) -> u32x4 {
    const WIDTH: u32 = core::mem::size_of::<u32x4>() as u32 * 8;

    let n = OFFSET % WIDTH;
    (value << u32x4::splat(n))
        | (value >> u32x4::splat((WIDTH - n) % WIDTH))
}
