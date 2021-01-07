#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Second Counter. This register contains the 32-bit second counter."]
    pub sec: SEC,
    #[doc = "0x04 - RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
    pub ssec: SSEC,
    #[doc = "0x08 - Time-of-day Alarm."]
    pub ras: RAS,
    #[doc = "0x0c - RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
    pub rssa: RSSA,
    #[doc = "0x10 - RTC Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x14 - RTC Trim Register."]
    pub trim: TRIM,
    #[doc = "0x18 - RTC Oscillator Control Register."]
    pub oscctrl: OSCCTRL,
}
#[doc = "RTC Second Counter. This register contains the 32-bit second counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec](sec) module"]
pub type SEC = crate::Reg<u32, _SEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC;
#[doc = "`read()` method returns [sec::R](sec::R) reader structure"]
impl crate::Readable for SEC {}
#[doc = "`write(|w| ..)` method takes [sec::W](sec::W) writer structure"]
impl crate::Writable for SEC {}
#[doc = "RTC Second Counter. This register contains the 32-bit second counter."]
pub mod sec;
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssec](ssec) module"]
pub type SSEC = crate::Reg<u32, _SSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSEC;
#[doc = "`read()` method returns [ssec::R](ssec::R) reader structure"]
impl crate::Readable for SSEC {}
#[doc = "`write(|w| ..)` method takes [ssec::W](ssec::W) writer structure"]
impl crate::Writable for SSEC {}
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
pub mod ssec;
#[doc = "Time-of-day Alarm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ras](ras) module"]
pub type RAS = crate::Reg<u32, _RAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAS;
#[doc = "`read()` method returns [ras::R](ras::R) reader structure"]
impl crate::Readable for RAS {}
#[doc = "`write(|w| ..)` method takes [ras::W](ras::W) writer structure"]
impl crate::Writable for RAS {}
#[doc = "Time-of-day Alarm."]
pub mod ras;
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssa](rssa) module"]
pub type RSSA = crate::Reg<u32, _RSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSSA;
#[doc = "`read()` method returns [rssa::R](rssa::R) reader structure"]
impl crate::Readable for RSSA {}
#[doc = "`write(|w| ..)` method takes [rssa::W](rssa::W) writer structure"]
impl crate::Writable for RSSA {}
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
pub mod rssa;
#[doc = "RTC Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RTC Control Register."]
pub mod ctrl;
#[doc = "RTC Trim Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim](trim) module"]
pub type TRIM = crate::Reg<u32, _TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIM;
#[doc = "`read()` method returns [trim::R](trim::R) reader structure"]
impl crate::Readable for TRIM {}
#[doc = "`write(|w| ..)` method takes [trim::W](trim::W) writer structure"]
impl crate::Writable for TRIM {}
#[doc = "RTC Trim Register."]
pub mod trim;
#[doc = "RTC Oscillator Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl](oscctrl) module"]
pub type OSCCTRL = crate::Reg<u32, _OSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCTRL;
#[doc = "`read()` method returns [oscctrl::R](oscctrl::R) reader structure"]
impl crate::Readable for OSCCTRL {}
#[doc = "`write(|w| ..)` method takes [oscctrl::W](oscctrl::W) writer structure"]
impl crate::Writable for OSCCTRL {}
#[doc = "RTC Oscillator Control Register."]
pub mod oscctrl;
