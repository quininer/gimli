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
    if cfg_feature_enabled!("ssse3") {
        unsafe { ssse3::gimli(state) }
    } else {
        portable::gimli(state)
    }
}
