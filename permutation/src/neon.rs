use crate::{ simd128, SIZE };


#[target_feature(enable = "neon")]
pub unsafe fn gimli(state: &mut [u32; SIZE]) {
    enum Neon {}

    simd128::gimli::<Neon>(state)
}
