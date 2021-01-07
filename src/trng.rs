#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRNG Control Register."]
    pub cn: CN,
    #[doc = "0x04 - Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
    pub data: DATA,
}
#[doc = "TRNG Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cn](cn) module"]
pub type CN = crate::Reg<u32, _CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CN;
#[doc = "`read()` method returns [cn::R](cn::R) reader structure"]
impl crate::Readable for CN {}
#[doc = "`write(|w| ..)` method takes [cn::W](cn::W) writer structure"]
impl crate::Writable for CN {}
#[doc = "TRNG Control Register."]
pub mod cn;
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
pub mod data;
