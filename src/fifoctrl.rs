use core::fmt;
use embedded_hal_async::i2c::I2c;

use crate::Register;

/// The FIFO_CTRL1 to FIFO_CTRL4 registers
///
/// The four registers are handled as one because values and functionality is split across several
/// registers.
pub struct FifoCtrl {
    pub address: u8,
    value: [u8; 4],
}

impl fmt::Display for FifoCtrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            write!(f, "{}", r)?;
        }

        Ok(())
    }
}

impl fmt::Binary for FifoCtrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            write!(f, "{:b}", r)?;
        }

        Ok(())
    }
}

impl fmt::LowerHex for FifoCtrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            fmt::LowerHex::fmt(&r, f)?;
        }

        Ok(())
    }
}

pub const ADDR: u8 = 0x07_u8;

/// The maximum number of samples in an uncompressed FIFO.
pub const FIFO_SIZE: u16 = 512;

impl Register for FifoCtrl {}

/// FIFO mode
///
/// Default value is `Bypass` (off).
#[repr(u8)]
pub enum FifoMode {
    Bypass = 0b000,
    FifoMode = 0b001,
    ContinuousToFifo = 0b011,
    BypassToContinuous = 0b100,
    Continuous = 0b110,
    BypassToFifo = 0b111,
}

/// Batch data rate of gyroscope.
#[repr(u8)]
pub enum BdrGy {
    Off = 0b0000,
    Hz6_5 = 0b1011,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b100,
    Hz208 = 0b0101,
    Hz417 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

/// Batch data rate of accelerometer.
#[repr(u8)]
pub enum BdrXl {
    Off = 0b0000,
    Hz6_5 = 0b1011,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b100,
    Hz208 = 0b0101,
    Hz417 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

impl FifoCtrl {
    pub fn new(value: [u8; 4], address: u8) -> Self {
        FifoCtrl { address, value }
    }

    /// Enable compression of values in FIFO, increasing FIFO size from 3kB to maximum 9kB.
    pub async fn compression<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.value[1] &= !(1 << 6);
        self.value[1] |= (value as u8) << 6;
        self.write(i2c, self.address, ADDR + 1, self.value[1]).await
    }

    /// Set the FIFO mode (or disable FIFO)
    pub async fn mode<I2C>(&mut self, i2c: &mut I2C, mode: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        const RESET: u8 = 0b111;

        self.value[3] &= !RESET;
        self.value[3] |= mode as u8;
        self.write(i2c, self.address, ADDR + 3, self.value[3]).await
    }

    /// Set the batch data rate for the accelerometer.
    pub async fn set_accelerometer_batch_data_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrXl,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        const RESET: u8 = 0b00001111;
        self.value[2] &= !RESET;
        self.value[2] |= rate as u8;
        self.write(i2c, self.address, ADDR + 2, self.value[2]).await
    }

    /// Set the batch data rate for the gyroscope.
    pub async fn set_gyroscope_batch_data_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrGy,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        const RESET: u8 = 0b11110000;
        self.value[2] &= !RESET;
        self.value[2] |= (rate as u8) << 4;
        self.write(i2c, self.address, ADDR + 2, self.value[2]).await
    }
}
