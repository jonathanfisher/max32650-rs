#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data8: [u8; 4usize],
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    pub ctrl0: CTRL0,
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    pub ctrl1: CTRL1,
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    pub ctrl2: CTRL2,
    #[doc = "0x10 - Register for controlling SPI peripheral/Slave Select Timing."]
    pub ss_time: SS_TIME,
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    pub clk_cfg: CLK_CFG,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Register for controlling DMA."]
    pub dma: DMA,
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    pub int_fl: INT_FL,
    #[doc = "0x24 - Register for enabling interrupts."]
    pub int_en: INT_EN,
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    pub wake_fl: WAKE_FL,
    #[doc = "0x2c - Register for wake up enable."]
    pub wake_en: WAKE_EN,
    #[doc = "0x30 - SPI Status register."]
    pub stat: STAT,
}
impl RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data8(&self) -> &[DATA8; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const [DATA8; 4]) }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data8_mut(&self) -> &mut [DATA8; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut [DATA8; 4]) }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data16(&self) -> &[DATA16; 2] {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const [DATA16; 2]) }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data16_mut(&self) -> &mut [DATA16; 2] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut [DATA16; 2]) }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data32(&self) -> &DATA32 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATA32) }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn data32_mut(&self) -> &mut DATA32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATA32) }
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data32](data32) module"]
pub type DATA32 = crate::Reg<u32, _DATA32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA32;
#[doc = "`read()` method returns [data32::R](data32::R) reader structure"]
impl crate::Readable for DATA32 {}
#[doc = "`write(|w| ..)` method takes [data32::W](data32::W) writer structure"]
impl crate::Writable for DATA32 {}
#[doc = "Register for reading and writing the FIFO."]
pub mod data32;
#[doc = "Register for reading and writing the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data16](data16) module"]
pub type DATA16 = crate::Reg<u16, _DATA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA16;
#[doc = "`read()` method returns [data16::R](data16::R) reader structure"]
impl crate::Readable for DATA16 {}
#[doc = "`write(|w| ..)` method takes [data16::W](data16::W) writer structure"]
impl crate::Writable for DATA16 {}
#[doc = "Register for reading and writing the FIFO."]
pub mod data16;
#[doc = "Register for reading and writing the FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8](data8) module"]
pub type DATA8 = crate::Reg<u8, _DATA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA8;
#[doc = "`read()` method returns [data8::R](data8::R) reader structure"]
impl crate::Readable for DATA8 {}
#[doc = "`write(|w| ..)` method takes [data8::W](data8::W) writer structure"]
impl crate::Writable for DATA8 {}
#[doc = "Register for reading and writing the FIFO."]
pub mod data8;
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u32, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl0;
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl1;
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl2;
#[doc = "Register for controlling SPI peripheral/Slave Select Timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss_time](ss_time) module"]
pub type SS_TIME = crate::Reg<u32, _SS_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS_TIME;
#[doc = "`read()` method returns [ss_time::R](ss_time::R) reader structure"]
impl crate::Readable for SS_TIME {}
#[doc = "`write(|w| ..)` method takes [ss_time::W](ss_time::W) writer structure"]
impl crate::Writable for SS_TIME {}
#[doc = "Register for controlling SPI peripheral/Slave Select Timing."]
pub mod ss_time;
#[doc = "Register for controlling SPI clock rate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg](clk_cfg) module"]
pub type CLK_CFG = crate::Reg<u32, _CLK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_CFG;
#[doc = "`read()` method returns [clk_cfg::R](clk_cfg::R) reader structure"]
impl crate::Readable for CLK_CFG {}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](clk_cfg::W) writer structure"]
impl crate::Writable for CLK_CFG {}
#[doc = "Register for controlling SPI clock rate."]
pub mod clk_cfg;
#[doc = "Register for controlling DMA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](dma) module"]
pub type DMA = crate::Reg<u32, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "Register for controlling DMA."]
pub mod dma;
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_fl](int_fl) module"]
pub type INT_FL = crate::Reg<u32, _INT_FL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_FL;
#[doc = "`read()` method returns [int_fl::R](int_fl::R) reader structure"]
impl crate::Readable for INT_FL {}
#[doc = "`write(|w| ..)` method takes [int_fl::W](int_fl::W) writer structure"]
impl crate::Writable for INT_FL {}
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod int_fl;
#[doc = "Register for enabling interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u32, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "Register for enabling interrupts."]
pub mod int_en;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_fl](wake_fl) module"]
pub type WAKE_FL = crate::Reg<u32, _WAKE_FL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_FL;
#[doc = "`read()` method returns [wake_fl::R](wake_fl::R) reader structure"]
impl crate::Readable for WAKE_FL {}
#[doc = "`write(|w| ..)` method takes [wake_fl::W](wake_fl::W) writer structure"]
impl crate::Writable for WAKE_FL {}
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wake_fl;
#[doc = "Register for wake up enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en](wake_en) module"]
pub type WAKE_EN = crate::Reg<u32, _WAKE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_EN;
#[doc = "`read()` method returns [wake_en::R](wake_en::R) reader structure"]
impl crate::Readable for WAKE_EN {}
#[doc = "`write(|w| ..)` method takes [wake_en::W](wake_en::W) writer structure"]
impl crate::Writable for WAKE_EN {}
#[doc = "Register for wake up enable."]
pub mod wake_en;
#[doc = "SPI Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "SPI Status register."]
pub mod stat;
