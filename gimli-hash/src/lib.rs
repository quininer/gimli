#![no_std]

extern crate gimli_permutation;

use core::{ cmp, mem };
use gimli_permutation::{ BLOCK_LENGTH, gimli };


pub const RATE: usize = 16;

pub struct GimliHash {
    state: [u8; BLOCK_LENGTH * 4],
    pos: usize
}

impl Default for GimliHash {
    fn default() -> Self {
        GimliHash { state: [0; BLOCK_LENGTH * 4], pos: 0 }
    }
}

impl Clone for GimliHash {
    fn clone(&self) -> Self {
        let mut gimli = GimliHash::default();
        gimli.state.copy_from_slice(&self.state);
        gimli.pos = self.pos;
        gimli
    }
}

impl GimliHash {
    #[inline]
    pub fn input(&mut self, buf: &[u8]) {
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

    fn absorb(&mut self, buf: &[u8]) {
        let mut start = 0;
        let mut len = buf.len();

        while len > 0 {
            let take = cmp::min(RATE - self.pos, len);
            for _ in 0..take {
                self.state[self.pos] ^= buf[start];
                self.pos += 1;
                start += 1;
                len -= 1;
            }

            if self.pos == RATE {
                gimli(array_to_block(&mut self.state));
                self.pos = 0;
            }
        }
    }

    fn pad(&mut self) {
        self.state[self.pos] ^= 0x1f;
        self.state[RATE - 1] ^= 0x80;
        gimli(array_to_block(&mut self.state));
    }
}


pub struct XofReader {
    state: [u8; BLOCK_LENGTH * 4],
    pos: usize
}

impl XofReader {
    pub fn squeeze(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(RATE) {
            let len = chunk.len();
            chunk.copy_from_slice(&self.state[self.pos..][..len]);
            self.pos += len;

            if self.pos == RATE {
                gimli(array_to_block(&mut self.state));
                self.pos = 0;
            }
        }
    }
}



#[inline]
fn array_to_block(arr: &mut [u8; BLOCK_LENGTH * 4]) -> &mut [u32; BLOCK_LENGTH] {
    unsafe { mem::transmute(arr) }
}
