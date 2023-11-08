use crate::base32::encode;

#[test]
fn test_encoding() {
    let mut dst = [0u8; 18];
    let src = b"hello world";
    encode(&mut dst, src);
    assert_eq!(&dst, b"nbswy3dpeb3w64tmmq");
}
