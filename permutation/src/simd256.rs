use std::simd::{ u32x8, simd_swizzle };
use crate::SIZE;


const COEFFS: [u32x8; 6] = [
    u32x8::from_array([0x9e37_7904, 0, 0, 0, 0x9e37_7904, 0, 0, 0]),
    u32x8::from_array([0x9e37_7908, 0, 0, 0, 0x9e37_7908, 0, 0, 0]),
    u32x8::from_array([0x9e37_790c, 0, 0, 0, 0x9e37_790c, 0, 0, 0]),
    u32x8::from_array([0x9e37_7910, 0, 0, 0, 0x9e37_7910, 0, 0, 0]),
    u32x8::from_array([0x9e37_7914, 0, 0, 0, 0x9e37_7914, 0, 0, 0]),
    u32x8::from_array([0x9e37_7918, 0, 0, 0, 0x9e37_7918, 0, 0, 0])
];

#[inline]
pub fn gimli_x2<T>(state: &mut [u32; SIZE], state2: &mut [u32; SIZE]) {
    macro_rules! load {
        ( $s:expr, $s2:expr, $n:expr ) => {
            u32x8::from_array([
                $s[$n], $s[$n + 1], $s[$n + 2], $s[$n + 3],
                $s2[$n], $s2[$n + 1], $s2[$n + 2], $s2[$n + 3]
            ])
        }
    }

    macro_rules! store {
        ( $x:expr, $s:expr, $s2:expr, $n:expr ) => {
            let buf = $x.as_array();
            $s[$n..][..4].copy_from_slice(&buf[..4]);
            $s2[$n..][..4].copy_from_slice(&buf[4..]);
        }
    }

    let mut x = load!(state, state2, 0);
    let mut y = load!(state, state2, 4);
    let mut z = load!(state, state2, 8);

    macro_rules! round {
        () => {
            x = u32x8_rotate_left::<24>(x);
            y = u32x8_rotate_left::<9>(y);
            let newz = x ^ (z << u32x8::splat(1))   ^ ((y & z) << u32x8::splat(2));
            let newy = y ^ x                        ^ ((x | z) << u32x8::splat(1));
            x        = z ^ y                        ^ ((x & y) << u32x8::splat(3));
            y = newy;
            z = newz;
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = simd_swizzle!(x, [1, 0, 3, 2, 5, 4, 7, 6]);
        x ^= round;

        round!();
        round!();

        x = simd_swizzle!(x, [2, 3, 0, 1, 6, 7, 4, 5]);

        round!();
    }

    store!(x, state, state2, 0);
    store!(y, state, state2, 4);
    store!(z, state, state2, 8);
}

#[inline]
fn u32x8_rotate_left<const OFFSET: u32>(value: u32x8) -> u32x8 {
    const WIDTH: u32 = core::mem::size_of::<u32x8>() as u32 * 8;

    let n = OFFSET % WIDTH;
    (value << u32x8::splat(n))
        | (value >> u32x8::splat((WIDTH - n) % WIDTH))
}
