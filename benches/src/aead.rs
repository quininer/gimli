use criterion::{ criterion_main, criterion_group, Criterion, black_box };
use ring::aead::{ CHACHA20_POLY1305, AES_256_GCM, UnboundKey, Nonce, Aad, LessSafeKey };
use gimli_aead::GimliAead;


fn bench_gimli(c: &mut Criterion) {
    c.bench_function("gimli-aead", |b| {
        let key = [0x08; 32];
        let nonce = [0x09; 16];
        let buf = vec![0x10; 4096];

        b.iter(move || {
            let key = black_box(key.clone());
            let nonce = black_box(nonce.clone());
            let mut buf = black_box(buf.clone());

            let tag = GimliAead::new(&key, &nonce)
                .encrypt(&[], &mut buf);

            black_box(buf);
            black_box(tag);
        });
    });
}


fn bench_aesgcm(c: &mut Criterion) {
    c.bench_function("aesgcm", |b| {
        let key = [0x011; 32];
        let nonce = [0x12u8; 12];
        let buf = vec![0x13; 4096];

        b.iter(move || {
            let key = black_box(key.clone());
            let nonce = black_box(nonce.clone());
            let mut buf = black_box(buf.clone());

            let key = UnboundKey::new(&AES_256_GCM, &key).unwrap();
            let nonce = Nonce::assume_unique_for_key(nonce);

            let tag = LessSafeKey::new(key)
                .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                .unwrap();

            black_box(buf);
            let _ = black_box(tag);
        });
    });
}

fn bench_chacha20(c: &mut Criterion) {
    c.bench_function("chacha20", |b| {
        let key = [0x014; 32];
        let nonce = [0x15u8; 12];
        let buf = vec![0x16; 4096];

        b.iter(move || {
            let key = black_box(key.clone());
            let nonce = black_box(nonce.clone());
            let mut buf = black_box(buf.clone());

            let key = UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap();
            let nonce = Nonce::assume_unique_for_key(nonce);

            let tag = LessSafeKey::new(key)
                .seal_in_place_separate_tag(nonce, Aad::empty(), &mut buf)
                .unwrap();

            black_box(buf);
            let _ = black_box(tag);
        });
    });
}

criterion_group!(aead, bench_aesgcm, bench_chacha20, bench_gimli);
criterion_main!(aead);
