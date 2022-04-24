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
        let take = cmp::min(RATE - self.pos, buf.len());
        let (prefix, buf) = buf.split_at(take);

        if !prefix.is_empty() {
            let pos = self.pos;
            with(&mut self.state, |state| {
                for (dest, src) in state.iter_mut()
                    .skip(pos)
                    .zip(prefix)
                {
                    *dest ^= *src;
                }
            });

            self.pos += prefix.len();

            if self.pos == RATE {
                gimli(&mut self.state);
                self.pos = 0;
            }
        }

        let mut iter = buf.chunks_exact(RATE);
        for chunk in &mut iter {
            with(&mut self.state, |state| {
                for (dest, src) in state.iter_mut().zip(chunk) {
                    *dest ^= *src;
                }
            });
            gimli(&mut self.state);
        }

        let chunk = iter.remainder();
        if !chunk.is_empty() {
            with(&mut self.state, |state| {
                for (dest, src) in state.iter_mut().zip(chunk) {
                    *dest ^= *src;
                }
            });
            self.pos += chunk.len();
        }
    }

    fn pad(&mut self) {
        let pos = self.pos;
        with(&mut self.state, |state| {
            state[pos] ^= 0x1f;
            state[RATE - 1] ^= 0x80;
        });
        gimli(&mut self.state);
    }
}


pub struct XofReader {
    state: [u32; SIZE],
    pos: usize
}

impl XofReader {
    pub fn squeeze(&mut self, buf: &mut [u8]) {
        let take = cmp::min(RATE - self.pos, buf.len());
        let (prefix, buf) = buf.split_at_mut(take);

        if !prefix.is_empty() {
            let pos = self.pos;
            with(&mut self.state, |state| {
                prefix.copy_from_slice(&state[pos..][..prefix.len()]);
            });

            self.pos += prefix.len();

            if self.pos == RATE {
                gimli(&mut self.state);
                self.pos = 0;
            }
        }

        let mut iter = buf.chunks_exact_mut(RATE);
        for chunk in &mut iter {
            with(&mut self.state, |state| chunk.copy_from_slice(&state[..RATE]));
            gimli(&mut self.state);
        }

        let chunk = iter.into_remainder();
        if !chunk.is_empty() {
            with(&mut self.state, |state| {
                chunk.copy_from_slice(&state[..chunk.len()]);
            });
            self.pos += take;
        }
    }
}
