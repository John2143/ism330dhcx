use embedded_hal_async::i2c::I2c;

use crate::Register;

/// The FIFO_STATUS registers.
pub struct FifoStatus {
    pub address: u8,
}

pub const ADDR: u8 = 0x3a_u8;

impl Register for FifoStatus {}

impl FifoStatus {
    pub fn new(address: u8) -> Self {
        FifoStatus { address }
    }

    /// Is the FIFO full
    pub async fn full<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1).await?;

        Ok(v & (1 << 5) != 0)
    }

    /// Is the FIFO overrun
    pub async fn overrun<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1).await?;

        Ok(v & (1 << 6) != 0)
    }

    /// Is the FIFO watermark reached.
    pub async fn watermark_reached<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1).await?;

        Ok(v & (1 << 7) != 0)
    }

    /// Latched FIFO overrun status.
    pub async fn overrun_latched<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1).await?;

        Ok(v & (1 << 3) != 0)
    }

    /// Counter BDR reached.
    pub async fn count_bdr_reached<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1).await?;

        Ok(v & (1 << 4) != 0)
    }

    /// Number of unread sensor data in FIFO.
    pub async fn diff_fifo<I2C>(&mut self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut v = [0u8; 2];
        i2c.write_read(self.address, &[ADDR], &mut v).await?;
        v[1] &= 0b11;

        Ok(u16::from_le_bytes(v))
    }
}
