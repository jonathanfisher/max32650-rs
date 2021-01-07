#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Sensor Control Register."]
    pub extscn: EXTSCN,
    #[doc = "0x04 - Internal Sensor Control Register."]
    pub intscn: INTSCN,
    #[doc = "0x08 - Security Alarm Register."]
    pub secalm: SECALM,
    #[doc = "0x0c - Security Diagnostic Register."]
    pub secdiag: SECDIAG,
    #[doc = "0x10 - DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occurred."]
    pub dlrtc: DLRTC,
    _reserved5: [u8; 32usize],
    #[doc = "0x34 - Security Monitor Status Register."]
    pub secst: SECST,
}
#[doc = "External Sensor Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extscn](extscn) module"]
pub type EXTSCN = crate::Reg<u32, _EXTSCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTSCN;
#[doc = "`read()` method returns [extscn::R](extscn::R) reader structure"]
impl crate::Readable for EXTSCN {}
#[doc = "`write(|w| ..)` method takes [extscn::W](extscn::W) writer structure"]
impl crate::Writable for EXTSCN {}
#[doc = "External Sensor Control Register."]
pub mod extscn;
#[doc = "Internal Sensor Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intscn](intscn) module"]
pub type INTSCN = crate::Reg<u32, _INTSCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSCN;
#[doc = "`read()` method returns [intscn::R](intscn::R) reader structure"]
impl crate::Readable for INTSCN {}
#[doc = "`write(|w| ..)` method takes [intscn::W](intscn::W) writer structure"]
impl crate::Writable for INTSCN {}
#[doc = "Internal Sensor Control Register."]
pub mod intscn;
#[doc = "Security Alarm Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secalm](secalm) module"]
pub type SECALM = crate::Reg<u32, _SECALM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECALM;
#[doc = "`read()` method returns [secalm::R](secalm::R) reader structure"]
impl crate::Readable for SECALM {}
#[doc = "`write(|w| ..)` method takes [secalm::W](secalm::W) writer structure"]
impl crate::Writable for SECALM {}
#[doc = "Security Alarm Register."]
pub mod secalm;
#[doc = "Security Diagnostic Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secdiag](secdiag) module"]
pub type SECDIAG = crate::Reg<u32, _SECDIAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECDIAG;
#[doc = "`read()` method returns [secdiag::R](secdiag::R) reader structure"]
impl crate::Readable for SECDIAG {}
#[doc = "Security Diagnostic Register."]
pub mod secdiag;
#[doc = "DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occurred.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlrtc](dlrtc) module"]
pub type DLRTC = crate::Reg<u32, _DLRTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLRTC;
#[doc = "`read()` method returns [dlrtc::R](dlrtc::R) reader structure"]
impl crate::Readable for DLRTC {}
#[doc = "DRS Log RTC Value. This register contains the 32 bit value in the RTC second register when the last DRS event occurred."]
pub mod dlrtc;
#[doc = "Security Monitor Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secst](secst) module"]
pub type SECST = crate::Reg<u32, _SECST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECST;
#[doc = "`read()` method returns [secst::R](secst::R) reader structure"]
impl crate::Readable for SECST {}
#[doc = "Security Monitor Status Register."]
pub mod secst;
