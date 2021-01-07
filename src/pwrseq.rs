#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Control Register."]
    pub lpcn: LPCN,
    #[doc = "0x04 - Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
    pub lpwkst0: LPWKST0,
    #[doc = "0x08 - Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
    pub lpwken0: LPWKEN0,
    #[doc = "0x0c - Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1."]
    pub lpwkst1: LPWKST1,
    #[doc = "0x10 - Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1."]
    pub lpwken1: LPWKEN1,
    #[doc = "0x14 - Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2."]
    pub lpwkst2: LPWKST2,
    #[doc = "0x18 - Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2."]
    pub lpwken2: LPWKEN2,
    #[doc = "0x1c - Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3."]
    pub lpwkst3: LPWKST3,
    #[doc = "0x20 - Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3."]
    pub lpwken3: LPWKEN3,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - Low Power Peripheral Wakeup Status Register."]
    pub lppwst: LPPWST,
    #[doc = "0x34 - Low Power Peripheral Wakeup Enable Register."]
    pub lppwen: LPPWEN,
    _reserved11: [u8; 8usize],
    #[doc = "0x40 - Low Power Memory Shutdown Control."]
    pub lpmemsd: LPMEMSD,
    _reserved12: [u8; 4usize],
    #[doc = "0x48 - Battery Backed General Purpose Register 0"]
    pub bb_gp0: BB_GP0,
    #[doc = "0x4c - Battery Backed General Purpose Register 1"]
    pub bb_gp1: BB_GP1,
    #[doc = "0x50 - Low Power Multi-Core Status"]
    pub lpmcstat: LPMCSTAT,
    #[doc = "0x54 - Low Power Multi-Core Request"]
    pub lpmcreq: LPMCREQ,
}
#[doc = "Low Power Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcn](lpcn) module"]
pub type LPCN = crate::Reg<u32, _LPCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCN;
#[doc = "`read()` method returns [lpcn::R](lpcn::R) reader structure"]
impl crate::Readable for LPCN {}
#[doc = "`write(|w| ..)` method takes [lpcn::W](lpcn::W) writer structure"]
impl crate::Writable for LPCN {}
#[doc = "Low Power Control Register."]
pub mod lpcn;
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwkst0](lpwkst0) module"]
pub type LPWKST0 = crate::Reg<u32, _LPWKST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKST0;
#[doc = "`read()` method returns [lpwkst0::R](lpwkst0::R) reader structure"]
impl crate::Readable for LPWKST0 {}
#[doc = "`write(|w| ..)` method takes [lpwkst0::W](lpwkst0::W) writer structure"]
impl crate::Writable for LPWKST0 {}
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
pub mod lpwkst0;
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwken0](lpwken0) module"]
pub type LPWKEN0 = crate::Reg<u32, _LPWKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKEN0;
#[doc = "`read()` method returns [lpwken0::R](lpwken0::R) reader structure"]
impl crate::Readable for LPWKEN0 {}
#[doc = "`write(|w| ..)` method takes [lpwken0::W](lpwken0::W) writer structure"]
impl crate::Writable for LPWKEN0 {}
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
pub mod lpwken0;
#[doc = "Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwkst1](lpwkst1) module"]
pub type LPWKST1 = crate::Reg<u32, _LPWKST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKST1;
#[doc = "`read()` method returns [lpwkst1::R](lpwkst1::R) reader structure"]
impl crate::Readable for LPWKST1 {}
#[doc = "`write(|w| ..)` method takes [lpwkst1::W](lpwkst1::W) writer structure"]
impl crate::Writable for LPWKST1 {}
#[doc = "Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1."]
pub mod lpwkst1;
#[doc = "Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwken1](lpwken1) module"]
pub type LPWKEN1 = crate::Reg<u32, _LPWKEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKEN1;
#[doc = "`read()` method returns [lpwken1::R](lpwken1::R) reader structure"]
impl crate::Readable for LPWKEN1 {}
#[doc = "`write(|w| ..)` method takes [lpwken1::W](lpwken1::W) writer structure"]
impl crate::Writable for LPWKEN1 {}
#[doc = "Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1."]
pub mod lpwken1;
#[doc = "Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwkst2](lpwkst2) module"]
pub type LPWKST2 = crate::Reg<u32, _LPWKST2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKST2;
#[doc = "`read()` method returns [lpwkst2::R](lpwkst2::R) reader structure"]
impl crate::Readable for LPWKST2 {}
#[doc = "`write(|w| ..)` method takes [lpwkst2::W](lpwkst2::W) writer structure"]
impl crate::Writable for LPWKST2 {}
#[doc = "Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2."]
pub mod lpwkst2;
#[doc = "Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwken2](lpwken2) module"]
pub type LPWKEN2 = crate::Reg<u32, _LPWKEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKEN2;
#[doc = "`read()` method returns [lpwken2::R](lpwken2::R) reader structure"]
impl crate::Readable for LPWKEN2 {}
#[doc = "`write(|w| ..)` method takes [lpwken2::W](lpwken2::W) writer structure"]
impl crate::Writable for LPWKEN2 {}
#[doc = "Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2."]
pub mod lpwken2;
#[doc = "Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwkst3](lpwkst3) module"]
pub type LPWKST3 = crate::Reg<u32, _LPWKST3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKST3;
#[doc = "`read()` method returns [lpwkst3::R](lpwkst3::R) reader structure"]
impl crate::Readable for LPWKST3 {}
#[doc = "`write(|w| ..)` method takes [lpwkst3::W](lpwkst3::W) writer structure"]
impl crate::Writable for LPWKST3 {}
#[doc = "Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3."]
pub mod lpwkst3;
#[doc = "Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpwken3](lpwken3) module"]
pub type LPWKEN3 = crate::Reg<u32, _LPWKEN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPWKEN3;
#[doc = "`read()` method returns [lpwken3::R](lpwken3::R) reader structure"]
impl crate::Readable for LPWKEN3 {}
#[doc = "`write(|w| ..)` method takes [lpwken3::W](lpwken3::W) writer structure"]
impl crate::Writable for LPWKEN3 {}
#[doc = "Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3."]
pub mod lpwken3;
#[doc = "Low Power Peripheral Wakeup Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lppwst](lppwst) module"]
pub type LPPWST = crate::Reg<u32, _LPPWST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPPWST;
#[doc = "`read()` method returns [lppwst::R](lppwst::R) reader structure"]
impl crate::Readable for LPPWST {}
#[doc = "`write(|w| ..)` method takes [lppwst::W](lppwst::W) writer structure"]
impl crate::Writable for LPPWST {}
#[doc = "Low Power Peripheral Wakeup Status Register."]
pub mod lppwst;
#[doc = "Low Power Peripheral Wakeup Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lppwen](lppwen) module"]
pub type LPPWEN = crate::Reg<u32, _LPPWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPPWEN;
#[doc = "`read()` method returns [lppwen::R](lppwen::R) reader structure"]
impl crate::Readable for LPPWEN {}
#[doc = "`write(|w| ..)` method takes [lppwen::W](lppwen::W) writer structure"]
impl crate::Writable for LPPWEN {}
#[doc = "Low Power Peripheral Wakeup Enable Register."]
pub mod lppwen;
#[doc = "Low Power Memory Shutdown Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmemsd](lpmemsd) module"]
pub type LPMEMSD = crate::Reg<u32, _LPMEMSD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMEMSD;
#[doc = "`read()` method returns [lpmemsd::R](lpmemsd::R) reader structure"]
impl crate::Readable for LPMEMSD {}
#[doc = "`write(|w| ..)` method takes [lpmemsd::W](lpmemsd::W) writer structure"]
impl crate::Writable for LPMEMSD {}
#[doc = "Low Power Memory Shutdown Control."]
pub mod lpmemsd;
#[doc = "Battery Backed General Purpose Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_gp0](bb_gp0) module"]
pub type BB_GP0 = crate::Reg<u32, _BB_GP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_GP0;
#[doc = "`read()` method returns [bb_gp0::R](bb_gp0::R) reader structure"]
impl crate::Readable for BB_GP0 {}
#[doc = "`write(|w| ..)` method takes [bb_gp0::W](bb_gp0::W) writer structure"]
impl crate::Writable for BB_GP0 {}
#[doc = "Battery Backed General Purpose Register 0"]
pub mod bb_gp0;
#[doc = "Battery Backed General Purpose Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_gp1](bb_gp1) module"]
pub type BB_GP1 = crate::Reg<u32, _BB_GP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BB_GP1;
#[doc = "`read()` method returns [bb_gp1::R](bb_gp1::R) reader structure"]
impl crate::Readable for BB_GP1 {}
#[doc = "`write(|w| ..)` method takes [bb_gp1::W](bb_gp1::W) writer structure"]
impl crate::Writable for BB_GP1 {}
#[doc = "Battery Backed General Purpose Register 1"]
pub mod bb_gp1;
#[doc = "Low Power Multi-Core Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcstat](lpmcstat) module"]
pub type LPMCSTAT = crate::Reg<u32, _LPMCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCSTAT;
#[doc = "`read()` method returns [lpmcstat::R](lpmcstat::R) reader structure"]
impl crate::Readable for LPMCSTAT {}
#[doc = "`write(|w| ..)` method takes [lpmcstat::W](lpmcstat::W) writer structure"]
impl crate::Writable for LPMCSTAT {}
#[doc = "Low Power Multi-Core Status"]
pub mod lpmcstat;
#[doc = "Low Power Multi-Core Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcreq](lpmcreq) module"]
pub type LPMCREQ = crate::Reg<u32, _LPMCREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCREQ;
#[doc = "`read()` method returns [lpmcreq::R](lpmcreq::R) reader structure"]
impl crate::Readable for LPMCREQ {}
#[doc = "`write(|w| ..)` method takes [lpmcreq::W](lpmcreq::W) writer structure"]
impl crate::Writable for LPMCREQ {}
#[doc = "Low Power Multi-Core Request"]
pub mod lpmcreq;
