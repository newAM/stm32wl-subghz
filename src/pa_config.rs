/// Power amplifier configuration paramters.
///
/// This is an argument of [`set_pa_config`].
///
/// [`set_pa_config`]: crate::SubGhz::set_pa_config
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PaConfig {
    buf: [u8; 5],
}

impl PaConfig {
    /// Create a new `PaConfig` struct.
    ///
    /// This is the same as `default`, but in a `const` function.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::PaConfig;
    ///
    /// const PA_CONFIG: PaConfig = PaConfig::new();
    /// ```
    pub const fn new() -> PaConfig {
        PaConfig {
            buf: [crate::OpCode::SetPaConfig as u8, 0x01, 0x00, 0x01, 0x01],
        }
    }

    /// Set the power amplifier duty cycle (conduit angle) control.
    ///
    /// **Note:** Only the first 3 bits of the `pa_duty_cycle` argument are used.
    ///
    /// Duty cycle = 0.2 + 0.04 × bits
    ///
    /// # Caution
    ///
    /// The following restrictions must be observed to avoid over-stress on the PA:
    /// * LP PA mode with synthesis frequency > 400 MHz, PaDutyCycle must be < 0x7.
    /// * LP PA mode with synthesis frequency < 400 MHz, PaDutyCycle must be < 0x4.
    /// * HP PA mode, PaDutyCycle must be < 0x4
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::{PaConfig, PaSel};
    ///
    /// const PA_CONFIG: PaConfig = PaConfig::new().set_pa(PaSel::Lp).set_pa_duty_cycle(0x4);
    /// # assert_eq!(PA_CONFIG.as_slice()[1], 0x04);
    /// ```
    #[must_use = "set_pa_duty_cycle returns a new PaConfig"]
    pub const fn set_pa_duty_cycle(mut self, pa_duty_cycle: u8) -> PaConfig {
        self.buf[1] = pa_duty_cycle & 0b111;
        self
    }

    /// Set the high power amplifier output power.
    ///
    /// **Note:** Only the first 3 bits of the `hp_max` argument are used.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::{PaConfig, PaSel};
    ///
    /// const PA_CONFIG: PaConfig = PaConfig::new().set_pa(PaSel::Hp).set_hp_max(0x2);
    /// # assert_eq!(PA_CONFIG.as_slice()[2], 0x02);
    /// ```
    #[must_use = "set_hp_max returns a new PaConfig"]
    pub const fn set_hp_max(mut self, hp_max: u8) -> PaConfig {
        self.buf[2] = hp_max & 0b111;
        self
    }

    /// Set the power amplifier to use, low or high power.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::{PaConfig, PaSel};
    ///
    /// const PA_CONFIG_HP: PaConfig = PaConfig::new().set_pa(PaSel::Hp);
    /// const PA_CONFIG_LP: PaConfig = PaConfig::new().set_pa(PaSel::Lp);
    /// # assert_eq!(PA_CONFIG_HP.as_slice()[3], 0x00);
    /// # assert_eq!(PA_CONFIG_LP.as_slice()[3], 0x01);
    /// ```
    #[must_use = "set_pa returns a new PaConfig"]
    pub const fn set_pa(mut self, pa: PaSel) -> PaConfig {
        self.buf[3] = pa as u8;
        self
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::{PaConfig, PaSel};
    ///
    /// const PA_CONFIG: PaConfig = PaConfig::new()
    ///     .set_pa(PaSel::Hp)
    ///     .set_pa_duty_cycle(0x2)
    ///     .set_hp_max(0x3);
    ///
    /// assert_eq!(PA_CONFIG.as_slice(), &[0x95, 0x2, 0x03, 0x00, 0x01]);
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for PaConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Power amplifier selection.
///
/// This is an argument of [`PaConfig::set_pa`].
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PaSel {
    /// High power amplifier.
    Hp = 0b0,
    /// Low power amplifier.
    Lp = 0b1,
}

impl PartialOrd for PaSel {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PaSel {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (PaSel::Hp, PaSel::Hp) | (PaSel::Lp, PaSel::Lp) => core::cmp::Ordering::Equal,
            (PaSel::Hp, PaSel::Lp) => core::cmp::Ordering::Greater,
            (PaSel::Lp, PaSel::Hp) => core::cmp::Ordering::Less,
        }
    }
}

impl Default for PaSel {
    fn default() -> Self {
        PaSel::Lp
    }
}
