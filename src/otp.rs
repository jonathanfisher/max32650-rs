#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTP Control Register."]
    pub ocntl: OCNTL,
    #[doc = "0x04 - Clock Divider Register."]
    pub ckdiv: CKDIV,
    #[doc = "0x08 - Read Data Register."]
    pub otprdata: OTPRDATA,
    #[doc = "0x0c - OTP Status Register."]
    pub stat: STAT,
    _reserved4: [u8; 32usize],
    #[doc = "0x30 - Write Data Register."]
    pub odata: ODATA,
    _reserved5: [u8; 12usize],
    #[doc = "0x40 - Access Control Register."]
    pub acntl: ACNTL,
}
#[doc = "OTP Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocntl](ocntl) module"]
pub type OCNTL = crate::Reg<u32, _OCNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCNTL;
#[doc = "`read()` method returns [ocntl::R](ocntl::R) reader structure"]
impl crate::Readable for OCNTL {}
#[doc = "`write(|w| ..)` method takes [ocntl::W](ocntl::W) writer structure"]
impl crate::Writable for OCNTL {}
#[doc = "OTP Control Register."]
pub mod ocntl;
#[doc = "Clock Divider Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckdiv](ckdiv) module"]
pub type CKDIV = crate::Reg<u32, _CKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKDIV;
#[doc = "`read()` method returns [ckdiv::R](ckdiv::R) reader structure"]
impl crate::Readable for CKDIV {}
#[doc = "`write(|w| ..)` method takes [ckdiv::W](ckdiv::W) writer structure"]
impl crate::Writable for CKDIV {}
#[doc = "Clock Divider Register."]
pub mod ckdiv;
#[doc = "Read Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otprdata](otprdata) module"]
pub type OTPRDATA = crate::Reg<u32, _OTPRDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTPRDATA;
#[doc = "`read()` method returns [otprdata::R](otprdata::R) reader structure"]
impl crate::Readable for OTPRDATA {}
#[doc = "Read Data Register."]
pub mod otprdata;
#[doc = "OTP Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "OTP Status Register."]
pub mod stat;
#[doc = "Write Data Register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odata](odata) module"]
pub type ODATA = crate::Reg<u32, _ODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODATA;
#[doc = "`write(|w| ..)` method takes [odata::W](odata::W) writer structure"]
impl crate::Writable for ODATA {}
#[doc = "Write Data Register."]
pub mod odata;
#[doc = "Access Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acntl](acntl) module"]
pub type ACNTL = crate::Reg<u32, _ACNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACNTL;
#[doc = "`read()` method returns [acntl::R](acntl::R) reader structure"]
impl crate::Readable for ACNTL {}
#[doc = "`write(|w| ..)` method takes [acntl::W](acntl::W) writer structure"]
impl crate::Writable for ACNTL {}
#[doc = "Access Control Register."]
pub mod acntl;
