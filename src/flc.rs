#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Write Address."]
    pub addr: ADDR,
    #[doc = "0x04 - Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller."]
    pub clkdiv: CLKDIV,
    #[doc = "0x08 - Flash Control Register."]
    pub cn: CN,
    _reserved3: [u8; 24usize],
    #[doc = "0x24 - Flash Interrupt Register."]
    pub intr: INTR,
    _reserved4: [u8; 8usize],
    #[doc = "0x30 - Flash Write Data."]
    pub data: [DATA; 4],
    #[doc = "0x40 - Access Control Register. Writing the ACNTL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-acntl = 0x3a7f5ca3; pflc-acntl = 0xa1e34f20; pflc-acntl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero."]
    pub acntl: ACNTL,
}
#[doc = "Flash Write Address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Flash Write Address."]
pub mod addr;
#[doc = "Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller."]
pub mod clkdiv;
#[doc = "Flash Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cn](cn) module"]
pub type CN = crate::Reg<u32, _CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CN;
#[doc = "`read()` method returns [cn::R](cn::R) reader structure"]
impl crate::Readable for CN {}
#[doc = "`write(|w| ..)` method takes [cn::W](cn::W) writer structure"]
impl crate::Writable for CN {}
#[doc = "Flash Control Register."]
pub mod cn;
#[doc = "Flash Interrupt Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Flash Interrupt Register."]
pub mod intr;
#[doc = "Flash Write Data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Flash Write Data."]
pub mod data;
#[doc = "Access Control Register. Writing the ACNTL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-acntl = 0x3a7f5ca3; pflc-acntl = 0xa1e34f20; pflc-acntl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acntl](acntl) module"]
pub type ACNTL = crate::Reg<u32, _ACNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACNTL;
#[doc = "`write(|w| ..)` method takes [acntl::W](acntl::W) writer structure"]
impl crate::Writable for ACNTL {}
#[doc = "Access Control Register. Writing the ACNTL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-acntl = 0x3a7f5ca3; pflc-acntl = 0xa1e34f20; pflc-acntl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero."]
pub mod acntl;
