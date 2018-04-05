#![no_std]
#![cfg_attr(feature = "simd", feature(target_feature, cfg_target_feature, stdsimd))]

// FIXME `is_x86_feature_detected!` missing in `core`.
#[cfg(feature = "simd")]
#[macro_use]
extern crate std;

pub mod portable;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod ssse3;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;


pub const S: usize = 12;

#[deprecated(since="0.1.1", note="please use `S` instead")]
pub const BLOCK_LENGTH: usize = S;

#[inline]
pub fn gimli(state: &mut [u32; S]) {
    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("ssse3") {
            return ssse3::gimli(state);
        }
    }

    portable::gimli(state)
}

#[deprecated(since="0.1.1", note="please use `gimli_x2` instead")]
#[inline]
pub fn gimlix2(state: &mut [u32; S], state2: &mut [u32; S]) {
    gimli_x2(state, state2)
}

#[inline]
pub fn gimli_x2(state: &mut [u32; S], state2: &mut [u32; S]) {
    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return avx2::gimli_x2(state, state2);
        }
    }

    gimli(state);
    gimli(state2);
}
