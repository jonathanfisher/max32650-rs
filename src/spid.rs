#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data8: [u8; 4usize],
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    pub ctrl2: CTRL2,
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    pub ctrl3: CTRL3,
    #[doc = "0x10 - Register for controlling SPI peripheral."]
    pub ctrl4: CTRL4,
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    pub brg_ctrl: BRG_CTRL,
    #[doc = "0x18 - Register for controlling I2C mode."]
    pub i2s_ctrl: I2S_CTRL,
    #[doc = "0x1c - Register for controlling DMA."]
    pub dma: DMA,
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    pub irq: IRQ,
    #[doc = "0x24 - Register for enabling interrupts."]
    pub irqe: IRQE,
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    pub wake: WAKE,
    #[doc = "0x2c - Register for wake up enable."]
    pub wakee: WAKEE,
    #[doc = "0x30 - SPI Status register."]
    pub stat: STAT,
    #[doc = "0x34 - Register to control external memory."]
    pub xmem_ctrl: XMEM_CTRL,
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
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl3](ctrl3) module"]
pub type CTRL3 = crate::Reg<u32, _CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL3;
#[doc = "`read()` method returns [ctrl3::R](ctrl3::R) reader structure"]
impl crate::Readable for CTRL3 {}
#[doc = "`write(|w| ..)` method takes [ctrl3::W](ctrl3::W) writer structure"]
impl crate::Writable for CTRL3 {}
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl3;
#[doc = "Register for controlling SPI peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl4](ctrl4) module"]
pub type CTRL4 = crate::Reg<u32, _CTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL4;
#[doc = "`read()` method returns [ctrl4::R](ctrl4::R) reader structure"]
impl crate::Readable for CTRL4 {}
#[doc = "`write(|w| ..)` method takes [ctrl4::W](ctrl4::W) writer structure"]
impl crate::Writable for CTRL4 {}
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl4;
#[doc = "Register for controlling SPI clock rate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg_ctrl](brg_ctrl) module"]
pub type BRG_CTRL = crate::Reg<u32, _BRG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRG_CTRL;
#[doc = "`read()` method returns [brg_ctrl::R](brg_ctrl::R) reader structure"]
impl crate::Readable for BRG_CTRL {}
#[doc = "`write(|w| ..)` method takes [brg_ctrl::W](brg_ctrl::W) writer structure"]
impl crate::Writable for BRG_CTRL {}
#[doc = "Register for controlling SPI clock rate."]
pub mod brg_ctrl;
#[doc = "Register for controlling I2C mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_ctrl](i2s_ctrl) module"]
pub type I2S_CTRL = crate::Reg<u32, _I2S_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CTRL;
#[doc = "`read()` method returns [i2s_ctrl::R](i2s_ctrl::R) reader structure"]
impl crate::Readable for I2S_CTRL {}
#[doc = "`write(|w| ..)` method takes [i2s_ctrl::W](i2s_ctrl::W) writer structure"]
impl crate::Writable for I2S_CTRL {}
#[doc = "Register for controlling I2C mode."]
pub mod i2s_ctrl;
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
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](irq) module"]
pub type IRQ = crate::Reg<u32, _IRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ;
#[doc = "`read()` method returns [irq::R](irq::R) reader structure"]
impl crate::Readable for IRQ {}
#[doc = "`write(|w| ..)` method takes [irq::W](irq::W) writer structure"]
impl crate::Writable for IRQ {}
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod irq;
#[doc = "Register for enabling interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqe](irqe) module"]
pub type IRQE = crate::Reg<u32, _IRQE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQE;
#[doc = "`read()` method returns [irqe::R](irqe::R) reader structure"]
impl crate::Readable for IRQE {}
#[doc = "`write(|w| ..)` method takes [irqe::W](irqe::W) writer structure"]
impl crate::Writable for IRQE {}
#[doc = "Register for enabling interrupts."]
pub mod irqe;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake](wake) module"]
pub type WAKE = crate::Reg<u32, _WAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE;
#[doc = "`read()` method returns [wake::R](wake::R) reader structure"]
impl crate::Readable for WAKE {}
#[doc = "`write(|w| ..)` method takes [wake::W](wake::W) writer structure"]
impl crate::Writable for WAKE {}
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wake;
#[doc = "Register for wake up enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakee](wakee) module"]
pub type WAKEE = crate::Reg<u32, _WAKEE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEE;
#[doc = "`read()` method returns [wakee::R](wakee::R) reader structure"]
impl crate::Readable for WAKEE {}
#[doc = "`write(|w| ..)` method takes [wakee::W](wakee::W) writer structure"]
impl crate::Writable for WAKEE {}
#[doc = "Register for wake up enable."]
pub mod wakee;
#[doc = "SPI Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "SPI Status register."]
pub mod stat;
#[doc = "Register to control external memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xmem_ctrl](xmem_ctrl) module"]
pub type XMEM_CTRL = crate::Reg<u32, _XMEM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XMEM_CTRL;
#[doc = "`read()` method returns [xmem_ctrl::R](xmem_ctrl::R) reader structure"]
impl crate::Readable for XMEM_CTRL {}
#[doc = "`write(|w| ..)` method takes [xmem_ctrl::W](xmem_ctrl::W) writer structure"]
impl crate::Writable for XMEM_CTRL {}
#[doc = "Register to control external memory."]
pub mod xmem_ctrl;
