#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Watchdog Timer Reset Register."]
    pub rst: RST,
}
#[doc = "Watchdog Timer Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Watchdog Timer Control Register."]
pub mod ctrl;
#[doc = "Watchdog Timer Reset Register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst](rst) module"]
pub type RST = crate::Reg<u32, _RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RST;
#[doc = "`write(|w| ..)` method takes [rst::W](rst::W) writer structure"]
impl crate::Writable for RST {}
#[doc = "Watchdog Timer Reset Register."]
pub mod rst;
