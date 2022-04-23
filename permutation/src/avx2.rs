use crate::{ simd256, SIZE };


#[target_feature(enable = "avx2")]
pub unsafe fn gimli_x2(state: &mut [u32; SIZE], state2: &mut [u32; SIZE]) {
    enum Avx2 {}

    simd256::gimli_x2::<Avx2>(state, state2)
}
