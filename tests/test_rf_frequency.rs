use subghz::RfFreq;

#[test]
fn max() {
    assert_eq!(RfFreq::from_bits(u32::MAX).freq(), 4_095_999_999);
}

#[test]
fn min() {
    assert_eq!(RfFreq::from_bits(u32::MIN).freq(), 0);
}
