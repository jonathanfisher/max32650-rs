#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control."]
    pub scon: SCON,
    #[doc = "0x04 - Reset."]
    pub rstr0: RSTR0,
    #[doc = "0x08 - Clock Control."]
    pub clkcn: CLKCN,
    #[doc = "0x0c - Power Management."]
    pub pm: PM,
    #[doc = "0x10 - PLL 0 Control."]
    pub pll0cn: PLL0CN,
    #[doc = "0x14 - PLL 1 Control."]
    pub pll1cn: PLL1CN,
    #[doc = "0x18 - Peripheral Clock Divider."]
    pub pckdiv: PCKDIV,
    _reserved7: [u8; 8usize],
    #[doc = "0x24 - Peripheral Clock Disable."]
    pub perckcn0: PERCKCN0,
    #[doc = "0x28 - Memory Clock Control Register."]
    pub memckcn: MEMCKCN,
    #[doc = "0x2c - Memory Zeroize Control."]
    pub memzcn: MEMZCN,
    _reserved10: [u8; 4usize],
    #[doc = "0x34 - Smart Card Clock Control."]
    pub scck: SCCK,
    #[doc = "0x38 - Master Priority Control Register 0."]
    pub mpri0: MPRI0,
    #[doc = "0x3c - Mater Priority Control Register 1."]
    pub mpri1: MPRI1,
    #[doc = "0x40 - System Status Register."]
    pub sysst: SYSST,
    #[doc = "0x44 - Reset 1."]
    pub rstr1: RSTR1,
    #[doc = "0x48 - Peripheral Clock Disable."]
    pub perckcn1: PERCKCN1,
    #[doc = "0x4c - Event Enable Register."]
    pub evten: EVTEN,
    #[doc = "0x50 - Revision Register."]
    pub revision: REVISION,
    #[doc = "0x54 - System Status Interrupt Enable Register."]
    pub syssie: SYSSIE,
}
#[doc = "System Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scon](scon) module"]
pub type SCON = crate::Reg<u32, _SCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCON;
#[doc = "`read()` method returns [scon::R](scon::R) reader structure"]
impl crate::Readable for SCON {}
#[doc = "`write(|w| ..)` method takes [scon::W](scon::W) writer structure"]
impl crate::Writable for SCON {}
#[doc = "System Control."]
pub mod scon;
#[doc = "Reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstr0](rstr0) module"]
pub type RSTR0 = crate::Reg<u32, _RSTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTR0;
#[doc = "`read()` method returns [rstr0::R](rstr0::R) reader structure"]
impl crate::Readable for RSTR0 {}
#[doc = "`write(|w| ..)` method takes [rstr0::W](rstr0::W) writer structure"]
impl crate::Writable for RSTR0 {}
#[doc = "Reset."]
pub mod rstr0;
#[doc = "Clock Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcn](clkcn) module"]
pub type CLKCN = crate::Reg<u32, _CLKCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCN;
#[doc = "`read()` method returns [clkcn::R](clkcn::R) reader structure"]
impl crate::Readable for CLKCN {}
#[doc = "`write(|w| ..)` method takes [clkcn::W](clkcn::W) writer structure"]
impl crate::Writable for CLKCN {}
#[doc = "Clock Control."]
pub mod clkcn;
#[doc = "Power Management.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm](pm) module"]
pub type PM = crate::Reg<u32, _PM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PM;
#[doc = "`read()` method returns [pm::R](pm::R) reader structure"]
impl crate::Readable for PM {}
#[doc = "`write(|w| ..)` method takes [pm::W](pm::W) writer structure"]
impl crate::Writable for PM {}
#[doc = "Power Management."]
pub mod pm;
#[doc = "PLL 0 Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0cn](pll0cn) module"]
pub type PLL0CN = crate::Reg<u32, _PLL0CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0CN;
#[doc = "`read()` method returns [pll0cn::R](pll0cn::R) reader structure"]
impl crate::Readable for PLL0CN {}
#[doc = "`write(|w| ..)` method takes [pll0cn::W](pll0cn::W) writer structure"]
impl crate::Writable for PLL0CN {}
#[doc = "PLL 0 Control."]
pub mod pll0cn;
#[doc = "PLL 1 Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1cn](pll1cn) module"]
pub type PLL1CN = crate::Reg<u32, _PLL1CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1CN;
#[doc = "`read()` method returns [pll1cn::R](pll1cn::R) reader structure"]
impl crate::Readable for PLL1CN {}
#[doc = "`write(|w| ..)` method takes [pll1cn::W](pll1cn::W) writer structure"]
impl crate::Writable for PLL1CN {}
#[doc = "PLL 1 Control."]
pub mod pll1cn;
#[doc = "Peripheral Clock Divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pckdiv](pckdiv) module"]
pub type PCKDIV = crate::Reg<u32, _PCKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCKDIV;
#[doc = "`read()` method returns [pckdiv::R](pckdiv::R) reader structure"]
impl crate::Readable for PCKDIV {}
#[doc = "`write(|w| ..)` method takes [pckdiv::W](pckdiv::W) writer structure"]
impl crate::Writable for PCKDIV {}
#[doc = "Peripheral Clock Divider."]
pub mod pckdiv;
#[doc = "Peripheral Clock Disable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perckcn0](perckcn0) module"]
pub type PERCKCN0 = crate::Reg<u32, _PERCKCN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERCKCN0;
#[doc = "`read()` method returns [perckcn0::R](perckcn0::R) reader structure"]
impl crate::Readable for PERCKCN0 {}
#[doc = "`write(|w| ..)` method takes [perckcn0::W](perckcn0::W) writer structure"]
impl crate::Writable for PERCKCN0 {}
#[doc = "Peripheral Clock Disable."]
pub mod perckcn0;
#[doc = "Memory Clock Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memckcn](memckcn) module"]
pub type MEMCKCN = crate::Reg<u32, _MEMCKCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMCKCN;
#[doc = "`read()` method returns [memckcn::R](memckcn::R) reader structure"]
impl crate::Readable for MEMCKCN {}
#[doc = "`write(|w| ..)` method takes [memckcn::W](memckcn::W) writer structure"]
impl crate::Writable for MEMCKCN {}
#[doc = "Memory Clock Control Register."]
pub mod memckcn;
#[doc = "Memory Zeroize Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memzcn](memzcn) module"]
pub type MEMZCN = crate::Reg<u32, _MEMZCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMZCN;
#[doc = "`read()` method returns [memzcn::R](memzcn::R) reader structure"]
impl crate::Readable for MEMZCN {}
#[doc = "`write(|w| ..)` method takes [memzcn::W](memzcn::W) writer structure"]
impl crate::Writable for MEMZCN {}
#[doc = "Memory Zeroize Control."]
pub mod memzcn;
#[doc = "Smart Card Clock Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scck](scck) module"]
pub type SCCK = crate::Reg<u32, _SCCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCCK;
#[doc = "`read()` method returns [scck::R](scck::R) reader structure"]
impl crate::Readable for SCCK {}
#[doc = "`write(|w| ..)` method takes [scck::W](scck::W) writer structure"]
impl crate::Writable for SCCK {}
#[doc = "Smart Card Clock Control."]
pub mod scck;
#[doc = "Master Priority Control Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpri0](mpri0) module"]
pub type MPRI0 = crate::Reg<u32, _MPRI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPRI0;
#[doc = "`read()` method returns [mpri0::R](mpri0::R) reader structure"]
impl crate::Readable for MPRI0 {}
#[doc = "`write(|w| ..)` method takes [mpri0::W](mpri0::W) writer structure"]
impl crate::Writable for MPRI0 {}
#[doc = "Master Priority Control Register 0."]
pub mod mpri0;
#[doc = "Mater Priority Control Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpri1](mpri1) module"]
pub type MPRI1 = crate::Reg<u32, _MPRI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPRI1;
#[doc = "`read()` method returns [mpri1::R](mpri1::R) reader structure"]
impl crate::Readable for MPRI1 {}
#[doc = "`write(|w| ..)` method takes [mpri1::W](mpri1::W) writer structure"]
impl crate::Writable for MPRI1 {}
#[doc = "Mater Priority Control Register 1."]
pub mod mpri1;
#[doc = "System Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysst](sysst) module"]
pub type SYSST = crate::Reg<u32, _SYSST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSST;
#[doc = "`read()` method returns [sysst::R](sysst::R) reader structure"]
impl crate::Readable for SYSST {}
#[doc = "`write(|w| ..)` method takes [sysst::W](sysst::W) writer structure"]
impl crate::Writable for SYSST {}
#[doc = "System Status Register."]
pub mod sysst;
#[doc = "Reset 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstr1](rstr1) module"]
pub type RSTR1 = crate::Reg<u32, _RSTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTR1;
#[doc = "`read()` method returns [rstr1::R](rstr1::R) reader structure"]
impl crate::Readable for RSTR1 {}
#[doc = "`write(|w| ..)` method takes [rstr1::W](rstr1::W) writer structure"]
impl crate::Writable for RSTR1 {}
#[doc = "Reset 1."]
pub mod rstr1;
#[doc = "Peripheral Clock Disable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perckcn1](perckcn1) module"]
pub type PERCKCN1 = crate::Reg<u32, _PERCKCN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERCKCN1;
#[doc = "`read()` method returns [perckcn1::R](perckcn1::R) reader structure"]
impl crate::Readable for PERCKCN1 {}
#[doc = "`write(|w| ..)` method takes [perckcn1::W](perckcn1::W) writer structure"]
impl crate::Writable for PERCKCN1 {}
#[doc = "Peripheral Clock Disable."]
pub mod perckcn1;
#[doc = "Event Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evten](evten) module"]
pub type EVTEN = crate::Reg<u32, _EVTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTEN;
#[doc = "`read()` method returns [evten::R](evten::R) reader structure"]
impl crate::Readable for EVTEN {}
#[doc = "`write(|w| ..)` method takes [evten::W](evten::W) writer structure"]
impl crate::Writable for EVTEN {}
#[doc = "Event Enable Register."]
pub mod evten;
#[doc = "Revision Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "Revision Register."]
pub mod revision;
#[doc = "System Status Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssie](syssie) module"]
pub type SYSSIE = crate::Reg<u32, _SYSSIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSIE;
#[doc = "`read()` method returns [syssie::R](syssie::R) reader structure"]
impl crate::Readable for SYSSIE {}
#[doc = "`write(|w| ..)` method takes [syssie::W](syssie::W) writer structure"]
impl crate::Writable for SYSSIE {}
#[doc = "System Status Interrupt Enable Register."]
pub mod syssie;
