use crate::{ simd128, S };


#[target_feature(enable = "ssse3")]
pub unsafe fn gimli(state: &mut [u32; S]) {
    enum Ssse3 {}

    simd128::gimli::<Ssse3>(state)
}
