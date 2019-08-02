use crate::{ simd256, S };


#[deprecated(since="0.1.1", note="please use `avx2::gimli_x2` instead")]
#[inline]
pub unsafe fn gimli(state: &mut [u32; S], state2: &mut [u32; S]) {
    gimli_x2(state, state2)
}

#[target_feature(enable = "avx2")]
pub unsafe fn gimli_x2(state: &mut [u32; S], state2: &mut [u32; S]) {
    enum Avx2 {}

    simd256::gimli_x2::<Avx2>(state, state2)
}
