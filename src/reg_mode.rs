/// Radio power supply selection.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum RegMode {
    /// Linear dropout regulator
    Ldo = 0b0,
    /// Switch mode power supply.
    ///
    /// Used in standby with HSE32, FS, RX, and TX modes.
    Smps = 0b1,
}

impl Default for RegMode {
    fn default() -> Self {
        RegMode::Ldo
    }
}
