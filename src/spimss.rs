#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data8: [u8; 2usize],
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - SPI Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x08 - SPI Status Register."]
    pub status: STATUS,
    #[doc = "0x0c - SPI Mode Register."]
    pub mod_: MOD,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Baud Rate Reload Value. The SPI Baud Rate register is a 16-bit reload value for the SPI Baud Rate Generator. The reload value must be greater than or equal to 0002H for proper SPI operation (maximum baud rate is PCLK frequency divided by 4)."]
    pub brg: BRG,
    #[doc = "0x18 - SPI DMA Register."]
    pub dma: DMA,
    #[doc = "0x1c - I2S Control Register."]
    pub i2s_ctrl: I2S_CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Data 8-bit access"]
    #[inline(always)]
    pub fn data8(&self) -> &[DATA8; 2] {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const [DATA8; 2]) }
    }
    #[doc = "0x00 - SPI Data 8-bit access"]
    #[inline(always)]
    pub fn data8_mut(&self) -> &mut [DATA8; 2] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut [DATA8; 2]) }
    }
    #[doc = "0x00 - SPI 16-bit Data Access"]
    #[inline(always)]
    pub fn data16(&self) -> &DATA16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATA16) }
    }
    #[doc = "0x00 - SPI 16-bit Data Access"]
    #[inline(always)]
    pub fn data16_mut(&self) -> &mut DATA16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATA16) }
    }
}
#[doc = "SPI 16-bit Data Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data16](data16) module"]
pub type DATA16 = crate::Reg<u16, _DATA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA16;
#[doc = "`read()` method returns [data16::R](data16::R) reader structure"]
impl crate::Readable for DATA16 {}
#[doc = "`write(|w| ..)` method takes [data16::W](data16::W) writer structure"]
impl crate::Writable for DATA16 {}
#[doc = "SPI 16-bit Data Access"]
pub mod data16;
#[doc = "SPI Data 8-bit access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8](data8) module"]
pub type DATA8 = crate::Reg<u8, _DATA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA8;
#[doc = "`read()` method returns [data8::R](data8::R) reader structure"]
impl crate::Readable for DATA8 {}
#[doc = "`write(|w| ..)` method takes [data8::W](data8::W) writer structure"]
impl crate::Writable for DATA8 {}
#[doc = "SPI Data 8-bit access"]
pub mod data8;
#[doc = "SPI Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SPI Control Register."]
pub mod ctrl;
#[doc = "SPI Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "SPI Status Register."]
pub mod status;
#[doc = "SPI Mode Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "SPI Mode Register."]
pub mod mod_;
#[doc = "Baud Rate Reload Value. The SPI Baud Rate register is a 16-bit reload value for the SPI Baud Rate Generator. The reload value must be greater than or equal to 0002H for proper SPI operation (maximum baud rate is PCLK frequency divided by 4).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg](brg) module"]
pub type BRG = crate::Reg<u32, _BRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRG;
#[doc = "`read()` method returns [brg::R](brg::R) reader structure"]
impl crate::Readable for BRG {}
#[doc = "`write(|w| ..)` method takes [brg::W](brg::W) writer structure"]
impl crate::Writable for BRG {}
#[doc = "Baud Rate Reload Value. The SPI Baud Rate register is a 16-bit reload value for the SPI Baud Rate Generator. The reload value must be greater than or equal to 0002H for proper SPI operation (maximum baud rate is PCLK frequency divided by 4)."]
pub mod brg;
#[doc = "SPI DMA Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](dma) module"]
pub type DMA = crate::Reg<u32, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "SPI DMA Register."]
pub mod dma;
#[doc = "I2S Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_ctrl](i2s_ctrl) module"]
pub type I2S_CTRL = crate::Reg<u32, _I2S_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CTRL;
#[doc = "`read()` method returns [i2s_ctrl::R](i2s_ctrl::R) reader structure"]
impl crate::Readable for I2S_CTRL {}
#[doc = "`write(|w| ..)` method takes [i2s_ctrl::W](i2s_ctrl::W) writer structure"]
impl crate::Writable for I2S_CTRL {}
#[doc = "I2S Control Register."]
pub mod i2s_ctrl;
