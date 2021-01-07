#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Initialization Status Register."]
    pub sistat: SISTAT,
    #[doc = "0x04 - Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
    pub erraddr: ERRADDR,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - funcstat register."]
    pub fstat: FSTAT,
    #[doc = "0x104 - secfuncstat register."]
    pub sfstat: SFSTAT,
}
#[doc = "System Initialization Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sistat](sistat) module"]
pub type SISTAT = crate::Reg<u32, _SISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SISTAT;
#[doc = "`read()` method returns [sistat::R](sistat::R) reader structure"]
impl crate::Readable for SISTAT {}
#[doc = "System Initialization Status Register."]
pub mod sistat;
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erraddr](erraddr) module"]
pub type ERRADDR = crate::Reg<u32, _ERRADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRADDR;
#[doc = "`read()` method returns [erraddr::R](erraddr::R) reader structure"]
impl crate::Readable for ERRADDR {}
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
pub mod erraddr;
#[doc = "funcstat register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u32, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "funcstat register."]
pub mod fstat;
#[doc = "secfuncstat register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfstat](sfstat) module"]
pub type SFSTAT = crate::Reg<u32, _SFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFSTAT;
#[doc = "`read()` method returns [sfstat::R](sfstat::R) reader structure"]
impl crate::Readable for SFSTAT {}
#[doc = "secfuncstat register."]
pub mod sfstat;
