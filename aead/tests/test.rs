use std::{ io, fs };
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

    let buf = fs::read_to_string("./tests/LWC_AEAD_KAT_256_128.txt")?;
    let mut lines = buf.lines();
    let mut count = 0;

    while let Some(kat) = Kat::take(&mut lines) {
        count += 1;

        let mut buf = kat.pt.clone();
        let mut tag = [0; 16];

        GimliAead::new(&kat.key, &kat.nonce)
            .encrypt(&kat.ad)
            .process(&mut buf, &mut tag);

        buf.extend_from_slice(&tag);

        assert_eq!(count, kat.count);
        assert_eq!(buf, kat.ct);
    }

    assert_eq!(count, 1089);

    Ok(())
}
