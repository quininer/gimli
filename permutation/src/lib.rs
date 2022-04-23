#![cfg_attr(not(feature = "simd"), no_std)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(target_arch = "arm", feature(stdsimd, arm_target_feature))]
#![cfg_attr(target_arch = "aarch64", feature(stdsimd, aarch64_target_feature))]

pub mod portable;

#[cfg(feature = "simd")]
mod simd128;

#[cfg(feature = "simd")]
mod simd256;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ssse3;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod avx2;

#[cfg(feature = "simd")]
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod neon;

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

#[inline]
pub fn state_with<F>(state: &mut [u32; SIZE], f: F)
    where F: FnOnce(&mut [u8; SIZE * 4])
{
    #[inline]
    fn transmute(arr: &mut [u32; SIZE]) -> &mut [u8; SIZE * 4] {
        // # Safety
        //
        // u32 is always safe to convert to u8x4.
        unsafe { &mut *(arr as *mut [u32; SIZE] as *mut [u8; SIZE * 4]) }
    }

    #[cfg(target_endian = "big")]
    for n in state.iter_mut() {
        *n = n.to_le();
    }

    f(transmute(state));

    #[cfg(target_endian = "big")]
    for n in state.iter_mut() {
        *n = n.to_le();
    }
}

#[cfg(gimli_test)]
pub mod test {
    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub use crate::ssse3::gimli as ssse3_gimli;

    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub use crate::avx2::gimli_x2 as avx2_gimli_x2;

    #[cfg(feature = "simd")]
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub use crate::neon::gimli as neon_gimli;

    #[cfg(feature = "simd")]
    pub use crate::simd128::gimli as simd128_gimli;

    #[cfg(feature = "simd")]
    pub use crate::simd256::gimli_x2 as simd256_gimli_x2;
}
