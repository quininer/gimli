#![no_std]

use core::cmp;
use gimli_permutation::{ SIZE, gimli, state_with as with };


const RATE: usize = 16;

#[derive(Clone)]
pub struct GimliHash {
    state: [u32; SIZE],
    pos: usize
}

impl Default for GimliHash {
    fn default() -> Self {
        GimliHash { state: [0; SIZE], pos: 0 }
    }
}

impl GimliHash {
    #[inline]
    pub fn update(&mut self, buf: &[u8]) {
        self.absorb(buf);
    }

    #[inline]
    pub fn finalize(self, buf: &mut [u8]) {
        self.xof().squeeze(buf);
    }

    #[inline]
    pub fn xof(mut self) -> XofReader {
        self.pad();
        XofReader { state: self.state, pos: 0 }
    }

    pub fn fill_block(&mut self) {
        self.pos = 0;
        gimli(&mut self.state);
    }

    fn absorb(&mut self, buf: &[u8]) {
        let GimliHash { state, pos } = self;

        let mut start = 0;
        let mut len = buf.len();

        while len > 0 {
            let take = cmp::min(RATE - *pos, len);

            with(state, |state| {
                for (dst, &src) in state[*pos..][..take].iter_mut()
                    .zip(&buf[start..][..take])
                {
                    *dst ^= src;
                }
                *pos += take;
                start += take;
                len -= take;
            });

            if *pos == RATE {
                gimli(state);
                *pos = 0;
            }
        }
    }

    fn pad(&mut self) {
        let &mut GimliHash { ref mut state, pos } = self;

        with(state, |state| {
            state[pos] ^= 0x1f;
            state[RATE - 1] ^= 0x80;
        });
        gimli(state);
    }
}


pub struct XofReader {
    state: [u32; SIZE],
    pos: usize
}

impl XofReader {
    pub fn squeeze(&mut self, buf: &mut [u8]) {
        let XofReader { state, pos } = self;

        let take = cmp::min(RATE - *pos, buf.len());
        let (prefix, buf) = buf.split_at_mut(take);

        if !prefix.is_empty() {
            with(state, |state| prefix.copy_from_slice(&state[*pos..][..take]));

            *pos += take;
            if *pos == RATE {
                gimli(state);
                *pos = 0;
            }
        }

        let mut iter = buf.chunks_exact_mut(RATE);
        while let Some(chunk) = iter.next() {
            with(state, |state| chunk.copy_from_slice(&state[..RATE]));
            gimli(state);
        }

        let chunk = iter.into_remainder();
        if !chunk.is_empty() {
            let take = chunk.len();
            with(state, |state| chunk.copy_from_slice(&state[..take]));
            *pos += take;
        }
    }
}
