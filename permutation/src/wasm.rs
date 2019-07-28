use core::arch::wasm32::{
    v128,
    v128_load, v128_store, v128_const,
    v128_and, v128_or, v128_xor,
    i32x4_shl, i32x4_shr_u
};
use crate::S;


macro_rules! shift {
    ( >> $a:expr, $imm8:expr ) => {
        i32x4_shr_u($a, $imm8)
    };
    ( << $a:expr, $imm8:expr ) => {
        i32x4_shl($a, $imm8)
    }
}

macro_rules! xor {
    ( $x:expr , $y:expr $( , $z:expr )* ) => {{
        let mut t = v128_xor($x, $y);
        $(
            t = v128_xor(t, $z);
        )*
        t
    }}
}

macro_rules! rotate {
    ( $y:expr, $bits:expr ) => {
        v128_or(shift!(<< $y, $bits), shift!(>> $y, 32 - $bits))
    }
}

macro_rules! rotate24 {
    ( $x:expr ) => {{
        let mut buf: [u8; 16] = [0; 16];
        v128_store(buf.as_mut_ptr() as *mut _, $x);
        v128_const(
            buf[12], buf[15], buf[14], buf[13],
            buf[ 8], buf[11], buf[10], buf[ 9],
            buf[ 4], buf[ 7], buf[ 6], buf[ 5],
            buf[ 0], buf[ 3], buf[ 2], buf[ 1]
        )
    }}
}

const COEFFS: [[u32; 4]; 6] = [
    [0x9e37_7904, 0, 0, 0],    [0x9e37_7908, 0, 0, 0],    [0x9e37_790c, 0, 0, 0],
    [0x9e37_7910, 0, 0, 0],    [0x9e37_7914, 0, 0, 0],    [0x9e37_7918, 0, 0, 0]
];


pub fn gimli(state: &mut [u32; S]) {
    let mut x = v128_load(state[0..].as_ptr() as *const _);
    let mut y = v128_load(state[4..].as_ptr() as *const _);
    let mut z = v128_load(state[8..].as_ptr() as *const _);

    macro_rules! round {
        () => {
            x = rotate24!(x);
            y = rotate!(y, 9);
            let newz = xor!(x, shift!(<< z, 1), shift!(<< v128_and(y, z), 2));
            let newy = xor!(y, x,               shift!(<<  v128_or(x, z), 1));
            x =        xor!(z, y,               shift!(<< v128_and(x, y), 3));
            y = newy;
            z = newz;
        }
    }

    for round in COEFFS.iter().rev() {
        round!();

        x = i32x4_shuffle(x, (2, 3, 0, 1));
        x = v128_xor(x, v128_load(round.as_ptr() as *const _));

        round!();
        round!();

        x = i32x4_shuffle(x, (1, 0, 3, 2));

        round!();
    }

    v128_store(state[0..].as_mut_ptr() as *mut _, x);
    v128_store(state[4..].as_mut_ptr() as *mut _, y);
    v128_store(state[8..].as_mut_ptr() as *mut _, z);
}

fn i32x4_shuffle(x: v128, (a, b, c, d): (usize, usize, usize, usize)) -> v128 {
    let mut buf: [u32; 4] = [0; 4];
    v128_store(buf.as_mut_ptr() as *mut _, x);
    let buf = [buf[a], buf[b], buf[c], buf[d]];
    v128_load(buf.as_ptr() as *const _)
}
