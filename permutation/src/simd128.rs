use packed_simd::{ u32x4, shuffle };
use crate::S;


const COEFFS: [u32x4; 6] = [
    u32x4::new(0x9e37_7904, 0, 0, 0), u32x4::new(0x9e37_7908, 0, 0, 0), u32x4::new(0x9e37_790c, 0, 0, 0),
    u32x4::new(0x9e37_7910, 0, 0, 0), u32x4::new(0x9e37_7914, 0, 0, 0), u32x4::new(0x9e37_7918, 0, 0, 0)
];

#[inline]
pub fn gimli<T>(state: &mut [u32; S]) {
    let mut x = u32x4::from_slice_unaligned(&state[0..][..4]);
    let mut y = u32x4::from_slice_unaligned(&state[4..][..4]);
    let mut z = u32x4::from_slice_unaligned(&state[8..][..4]);

    macro_rules! round {
        () => {
            x = x.rotate_left(u32x4::splat(24));
            y = y.rotate_left(u32x4::splat(9));
            let newz = x ^ (z << 1) ^ ((y & z) << 2);
            let newy = y ^ x        ^ ((x | z) << 1);
            x        = z ^ y        ^ ((x & y) << 3);
            y = newy;
            z = newz;
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = shuffle!(x, [1, 0, 3, 2]);
        x ^= round;

        round!();
        round!();

        x = shuffle!(x, [2, 3, 0, 1]);

        round!();
    }

    x.write_to_slice_unaligned(&mut state[0..][..4]);
    y.write_to_slice_unaligned(&mut state[4..][..4]);
    z.write_to_slice_unaligned(&mut state[8..][..4]);
}
