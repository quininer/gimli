use crate::{ simd256, S };


#[allow(deprecated)]
#[deprecated(since="0.1.1", note="please use `avx2::gimli_x2` instead")]
#[inline]
pub unsafe fn gimli(state: &mut [u32; S], state2: &mut [u32; S]) {
    gimli_x2(state, state2)
}

#[deprecated(since="0.1.2", note="please use `gimli_x2` instead")]
#[target_feature(enable = "avx2")]
pub unsafe fn gimli_x2(state: &mut [u32; S], state2: &mut [u32; S]) {
    simd256::gimli_x2(state, state2)
}
