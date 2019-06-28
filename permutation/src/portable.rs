use core::ops::Range;
use crate::S;


pub fn gimli(state: &mut [u32; S]) {
    for round in R(24..0) {
        // SP-box
        for column in 0..4 {
            let x = state[column    ].rotate_left(24);
            let y = state[column + 4].rotate_left(9);
            let z = state[column + 8];

            state[column + 8] = x ^ (z << 1) ^ ((y & z) << 2);
            state[column + 4] = y ^ x        ^ ((x | z) << 1);
            state[column]     = z ^ y        ^ ((x & y) << 3);
        }

        // linear layer
        match round % 4 {
            0 => {
                // Small-Swap
                state.swap(0, 1);
                state.swap(2, 3);

                // Add constant
                state[0] ^= 0x9e37_7900 ^ round;
            },
            2 => {
                // Big-Swap
                state.swap(0, 2);
                state.swap(1, 3);
            },
            _ => ()
        }
    }
}


struct R(pub Range<u32>);

impl Iterator for R {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.start <= self.0.end {
            None
        } else {
            let i = self.0.start;
            self.0.start -= 1;
            Some(i)
        }
    }
}
