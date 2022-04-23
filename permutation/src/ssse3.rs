use crate::{ simd128, SIZE };

#[target_feature(enable = "ssse3")]
pub unsafe fn gimli(state: &mut [u32; SIZE]) {
    enum Ssse3 {}

    simd128::gimli::<Ssse3>(state)
}
