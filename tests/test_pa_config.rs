use subghz::PaSel;

#[test]
fn pa_sel_ord() {
    assert!(PaSel::Lp < PaSel::Hp);
    assert!(PaSel::Hp > PaSel::Lp);
}
