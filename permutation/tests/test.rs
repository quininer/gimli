use gimli_permutation::S;


const INPUT: [u32; S] = [
    0x00000000, 0x9e3779ba, 0x3c6ef37a, 0xdaa66d46,
    0x78dde724, 0x1715611a, 0xb54cdb2e, 0x53845566,
    0xf1bbcfc8, 0x8ff34a5a, 0x2e2ac522, 0xcc624026,
];
const OUTPUT: [u32; S] = [
    0xba11c85a, 0x91bad119, 0x380ce880, 0xd24c2c68,
    0x3eceffea, 0x277a921c, 0x4f73a0bd, 0xda5a9cd8,
    0x84b673f0, 0x34e52ff7, 0x9e2bef49, 0xf41bb8d6,
];

#[test]
fn test_gimli() {
    let mut data = [0; S];
    data.copy_from_slice(&INPUT);

    gimli_permutation::portable::gimli(&mut data);
    assert_eq!(data, OUTPUT);


    let mut data = [0; S];
    data.copy_from_slice(&INPUT);

    gimli_permutation::gimli(&mut data);
    assert_eq!(data, OUTPUT);
}

#[test]
fn test_gimli_simd256() {
    let mut data = [0; S];
    let mut data2 = [0; S];
    data.copy_from_slice(&INPUT);

    gimli_permutation::gimli_x2(&mut data, &mut data2);
    assert_eq!(data, OUTPUT);


    let mut data = [0; S];
    let mut data2 = [0; S];
    data2.copy_from_slice(&INPUT);

    gimli_permutation::gimli_x2(&mut data, &mut data2);
    assert_eq!(data2, OUTPUT);


    let mut data = [0; S];
    let mut data2 = [0; S];
    data.copy_from_slice(&INPUT);
    data2.copy_from_slice(&INPUT);

    gimli_permutation::gimli_x2(&mut data, &mut data2);
    assert_eq!(data, OUTPUT);
    assert_eq!(data2, OUTPUT);
}
