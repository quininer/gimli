#![no_std]
#![cfg_attr(feature = "simd", feature(cfg_target_feature))]

#[cfg(feature = "simd")]
#[macro_use]
extern crate coresimd;

pub mod portable;

#[cfg(feature = "simd")]
#[cfg(target_feature = "ssse3")]
pub mod ssse3;

#[cfg(feature = "simd")]
#[cfg(target_feature = "avx2")]
pub mod avx2;


pub const BLOCK_LENGTH: usize = 12;

#[cfg(not(feature = "simd"))]
pub use portable::gimli;

#[cfg(feature = "simd")]
#[inline]
pub fn gimli(state: &mut [u32; BLOCK_LENGTH]) {
    #[cfg(target_feature = "ssse3")] unsafe {
        if cfg_feature_enabled!("ssse3") {
            return ssse3::gimli(state);
        }
    }

    portable::gimli(state)
}

#[deprecated(since="0.1.1", note="please use `gimli_x2` instead")]
#[inline]
pub fn gimlix2(state: &mut [u32; BLOCK_LENGTH], state2: &mut [u32; BLOCK_LENGTH]) {
    gimli_x2(state, state2)
}

#[cfg(feature = "simd")]
#[inline]
pub fn gimli_x2(state: &mut [u32; BLOCK_LENGTH], state2: &mut [u32; BLOCK_LENGTH]) {
    #[cfg(target_feature = "avx2")] unsafe {
        if cfg_feature_enabled!("avx2") {
            return avx2::gimli_x2(state, state2);
        }
    }

    gimli(state);
    gimli(state2);
}

#[cfg(not(feature = "simd"))]
#[inline]
pub fn gimli_x2(state: &mut [u32; BLOCK_LENGTH], state2: &mut [u32; BLOCK_LENGTH]) {
    gimli(state);
    gimli(state2);
}
