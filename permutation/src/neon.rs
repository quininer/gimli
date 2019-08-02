use crate::{ simd128, S };


#[target_feature(enable = "neon")]
pub unsafe fn gimli(state: &mut [u32; S]) {
    enum Neon {}

    simd128::gimli::<Neon>(state)
}
