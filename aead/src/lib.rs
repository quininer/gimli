use byteorder::{ ByteOrder, LittleEndian };
use gimli_permutation::{ S, gimli };


#[derive(Clone)]
pub struct GimliAead([u32; S]);

pub struct Encrypt;
pub struct Decrypt;

impl GimliAead {
    pub fn new(key: &[u8; 32], nonce: &[u8; 16]) -> GimliAead {
        let mut state = [0; S];
        with(&mut state, |state| {
            state[..16].copy_from_slice(nonce);
            state[16..].copy_from_slice(key);
        });
        gimli(&mut state);
        GimliAead(state)
    }

    #[inline]
    fn process_aad<M>(self, aad: &[u8], mode: M) -> Process<M> {
        let GimliAead(mut state) = self;

        let mut iter = aad.chunks_exact(16);

        while let Some(chunk) = iter.next() {
            with(&mut state, |state| {
                for i in 0..16 {
                    state[i] ^= chunk[i];
                }
            });
            gimli(&mut state);
        }

        with(&mut state, |state| {
            let chunk = iter.remainder();
            for i in 0..chunk.len() {
                state[i] ^= chunk[i];
            }

            state[chunk.len()] ^= 1;
            state[47] ^= 1;
        });
        gimli(&mut state);

        Process { state, _mode: mode }
    }

    pub fn encrypt(self, aad: &[u8]) -> Process<Encrypt> {
        self.process_aad(aad, Encrypt)
    }

    pub fn decrypt(self, aad: &[u8]) -> Process<Decrypt> {
        self.process_aad(aad, Decrypt)
    }
}

pub struct Process<M> {
    state: [u32; S],
    _mode: M
}

impl Process<Encrypt> {
    pub fn process(self, buf: &mut [u8], tag: &mut [u8; 16]) {
        let Process { mut state, .. } = self;

        let mut iter = buf.chunks_exact_mut(16);

        while let Some(chunk) = iter.next() {
            with(&mut state, |state| {
                for i in 0..16 {
                    state[i] ^= chunk[i];
                    chunk[i] = state[i];
                }
            });
            gimli(&mut state);
        }

        with(&mut state, |state| {
            let chunk = iter.into_remainder();
            for i in 0..chunk.len() {
                state[i] ^= chunk[i];
                chunk[i] = state[i];
            }

            state[chunk.len()] ^= 1;
            state[47] ^= 1;
        });
        gimli(&mut state);

        with(&mut state, |state| tag.copy_from_slice(&state[..16]));
    }
}

impl Process<Decrypt> {
    pub fn process(self, buf: &mut [u8], tag: &[u8; 16]) -> bool {
        let Process { mut state, .. } = self;

        let mut iter = buf.rchunks_exact_mut(16);

        while let Some(chunk) = iter.next() {
            with(&mut state, |state| {
                for i in 0..16 {
                    let c = chunk[i];
                    chunk[i] = state[i] ^ c;
                    state[i] = c;
                }
            });
            gimli(&mut state);
        }

        with(&mut state, |state| {
            let chunk = iter.into_remainder();
            for i in 0..chunk.len() {
                let c = chunk[i];
                chunk[i] = state[i] ^ c;
                state[i] = c;
            }

            state[chunk.len()] ^= 1;
            state[47] ^= 1;
        });
        gimli(&mut state);

        let mut result = 0;
        with(&mut state, |state| {
            for i in 0..16 {
                result |= state[i] ^ tag[i];
            }
        });

        result == 0
    }
}


#[inline]
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
