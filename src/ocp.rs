/// Power amplifier over current protection.
///
/// Used by [`set_pa_ocp`].
///
/// [`set_pa_ocp`]: crate::SubGhz::set_pa_ocp
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum Ocp {
    /// Maximum 60mA current for LP PA mode.
    Max60m = 0x18,
    /// Maximum 140mA for HP PA mode.
    Max140m = 0x38,
}
