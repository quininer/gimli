#![no_std]

use gimli_permutation::{ SIZE, gimli, state_with as with };

#[derive(Clone)]
pub struct GimliAead([u32; SIZE]);

pub struct EncryptProcess(GimliAead);
pub struct DecryptProcess(GimliAead);

impl GimliAead {
    pub fn new(key: &[u8; 32], nonce: &[u8; 16]) -> GimliAead {
        let mut state = [0; SIZE];
        with(&mut state, |state| {
            state[..16].copy_from_slice(nonce);
            state[16..].copy_from_slice(key);
        });
        gimli(&mut state);
        GimliAead(state)
    }

    fn process_aad(&mut self, aad: &[u8]) {
        let state = &mut self.0;

        let mut iter = aad.chunks_exact(16);

        for chunk in &mut iter {
            with(state, |state| {
                for i in 0..16 {
                    state[i] ^= chunk[i];
                }
            });
            gimli(state);
        }

        with(state, |state| {
            let chunk = iter.remainder();
            for i in 0..chunk.len() {
                state[i] ^= chunk[i];
            }

            state[chunk.len()] ^= 1;
            state[47] ^= 1;
        });
        gimli(state);
    }

    pub fn encrypt(mut self, aad: &[u8]) -> EncryptProcess {
        self.process_aad(aad);
        EncryptProcess(self)
    }

    pub fn decrypt(mut self, aad: &[u8]) -> DecryptProcess {
        self.process_aad(aad);
        DecryptProcess(self)
    }
}

impl EncryptProcess {
    pub fn process(mut self, buf: &mut [u8], tag: &mut [u8; 16]) {
        let state = &mut (self.0).0;

        let mut iter = buf.chunks_exact_mut(16);

        for chunk in &mut iter {
            with(state, |state| {
                for i in 0..16 {
                    state[i] ^= chunk[i];
                    chunk[i] = state[i];
                }
            });
            gimli(state);
        }

        with(state, |state| {
            let chunk = iter.into_remainder();
            for i in 0..chunk.len() {
                state[i] ^= chunk[i];
                chunk[i] = state[i];
            }

            state[chunk.len()] ^= 1;
            state[47] ^= 1;
        });
        gimli(state);

        with(state, |state| tag.copy_from_slice(&state[..16]));
    }
}

impl DecryptProcess {
    pub fn process(mut self, buf: &mut [u8], tag: &[u8; 16]) -> bool {
        let state = &mut (self.0).0;

        let mut iter = buf.rchunks_exact_mut(16);

        for chunk in &mut iter {
            with(state, |state| {
                for i in 0..16 {
                    let c = chunk[i];
                    chunk[i] = state[i] ^ c;
                    state[i] = c;
                }
            });
            gimli(state);
        }

        with(state, |state| {
            let chunk = iter.into_remainder();
            for i in 0..chunk.len() {
                let c = chunk[i];
                chunk[i] = state[i] ^ c;
                state[i] = c;
            }

            state[chunk.len()] ^= 1;
            state[47] ^= 1;
        });
        gimli(state);

        let mut result = 0;
        with(state, |state| {
            for i in 0..16 {
                result |= state[i] ^ tag[i];
            }
        });

        result == 0
    }
}
