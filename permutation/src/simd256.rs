use packed_simd::{ u32x8, shuffle };
use crate::S;


const COEFFS: [u32x8; 6] = [
    u32x8::new(0x9e37_7904, 0, 0, 0, 0x9e37_7904, 0, 0, 0),
    u32x8::new(0x9e37_7908, 0, 0, 0, 0x9e37_7908, 0, 0, 0),
    u32x8::new(0x9e37_790c, 0, 0, 0, 0x9e37_790c, 0, 0, 0),
    u32x8::new(0x9e37_7910, 0, 0, 0, 0x9e37_7910, 0, 0, 0),
    u32x8::new(0x9e37_7914, 0, 0, 0, 0x9e37_7914, 0, 0, 0),
    u32x8::new(0x9e37_7918, 0, 0, 0, 0x9e37_7918, 0, 0, 0)
];

#[inline]
pub fn gimli_x2<T>(state: &mut [u32; S], state2: &mut [u32; S]) {
    macro_rules! load {
        ( $s:expr, $s2:expr, $n:expr ) => {
            u32x8::new(
                $s[$n], $s[$n + 1], $s[$n + 2], $s[$n + 3],
                $s2[$n], $s2[$n + 1], $s2[$n + 2], $s2[$n + 3]
            )
        }
    }

    macro_rules! store {
        ( $x:expr, $s:expr, $s2:expr, $n:expr ) => {
            let mut buf = [0; 8];
            $x.write_to_slice_unaligned(&mut buf);
            $s[$n..][..4].copy_from_slice(&buf[..4]);
            $s2[$n..][..4].copy_from_slice(&buf[4..]);
        }
    }

    let mut x = load!(state, state2, 0);
    let mut y = load!(state, state2, 4);
    let mut z = load!(state, state2, 8);

    macro_rules! round {
        () => {
            x = x.rotate_left(u32x8::splat(24));
            y = y.rotate_left(u32x8::splat(9));
            let newz = x ^ (z << 1) ^ ((y & z) << 2);
            let newy = y ^ x        ^ ((x | z) << 1);
            x        = z ^ y        ^ ((x & y) << 3);
            y = newy;
            z = newz;
        }
    }

    for &round in COEFFS.iter().rev() {
        round!();

        x = shuffle!(x, [1, 0, 3, 2, 1 + 4, 0 + 4, 3 + 4, 2 + 4]);
        x ^= round;

        round!();
        round!();

        x = shuffle!(x, [2, 3, 0, 1, 2 + 4, 3 + 4, 0 + 4, 1 + 4]);

        round!();
    }

    store!(x, state, state2, 0);
    store!(y, state, state2, 4);
    store!(z, state, state2, 8);
}
