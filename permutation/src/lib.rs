#![cfg_attr(not(feature = "simd"), no_std)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(target_arch = "arm", feature(stdsimd, arm_target_feature))]
#![cfg_attr(target_arch = "aarch64", feature(stdsimd, aarch64_target_feature))]

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

pub const SIZE: usize = 12;

pub fn gimli(state: &mut [u32; SIZE]) {
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

pub fn gimli_x2(state: &mut [u32; SIZE], state2: &mut [u32; SIZE]) {
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
