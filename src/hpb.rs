#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controller Status Register."]
    pub status: STATUS,
    #[doc = "0x04 - Interrupt Enable Control Register."]
    pub inten: INTEN,
    #[doc = "0x08 - Interrupt Status Register."]
    pub intfl: INTFL,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Memory Base Address Registers."]
    pub mbr: [MBR; 2],
    _reserved4: [u8; 8usize],
    #[doc = "0x20 - Memory Configuration Registers."]
    pub mcr: [MCR; 2],
    _reserved5: [u8; 8usize],
    #[doc = "0x30 - Memory Timing Registers."]
    pub mtr: [MTR; 2],
}
#[doc = "Controller Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Controller Status Register."]
pub mod status;
#[doc = "Interrupt Enable Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt Enable Control Register."]
pub mod inten;
#[doc = "Interrupt Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](intfl) module"]
pub type INTFL = crate::Reg<u32, _INTFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFL;
#[doc = "`read()` method returns [intfl::R](intfl::R) reader structure"]
impl crate::Readable for INTFL {}
#[doc = "Interrupt Status Register."]
pub mod intfl;
#[doc = "Memory Base Address Registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbr](mbr) module"]
pub type MBR = crate::Reg<u32, _MBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MBR;
#[doc = "`read()` method returns [mbr::R](mbr::R) reader structure"]
impl crate::Readable for MBR {}
#[doc = "`write(|w| ..)` method takes [mbr::W](mbr::W) writer structure"]
impl crate::Writable for MBR {}
#[doc = "Memory Base Address Registers."]
pub mod mbr;
#[doc = "Memory Configuration Registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Memory Configuration Registers."]
pub mod mcr;
#[doc = "Memory Timing Registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtr](mtr) module"]
pub type MTR = crate::Reg<u32, _MTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTR;
#[doc = "`read()` method returns [mtr::R](mtr::R) reader structure"]
impl crate::Readable for MTR {}
#[doc = "`write(|w| ..)` method takes [mtr::W](mtr::W) writer structure"]
impl crate::Writable for MTR {}
#[doc = "Memory Timing Registers."]
pub mod mtr;
