use ct_uint::{Limb, Uint};
use subtle::ConstantTimeEq;

type U256 = Uint<4>;

#[test]
fn limb_adc_sbb() {
    let (r, c) = Limb::from_u64(u64::MAX).adc(Limb::ONE, Limb::ZERO);
    assert_eq!(r.to_u64(), 0);
    assert_eq!(c.to_u64(), 1);

    let (r, b) = Limb::ZERO.sbb(Limb::ONE, Limb::ZERO);
    assert_eq!(r.to_u64(), u64::MAX);
    assert_eq!(b.to_u64(), 1);
}

#[test]
fn add_sub_roundtrip() {
    let a = U256::from_u64(12345);
    let b = U256::from_u64(67890);
    let (sum, overflow) = a.checked_add(&b);
    assert_eq!(bool::from(overflow), false);
    let (diff, underflow) = sum.checked_sub(&b);
    assert_eq!(bool::from(underflow), false);
    assert!(bool::from(diff.ct_eq(&a)));
}

#[test]
fn widening_mul_small() {
    let a = U256::from_u64(6);
    let b = U256::from_u64(7);
    let (lo, hi) = a.widening_mul(&b);
    assert_eq!(lo.to_u64(), Some(42));
    assert_eq!(hi.to_u64(), Some(0));
}

#[test]
fn widening_mul_full_limb() {
    let a = U256::from_u64(u64::MAX);
    let b = U256::from_u64(u64::MAX);
    let (lo, hi) = a.widening_mul(&b);

    assert_eq!(hi.to_u64(), Some(0));

    let mut lo_bytes = [0u8; 32];
    lo.write_le_bytes(&mut lo_bytes);

    let mut expected = [0u8; 32];
    expected[0] = 1;
    expected[8..16].copy_from_slice(&(u64::MAX - 1).to_le_bytes());

    assert_eq!(lo_bytes, expected);
}

#[test]
fn bytes_roundtrip() {
    let mut buf = [0u8; 32];
    buf[0] = 0xAA;
    buf[31] = 0x01;
    let v = U256::from_le_bytes(&buf).unwrap();
    let mut out = [0u8; 32];
    v.write_le_bytes(&mut out);
    assert_eq!(buf, out);
}
