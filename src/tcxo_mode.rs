use crate::Timeout;

/// TCXO trim.
///
/// **Note:** To use V<sub>DDTCXO</sub>, the V<sub>DDRF</sub> supply must be at
/// least + 200 mV higher than the selected `TcxoTrim` voltage level.
///
/// Used by [`TcxoMode`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
pub enum TcxoTrim {
    /// 1.6V
    Volts1pt6 = 0x0,
    /// 1.7V
    Volts1pt7 = 0x1,
    /// 1.8V
    Volts1pt8 = 0x2,
    /// 2.2V
    Volts2pt2 = 0x3,
    /// 2.4V
    Volts2pt4 = 0x4,
    /// 2.7V
    Volts2pt7 = 0x5,
    /// 3.0V
    Volts3pt0 = 0x6,
    /// 3.3V
    Volts3pt3 = 0x7,
}

impl core::fmt::Display for TcxoTrim {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TcxoTrim::Volts1pt6 => write!(f, "1.6V"),
            TcxoTrim::Volts1pt7 => write!(f, "1.7V"),
            TcxoTrim::Volts1pt8 => write!(f, "1.8V"),
            TcxoTrim::Volts2pt2 => write!(f, "2.2V"),
            TcxoTrim::Volts2pt4 => write!(f, "2.4V"),
            TcxoTrim::Volts2pt7 => write!(f, "2.7V"),
            TcxoTrim::Volts3pt0 => write!(f, "3.0V"),
            TcxoTrim::Volts3pt3 => write!(f, "3.3V"),
        }
    }
}

impl TcxoTrim {
    /// Get the value of the TXCO trim in millivolts.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::TcxoTrim;
    ///
    /// assert_eq!(TcxoTrim::Volts1pt6.as_millivolts(), 1600);
    /// assert_eq!(TcxoTrim::Volts1pt7.as_millivolts(), 1700);
    /// assert_eq!(TcxoTrim::Volts1pt8.as_millivolts(), 1800);
    /// assert_eq!(TcxoTrim::Volts2pt2.as_millivolts(), 2200);
    /// assert_eq!(TcxoTrim::Volts2pt4.as_millivolts(), 2400);
    /// assert_eq!(TcxoTrim::Volts2pt7.as_millivolts(), 2700);
    /// assert_eq!(TcxoTrim::Volts3pt0.as_millivolts(), 3000);
    /// assert_eq!(TcxoTrim::Volts3pt3.as_millivolts(), 3300);
    /// ```
    pub const fn as_millivolts(&self) -> u16 {
        match self {
            TcxoTrim::Volts1pt6 => 1600,
            TcxoTrim::Volts1pt7 => 1700,
            TcxoTrim::Volts1pt8 => 1800,
            TcxoTrim::Volts2pt2 => 2200,
            TcxoTrim::Volts2pt4 => 2400,
            TcxoTrim::Volts2pt7 => 2700,
            TcxoTrim::Volts3pt0 => 3000,
            TcxoTrim::Volts3pt3 => 3300,
        }
    }
}

/// TCXO trim and HSE32 ready timeout.
///
/// This is an argument of [`set_tcxo_mode`].
///
/// [`set_tcxo_mode`]: crate::SubGhz::set_tcxo_mode
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TcxoMode {
    buf: [u8; 5],
}

impl TcxoMode {
    /// Create a new `TcxoMode` struct.
    ///
    /// This is the same as `default`, but in a `const` function.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::TcxoMode;
    ///
    /// const TCXO_MODE: TcxoMode = TcxoMode::new();
    /// ```
    pub const fn new() -> TcxoMode {
        TcxoMode {
            buf: [crate::OpCode::SetTcxoMode as u8, 0x00, 0x00, 0x00, 0x00],
        }
    }

    /// Set the TCXO trim.
    ///
    /// **Note:** To use V<sub>DDTCXO</sub>, the V<sub>DDRF</sub> supply must be
    /// at least + 200 mV higher than the selected `TcxoTrim` voltage level.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::{TcxoMode, TcxoTrim};
    ///
    /// const TCXO_MODE: TcxoMode = TcxoMode::new().set_txco_trim(TcxoTrim::Volts1pt6);
    /// # assert_eq!(TCXO_MODE.as_slice()[1], 0x00);
    /// ```
    #[must_use = "set_txco_trim returns a new TcxoMode"]
    pub const fn set_txco_trim(mut self, tcxo_trim: TcxoTrim) -> TcxoMode {
        self.buf[1] = tcxo_trim as u8;
        self
    }

    /// Set the ready timeout duration.
    ///
    /// # Example
    ///
    /// ```
    /// use core::time::Duration;
    /// use subghz::{TcxoMode, Timeout};
    ///
    /// // 15.625 ms timeout
    /// const TIMEOUT: Timeout = Timeout::from_duration_sat(Duration::from_millis(15_625));
    /// const TCXO_MODE: TcxoMode = TcxoMode::new().set_timeout(&TIMEOUT);
    /// # assert_eq!(TCXO_MODE.as_slice()[2], 0x0F);
    /// # assert_eq!(TCXO_MODE.as_slice()[3], 0x42);
    /// # assert_eq!(TCXO_MODE.as_slice()[4], 0x40);
    /// ```
    #[must_use = "set_timeout returns a new TcxoMode"]
    pub const fn set_timeout(mut self, timeout: &Timeout) -> TcxoMode {
        let timeout_bits: u32 = timeout.as_bits();
        self.buf[2] = ((timeout_bits >> 16) & 0xFF) as u8;
        self.buf[3] = ((timeout_bits >> 8) & 0xFF) as u8;
        self.buf[4] = (timeout_bits & 0xFF) as u8;
        self
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::{TcxoMode, TcxoTrim, Timeout};
    ///
    /// const TCXO_MODE: TcxoMode = TcxoMode::new()
    ///     .set_txco_trim(TcxoTrim::Volts1pt7)
    ///     .set_timeout(&Timeout::from_bits(0x123456));
    /// assert_eq!(TCXO_MODE.as_slice(), &[0x97, 0x1, 0x12, 0x34, 0x56]);
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for TcxoMode {
    fn default() -> Self {
        Self::new()
    }
}
