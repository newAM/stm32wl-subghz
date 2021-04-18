/// Packet type definition.
///
/// An argument of [`set_packet_type`]
///
/// [`set_packet_type`]: crate::SubGhz::set_packet_type
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PacketType {
    /// FSK (frequency shift keying) generic packet type.
    Fsk = 0,
    /// LoRa (long range) packet type.
    LoRa = 1,
    /// BPSK (binary phase shift keying) packet type.
    Bpsk = 2,
    /// MSK (minimum shift keying) generic packet type.
    Msk = 3,
}

impl PacketType {
    /// Create a new `PacketType` from bits.
    ///
    /// # Example
    ///
    /// ```
    /// use subghz::PacketType;
    ///
    /// assert_eq!(PacketType::from_bits(0), Ok(PacketType::Fsk));
    /// assert_eq!(PacketType::from_bits(1), Ok(PacketType::LoRa));
    /// assert_eq!(PacketType::from_bits(2), Ok(PacketType::Bpsk));
    /// assert_eq!(PacketType::from_bits(3), Ok(PacketType::Msk));
    /// // Other values are reserved
    /// assert_eq!(PacketType::from_bits(4), Err(4));
    /// ```
    pub const fn from_bits(bits: u8) -> Result<PacketType, u8> {
        match bits {
            0 => Ok(PacketType::Fsk),
            1 => Ok(PacketType::LoRa),
            2 => Ok(PacketType::Bpsk),
            3 => Ok(PacketType::Msk),
            _ => Err(bits),
        }
    }
}
