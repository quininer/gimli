#![cfg_attr(not(feature = "simd"), no_std)]
#![cfg_attr(target_arch = "arm", feature(stdsimd, arm_target_feature))]
#![cfg_attr(target_arch = "aarch64", feature(stdsimd, aarch64_target_feature))]

extern crate core;

pub mod portable;

#[cfg(feature = "simd")]
pub mod simd128;

#[cfg(feature = "simd")]
pub mod simd256;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod ssse3;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod neon;

pub const S: usize = 12;

#[deprecated(since="0.1.1", note="please use `S` instead")]
pub const BLOCK_LENGTH: usize = S;

pub fn gimli(state: &mut [u32; S]) {
    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("ssse3") {
            return ssse3::gimli(state);
        }
    }

    #[cfg(feature = "simd")]
    #[cfg(target_arch = "arm")]
    unsafe {
        if is_arm_feature_detected!("neon") {
            return neon::gimli(state);
        }
    }

    #[cfg(feature = "simd")]
    #[cfg(target_arch = "aarch64")]
    unsafe {
        if is_aarch64_feature_detected!("neon") {
            return neon::gimli(state);
        }
    }

    #[cfg(feature = "simd")] {
        simd128::gimli::<()>(state)
    }

    #[cfg(not(feature = "simd"))] {
        portable::gimli(state)
    }
}

#[deprecated(since="0.1.1", note="please use `gimli_x2` instead")]
#[inline]
pub fn gimlix2(state: &mut [u32; S], state2: &mut [u32; S]) {
    gimli_x2(state, state2)
}

pub fn gimli_x2(state: &mut [u32; S], state2: &mut [u32; S]) {
    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return avx2::gimli_x2(state, state2);
        }
    }

    #[cfg(feature = "simd")] {
        simd256::gimli_x2::<()>(state, state2);
    }

    #[cfg(not(feature = "simd"))] {
        gimli(state);
        gimli(state2);
    }
}
