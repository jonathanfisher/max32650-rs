#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_tx_: [u8; 4usize],
    _reserved_1_rx_: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x00 - SPI TX FIFO 32-Bit Write"]
    #[inline(always)]
    pub fn tx_32(&self) -> &TX_32 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const TX_32) }
    }
    #[doc = "0x00 - SPI TX FIFO 32-Bit Write"]
    #[inline(always)]
    pub fn tx_32_mut(&self) -> &mut TX_32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut TX_32) }
    }
    #[doc = "0x00 - SPI TX FIFO 16-Bit Write"]
    #[inline(always)]
    pub fn tx_16(&self) -> &TX_16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const TX_16) }
    }
    #[doc = "0x00 - SPI TX FIFO 16-Bit Write"]
    #[inline(always)]
    pub fn tx_16_mut(&self) -> &mut TX_16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut TX_16) }
    }
    #[doc = "0x00 - SPI TX FIFO 8-Bit Write"]
    #[inline(always)]
    pub fn tx_8(&self) -> &TX_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const TX_8) }
    }
    #[doc = "0x00 - SPI TX FIFO 8-Bit Write"]
    #[inline(always)]
    pub fn tx_8_mut(&self) -> &mut TX_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut TX_8) }
    }
    #[doc = "0x04 - SPI RX FIFO 32-Bit Access"]
    #[inline(always)]
    pub fn rx_32(&self) -> &RX_32 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RX_32) }
    }
    #[doc = "0x04 - SPI RX FIFO 32-Bit Access"]
    #[inline(always)]
    pub fn rx_32_mut(&self) -> &mut RX_32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut RX_32) }
    }
    #[doc = "0x04 - SPI RX FIFO 16-Bit Access"]
    #[inline(always)]
    pub fn rx_16(&self) -> &RX_16 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RX_16) }
    }
    #[doc = "0x04 - SPI RX FIFO 16-Bit Access"]
    #[inline(always)]
    pub fn rx_16_mut(&self) -> &mut RX_16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut RX_16) }
    }
    #[doc = "0x04 - SPI RX FIFO 8-Bit Access"]
    #[inline(always)]
    pub fn rx_8(&self) -> &RX_8 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const RX_8) }
    }
    #[doc = "0x04 - SPI RX FIFO 8-Bit Access"]
    #[inline(always)]
    pub fn rx_8_mut(&self) -> &mut RX_8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut RX_8) }
    }
}
#[doc = "SPI TX FIFO 8-Bit Write\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_8](tx_8) module"]
pub type TX_8 = crate::Reg<u8, _TX_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_8;
#[doc = "`read()` method returns [tx_8::R](tx_8::R) reader structure"]
impl crate::Readable for TX_8 {}
#[doc = "`write(|w| ..)` method takes [tx_8::W](tx_8::W) writer structure"]
impl crate::Writable for TX_8 {}
#[doc = "SPI TX FIFO 8-Bit Write"]
pub mod tx_8;
#[doc = "SPI TX FIFO 16-Bit Write\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_16](tx_16) module"]
pub type TX_16 = crate::Reg<u16, _TX_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_16;
#[doc = "`read()` method returns [tx_16::R](tx_16::R) reader structure"]
impl crate::Readable for TX_16 {}
#[doc = "`write(|w| ..)` method takes [tx_16::W](tx_16::W) writer structure"]
impl crate::Writable for TX_16 {}
#[doc = "SPI TX FIFO 16-Bit Write"]
pub mod tx_16;
#[doc = "SPI TX FIFO 32-Bit Write\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_32](tx_32) module"]
pub type TX_32 = crate::Reg<u32, _TX_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_32;
#[doc = "`read()` method returns [tx_32::R](tx_32::R) reader structure"]
impl crate::Readable for TX_32 {}
#[doc = "`write(|w| ..)` method takes [tx_32::W](tx_32::W) writer structure"]
impl crate::Writable for TX_32 {}
#[doc = "SPI TX FIFO 32-Bit Write"]
pub mod tx_32;
#[doc = "SPI RX FIFO 8-Bit Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_8](rx_8) module"]
pub type RX_8 = crate::Reg<u8, _RX_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_8;
#[doc = "`read()` method returns [rx_8::R](rx_8::R) reader structure"]
impl crate::Readable for RX_8 {}
#[doc = "`write(|w| ..)` method takes [rx_8::W](rx_8::W) writer structure"]
impl crate::Writable for RX_8 {}
#[doc = "SPI RX FIFO 8-Bit Access"]
pub mod rx_8;
#[doc = "SPI RX FIFO 16-Bit Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_16](rx_16) module"]
pub type RX_16 = crate::Reg<u16, _RX_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_16;
#[doc = "`read()` method returns [rx_16::R](rx_16::R) reader structure"]
impl crate::Readable for RX_16 {}
#[doc = "`write(|w| ..)` method takes [rx_16::W](rx_16::W) writer structure"]
impl crate::Writable for RX_16 {}
#[doc = "SPI RX FIFO 16-Bit Access"]
pub mod rx_16;
#[doc = "SPI RX FIFO 32-Bit Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_32](rx_32) module"]
pub type RX_32 = crate::Reg<u32, _RX_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_32;
#[doc = "`read()` method returns [rx_32::R](rx_32::R) reader structure"]
impl crate::Readable for RX_32 {}
#[doc = "`write(|w| ..)` method takes [rx_32::W](rx_32::W) writer structure"]
impl crate::Writable for RX_32 {}
#[doc = "SPI RX FIFO 32-Bit Access"]
pub mod rx_32;
