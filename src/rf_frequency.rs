/// RF frequency structure.
///
/// This is an argument of [`set_rf_frequency`].
///
/// [`set_rf_frequency`]: crate::SubGhz::set_rf_frequency
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct RfFreq {
    buf: [u8; 5],
}

impl RfFreq {
    /// 915MHz, often used in Australia and North America.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// assert_eq!(RfFreq::F915.freq(), 915_000_000);
    /// ```
    pub const F915: RfFreq = RfFreq::from_bits(0x39_30_00_00);

    /// 868MHz, often used in Europe.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// assert_eq!(RfFreq::F868.freq(), 868_000_000);
    /// ```
    pub const F868: RfFreq = RfFreq::from_bits(0x36_40_00_00);

    /// 433MHz, often used in Europe.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// assert_eq!(RfFreq::F433.freq(), 433_000_000);
    /// ```
    pub const F433: RfFreq = RfFreq::from_bits(0x1B_10_00_00);

    /// Create a new `RfFreq` from a raw bit value.
    ///
    /// The equation used to get the PLL frequency from the raw bits is:
    ///
    /// RF<sub>PLL</sub> = 32e6 × bits / 2<sup>25</sup>
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// const FREQ: RfFreq = RfFreq::from_bits(0x39300000);
    /// assert_eq!(FREQ, RfFreq::F915);
    /// ```
    pub const fn from_bits(bits: u32) -> RfFreq {
        RfFreq {
            buf: [
                crate::OpCode::SetRfFrequency as u8,
                ((bits >> 24) & 0xFF) as u8,
                ((bits >> 16) & 0xFF) as u8,
                ((bits >> 8) & 0xFF) as u8,
                (bits & 0xFF) as u8,
            ],
        }
    }

    /// Create a new `RfFreq` from a PLL frequency.
    ///
    /// The equation used to get the raw bits from the PLL frequency is:
    ///
    /// bits = RF<sub>PLL</sub> * 2<sup>25</sup> / 32e6
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// const FREQ: RfFreq = RfFreq::from_frequency(915_000_000);
    /// assert_eq!(FREQ, RfFreq::F915);
    /// ```
    pub const fn from_frequency(freq: u32) -> RfFreq {
        Self::from_bits((((freq as u64) * (1 << 25)) / 32_000_000) as u32)
    }

    // Get the frequency bit value.
    const fn as_bits(&self) -> u32 {
        ((self.buf[1] as u32) << 24)
            | ((self.buf[2] as u32) << 16)
            | ((self.buf[3] as u32) << 8)
            | (self.buf[4] as u32)
    }

    /// Get the actual frequency.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// assert_eq!(RfFreq::from_bits(0x39300000).freq(), 915_000_000);
    /// ```
    pub fn freq(&self) -> u32 {
        (32_000_000 * (self.as_bits() as u64) / (1 << 25)) as u32
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::RfFreq;
    ///
    /// assert_eq!(RfFreq::F915.as_slice(), &[0x86, 0x39, 0x30, 0x00, 0x00]);
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}
