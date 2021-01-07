#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD Clock Register"]
    pub clk: CLK,
    #[doc = "0x04 - LCD Vertical Timing 0 Register"]
    pub vtim_0: VTIM_0,
    #[doc = "0x08 - LCD Vertical Timing 1 Register"]
    pub vtim_1: VTIM_1,
    #[doc = "0x0c - LCD Horizontal Timing Register."]
    pub htim: HTIM,
    #[doc = "0x10 - LCD Control Register"]
    pub ctrl: CTRL,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - FR"]
    pub fr: FR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - LCD Interrupt Enable Register."]
    pub int_en: INT_EN,
    #[doc = "0x24 - LCD Status Register."]
    pub stat: STAT,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - HV Phase"]
    pub hv_phase: HV_PHASE,
    _reserved9: [u8; 972usize],
    #[doc = "0x400 - Palette"]
    pub palette: [PALETTE; 256],
}
#[doc = "LCD Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](clk) module"]
pub type CLK = crate::Reg<u32, _CLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK;
#[doc = "`read()` method returns [clk::R](clk::R) reader structure"]
impl crate::Readable for CLK {}
#[doc = "`write(|w| ..)` method takes [clk::W](clk::W) writer structure"]
impl crate::Writable for CLK {}
#[doc = "LCD Clock Register"]
pub mod clk;
#[doc = "LCD Vertical Timing 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtim_0](vtim_0) module"]
pub type VTIM_0 = crate::Reg<u32, _VTIM_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTIM_0;
#[doc = "`read()` method returns [vtim_0::R](vtim_0::R) reader structure"]
impl crate::Readable for VTIM_0 {}
#[doc = "`write(|w| ..)` method takes [vtim_0::W](vtim_0::W) writer structure"]
impl crate::Writable for VTIM_0 {}
#[doc = "LCD Vertical Timing 0 Register"]
pub mod vtim_0;
#[doc = "LCD Vertical Timing 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtim_1](vtim_1) module"]
pub type VTIM_1 = crate::Reg<u32, _VTIM_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTIM_1;
#[doc = "`read()` method returns [vtim_1::R](vtim_1::R) reader structure"]
impl crate::Readable for VTIM_1 {}
#[doc = "`write(|w| ..)` method takes [vtim_1::W](vtim_1::W) writer structure"]
impl crate::Writable for VTIM_1 {}
#[doc = "LCD Vertical Timing 1 Register"]
pub mod vtim_1;
#[doc = "LCD Horizontal Timing Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htim](htim) module"]
pub type HTIM = crate::Reg<u32, _HTIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HTIM;
#[doc = "`read()` method returns [htim::R](htim::R) reader structure"]
impl crate::Readable for HTIM {}
#[doc = "`write(|w| ..)` method takes [htim::W](htim::W) writer structure"]
impl crate::Writable for HTIM {}
#[doc = "LCD Horizontal Timing Register."]
pub mod htim;
#[doc = "LCD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "LCD Control Register"]
pub mod ctrl;
#[doc = "FR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "`write(|w| ..)` method takes [fr::W](fr::W) writer structure"]
impl crate::Writable for FR {}
#[doc = "FR"]
pub mod fr;
#[doc = "LCD Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u32, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "LCD Interrupt Enable Register."]
pub mod int_en;
#[doc = "LCD Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "LCD Status Register."]
pub mod stat;
#[doc = "HV Phase\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hv_phase](hv_phase) module"]
pub type HV_PHASE = crate::Reg<u32, _HV_PHASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HV_PHASE;
#[doc = "`read()` method returns [hv_phase::R](hv_phase::R) reader structure"]
impl crate::Readable for HV_PHASE {}
#[doc = "`write(|w| ..)` method takes [hv_phase::W](hv_phase::W) writer structure"]
impl crate::Writable for HV_PHASE {}
#[doc = "HV Phase"]
pub mod hv_phase;
#[doc = "Palette\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [palette](palette) module"]
pub type PALETTE = crate::Reg<u32, _PALETTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PALETTE;
#[doc = "`read()` method returns [palette::R](palette::R) reader structure"]
impl crate::Readable for PALETTE {}
#[doc = "`write(|w| ..)` method takes [palette::W](palette::W) writer structure"]
impl crate::Writable for PALETTE {}
#[doc = "Palette"]
pub mod palette;
