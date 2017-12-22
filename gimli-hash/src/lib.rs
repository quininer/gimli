#![no_std]

extern crate byteorder;
extern crate gimli_permutation;

use core::{ cmp, mem };
use byteorder::{ ByteOrder, LittleEndian };
use gimli_permutation::{ BLOCK_LENGTH, gimli };


pub const RATE: usize = 16;
type State = [u8; BLOCK_LENGTH * 4];


#[derive(Clone)]
pub struct GimliHash {
    state: State,
    pos: usize
}

impl Default for GimliHash {
    fn default() -> Self {
        GimliHash { state: [0; BLOCK_LENGTH * 4], pos: 0 }
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

    #[inline]
    fn gimli(state: &mut State) {
        #[inline]
        fn array_as_block(arr: &mut [u8; BLOCK_LENGTH * 4]) -> &mut [u32; BLOCK_LENGTH] {
            unsafe { mem::transmute(arr) }
        }

        let state = array_as_block(state);
        LittleEndian::from_slice_u32(state);
        gimli(state);
        LittleEndian::from_slice_u32(state);
    }

    fn absorb(&mut self, buf: &[u8]) {
        let mut start = 0;
        let mut len = buf.len();

        while len > 0 {
            let take = cmp::min(RATE - self.pos, len);
            for (dst, &src) in self.state[self.pos..][..take].iter_mut()
                .zip(&buf[start..][..take])
            {
                *dst ^= src;
            }
            self.pos += take;
            start += take;
            len -= take;

            if self.pos == RATE {
                Self::gimli(&mut self.state);
                self.pos = 0;
            }
        }
    }

    fn pad(&mut self) {
        self.state[self.pos] ^= 0x1f;
        self.state[RATE - 1] ^= 0x80;
        Self::gimli(&mut self.state);
    }
}


pub struct XofReader {
    state: State,
    pos: usize
}

impl XofReader {
    pub fn squeeze(&mut self, buf: &mut [u8]) {
        for chunk in buf.chunks_mut(RATE) {
            let len = chunk.len();
            chunk.copy_from_slice(&self.state[self.pos..][..len]);
            self.pos += len;

            if self.pos == RATE {
                GimliHash::gimli(&mut self.state);
                self.pos = 0;
            }
        }
    }
}
