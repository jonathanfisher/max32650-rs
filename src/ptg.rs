#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Enable/Disable Controls for All Pulse Trains"]
    pub enable: ENABLE,
    #[doc = "0x04 - Global Resync (All Pulse Trains) Control"]
    pub resync: RESYNC,
    #[doc = "0x08 - Pulse Train Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x0c - Pulse Train Interrupt Enable/Disable"]
    pub inten: INTEN,
    #[doc = "0x10 - Pulse Train Global Safe Enable."]
    pub safe_en: SAFE_EN,
    #[doc = "0x14 - Pulse Train Global Safe Disable."]
    pub safe_dis: SAFE_DIS,
}
#[doc = "Global Enable/Disable Controls for All Pulse Trains\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub mod enable;
#[doc = "Global Resync (All Pulse Trains) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resync](resync) module"]
pub type RESYNC = crate::Reg<u32, _RESYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESYNC;
#[doc = "`read()` method returns [resync::R](resync::R) reader structure"]
impl crate::Readable for RESYNC {}
#[doc = "`write(|w| ..)` method takes [resync::W](resync::W) writer structure"]
impl crate::Writable for RESYNC {}
#[doc = "Global Resync (All Pulse Trains) Control"]
pub mod resync;
#[doc = "Pulse Train Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](intfl) module"]
pub type INTFL = crate::Reg<u32, _INTFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFL;
#[doc = "`read()` method returns [intfl::R](intfl::R) reader structure"]
impl crate::Readable for INTFL {}
#[doc = "`write(|w| ..)` method takes [intfl::W](intfl::W) writer structure"]
impl crate::Writable for INTFL {}
#[doc = "Pulse Train Interrupt Flags"]
pub mod intfl;
#[doc = "Pulse Train Interrupt Enable/Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Pulse Train Interrupt Enable/Disable"]
pub mod inten;
#[doc = "Pulse Train Global Safe Enable.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [safe_en](safe_en) module"]
pub type SAFE_EN = crate::Reg<u32, _SAFE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAFE_EN;
#[doc = "`write(|w| ..)` method takes [safe_en::W](safe_en::W) writer structure"]
impl crate::Writable for SAFE_EN {}
#[doc = "Pulse Train Global Safe Enable."]
pub mod safe_en;
#[doc = "Pulse Train Global Safe Disable.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [safe_dis](safe_dis) module"]
pub type SAFE_DIS = crate::Reg<u32, _SAFE_DIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAFE_DIS;
#[doc = "`write(|w| ..)` method takes [safe_dis::W](safe_dis::W) writer structure"]
impl crate::Writable for SAFE_DIS {}
#[doc = "Pulse Train Global Safe Disable."]
pub mod safe_dis;
