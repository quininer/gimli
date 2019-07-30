use crate::{ simd128, S };


#[deprecated(since="0.1.2", note="please use `gimli` instead")]
#[target_feature(enable = "ssse3")]
pub unsafe fn gimli(state: &mut [u32; S]) {
    simd128::gimli(state)
}
