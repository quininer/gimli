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
    #[cfg(target_feature = "ssse3")] {
        if cfg_feature_enabled!("ssse3") {
            unsafe { ssse3::gimli(state) }
        } else {
            portable::gimli(state)
        }
    }

    #[cfg(not(target_feature = "ssse3"))] {
        portable::gimli(state)
    }
}

#[cfg(feature = "simd")]
#[inline]
pub fn gimlix2(state: &mut [u32; BLOCK_LENGTH], state2: &mut [u32; BLOCK_LENGTH]) {
    #[cfg(target_feature = "avx2")] {
        if cfg_feature_enabled!("avx2") {
            unsafe { avx2::gimli(state, state2) }
        } else {
            gimli(state);
            gimli(state2);
        }
    }

    #[cfg(not(target_feature = "avx2"))] {
        gimli(state);
        gimli(state2);
    }
}

#[cfg(not(feature = "simd"))]
#[inline]
pub fn gimlix2(state: &mut [u32; BLOCK_LENGTH], state2: &mut [u32; BLOCK_LENGTH]) {
    gimli(state);
    gimli(state2);
}
