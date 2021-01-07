#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 1-Wire Master Configuration."]
    pub cfg: CFG,
    #[doc = "0x04 - 1-Wire Master Clock Divisor."]
    pub clk_div_1us: CLK_DIV_1US,
    #[doc = "0x08 - 1-Wire Master Control/Status."]
    pub ctrl_stat: CTRL_STAT,
    #[doc = "0x0c - 1-Wire Master Data Buffer."]
    pub data: DATA,
    #[doc = "0x10 - 1-Wire Master Interrupt Flags."]
    pub intfl: INTFL,
    #[doc = "0x14 - 1-Wire Master Interrupt Enables."]
    pub inten: INTEN,
}
#[doc = "1-Wire Master Configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "1-Wire Master Configuration."]
pub mod cfg;
#[doc = "1-Wire Master Clock Divisor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div_1us](clk_div_1us) module"]
pub type CLK_DIV_1US = crate::Reg<u32, _CLK_DIV_1US>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DIV_1US;
#[doc = "`read()` method returns [clk_div_1us::R](clk_div_1us::R) reader structure"]
impl crate::Readable for CLK_DIV_1US {}
#[doc = "`write(|w| ..)` method takes [clk_div_1us::W](clk_div_1us::W) writer structure"]
impl crate::Writable for CLK_DIV_1US {}
#[doc = "1-Wire Master Clock Divisor."]
pub mod clk_div_1us;
#[doc = "1-Wire Master Control/Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_stat](ctrl_stat) module"]
pub type CTRL_STAT = crate::Reg<u32, _CTRL_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_STAT;
#[doc = "`read()` method returns [ctrl_stat::R](ctrl_stat::R) reader structure"]
impl crate::Readable for CTRL_STAT {}
#[doc = "`write(|w| ..)` method takes [ctrl_stat::W](ctrl_stat::W) writer structure"]
impl crate::Writable for CTRL_STAT {}
#[doc = "1-Wire Master Control/Status."]
pub mod ctrl_stat;
#[doc = "1-Wire Master Data Buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "1-Wire Master Data Buffer."]
pub mod data;
#[doc = "1-Wire Master Interrupt Flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](intfl) module"]
pub type INTFL = crate::Reg<u32, _INTFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFL;
#[doc = "`read()` method returns [intfl::R](intfl::R) reader structure"]
impl crate::Readable for INTFL {}
#[doc = "`write(|w| ..)` method takes [intfl::W](intfl::W) writer structure"]
impl crate::Writable for INTFL {}
#[doc = "1-Wire Master Interrupt Flags."]
pub mod intfl;
#[doc = "1-Wire Master Interrupt Enables.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "1-Wire Master Interrupt Enables."]
pub mod inten;
