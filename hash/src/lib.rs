#![no_std]

extern crate byteorder;
extern crate gimli_permutation;

use core::cmp;
use byteorder::{ ByteOrder, LittleEndian };
use gimli_permutation::{ S, gimli };


pub const RATE: usize = 16;


#[derive(Clone)]
pub struct GimliHash {
    state: [u32; S],
    pos: usize
}

impl Default for GimliHash {
    fn default() -> Self {
        GimliHash { state: [0; S], pos: 0 }
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
    state: [u32; S],
    pos: usize
}

impl XofReader {
    pub fn squeeze(&mut self, buf: &mut [u8]) {
        let XofReader { state, pos } = self;

        let take = cmp::min(RATE - *pos, buf.len());
        let (prefix, buf) = buf.split_at_mut(take);

        if !prefix.is_empty() {
            with(state, |state| {
                prefix.copy_from_slice(&state[*pos..][..take]);
                *pos += take;
            });

            if *pos == RATE {
                gimli(state);
                *pos = 0;
            }
        }

        for chunk in buf.chunks_mut(RATE) {
            let take = chunk.len();
            with(state, |state| {
                chunk.copy_from_slice(&state[*pos..][..take]);
            });

            if *pos == RATE {
                gimli(state);
            } else {
                *pos += take;
            }
        }
    }
}

fn with<F>(state: &mut [u32; S], f: F)
    where F: FnOnce(&mut [u8; S * 4])
{
    #[inline]
    fn transmute(arr: &mut [u32; S]) -> &mut [u8; S * 4] {
        unsafe { &mut *(arr as *mut [u32; S] as *mut [u8; S * 4]) }
    }

    LittleEndian::from_slice_u32(state);
    f(transmute(state));
    LittleEndian::from_slice_u32(state);
}
