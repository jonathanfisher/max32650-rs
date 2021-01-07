#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function Control Register 0."]
    pub bbfcr0: BBFCR0,
}
#[doc = "Function Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbfcr0](bbfcr0) module"]
pub type BBFCR0 = crate::Reg<u32, _BBFCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BBFCR0;
#[doc = "`read()` method returns [bbfcr0::R](bbfcr0::R) reader structure"]
impl crate::Readable for BBFCR0 {}
#[doc = "`write(|w| ..)` method takes [bbfcr0::W](bbfcr0::W) writer structure"]
impl crate::Writable for BBFCR0 {}
#[doc = "Function Control Register 0."]
pub mod bbfcr0;
