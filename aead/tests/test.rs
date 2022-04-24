use std::io;
use std::str::Lines;
use gimli_aead::GimliAead;


#[test]
fn test_kat() -> io::Result<()> {
    struct Kat {
        count: usize,
        key: [u8; 32],
        nonce: [u8; 16],
        pt: Vec<u8>,
        ad: Vec<u8>,
        ct: Vec<u8>
    }

    impl Kat {
        fn take(lines: &mut Lines) -> Option<Kat> {
            let count = lines.next()?
                .splitn(2, " = ")
                .last()?
                .parse().ok()?;

            let key_buf = lines.next()?
                .splitn(2, " = ")
                .last()?;
            let mut key = [0; 32];
            key.copy_from_slice(&hex::decode(key_buf).ok()?);

            let nonce_buf = lines.next()?
                .splitn(2, " = ")
                .last()?;
            let mut nonce = [0; 16];
            nonce.copy_from_slice(&hex::decode(nonce_buf).ok()?);

            let pt = lines.next()?
                .splitn(2, " = ")
                .last()
                .and_then(|buf| hex::decode(buf).ok())?;
            let ad = lines.next()?
                .splitn(2, " = ")
                .last()
                .and_then(|buf| hex::decode(buf).ok())?;
            let ct = lines.next()?
                .splitn(2, " = ")
                .last()
                .and_then(|buf| hex::decode(buf).ok())?;

            let _ = lines.next()?;

            Some(Kat { count, key, nonce, pt, ad, ct })
        }
    }

    let buf = include_str!("LWC_AEAD_KAT_256_128.txt");
    let mut lines = buf.lines();
    let mut count = 0;

    while let Some(kat) = Kat::take(&mut lines) {
        count += 1;

        let mut buf = kat.pt.clone();

        let tag = GimliAead::new(&kat.key, &kat.nonce)
            .encrypt(&kat.ad, &mut buf);

        buf.extend_from_slice(&tag);

        assert_eq!(count, kat.count);
        assert_eq!(buf, kat.ct);
    }

    assert_eq!(count, 1089);

    Ok(())
}

#[test]
fn test_encrypt_decrypt() {
    let key = [0x55; 32];
    let nonce = [0x44; 16];

    let aad = [0x33; 21];
    let plaintext = vec![0x22; 123];

    let mut output = plaintext.clone();
    let tag = GimliAead::new(&key, &nonce)
        .encrypt(&aad, &mut output);

    assert_ne!(plaintext, output);

    let mut output2 = output.clone();
    let result = GimliAead::new(&key, &nonce)
        .decrypt(&aad, &mut output2, &tag);

    assert!(result);
    assert_eq!(plaintext, output2);

    output[1] ^= 0x1;
    let result = GimliAead::new(&key, &nonce)
        .decrypt(&aad, &mut output, &tag);

    assert!(!result);
}
