#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register."]
    pub cfg: CFG,
    #[doc = "0x04 - SPIX Controller Slave Select Polarity Register."]
    pub ss_pol: SS_POL,
    #[doc = "0x08 - SPIX Controller General Controller Register."]
    pub gen_ctrl: GEN_CTRL,
    #[doc = "0x0c - SPIX Controller FIFO Control and Status Register."]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x10 - SPIX Controller Special Control Register."]
    pub spctrl: SPCTRL,
    #[doc = "0x14 - SPIX Controller Interrupt Status Register."]
    pub intfl: INTFL,
    #[doc = "0x18 - SPIX Controller Interrupt Enable Register."]
    pub inten: INTEN,
}
#[doc = "Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration Register."]
pub mod cfg;
#[doc = "SPIX Controller Slave Select Polarity Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss_pol](ss_pol) module"]
pub type SS_POL = crate::Reg<u32, _SS_POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS_POL;
#[doc = "`read()` method returns [ss_pol::R](ss_pol::R) reader structure"]
impl crate::Readable for SS_POL {}
#[doc = "`write(|w| ..)` method takes [ss_pol::W](ss_pol::W) writer structure"]
impl crate::Writable for SS_POL {}
#[doc = "SPIX Controller Slave Select Polarity Register."]
pub mod ss_pol;
#[doc = "SPIX Controller General Controller Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen_ctrl](gen_ctrl) module"]
pub type GEN_CTRL = crate::Reg<u32, _GEN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GEN_CTRL;
#[doc = "`read()` method returns [gen_ctrl::R](gen_ctrl::R) reader structure"]
impl crate::Readable for GEN_CTRL {}
#[doc = "`write(|w| ..)` method takes [gen_ctrl::W](gen_ctrl::W) writer structure"]
impl crate::Writable for GEN_CTRL {}
#[doc = "SPIX Controller General Controller Register."]
pub mod gen_ctrl;
#[doc = "SPIX Controller FIFO Control and Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](fifo_ctrl) module"]
pub type FIFO_CTRL = crate::Reg<u32, _FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CTRL;
#[doc = "`read()` method returns [fifo_ctrl::R](fifo_ctrl::R) reader structure"]
impl crate::Readable for FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](fifo_ctrl::W) writer structure"]
impl crate::Writable for FIFO_CTRL {}
#[doc = "SPIX Controller FIFO Control and Status Register."]
pub mod fifo_ctrl;
#[doc = "SPIX Controller Special Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spctrl](spctrl) module"]
pub type SPCTRL = crate::Reg<u32, _SPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPCTRL;
#[doc = "`read()` method returns [spctrl::R](spctrl::R) reader structure"]
impl crate::Readable for SPCTRL {}
#[doc = "`write(|w| ..)` method takes [spctrl::W](spctrl::W) writer structure"]
impl crate::Writable for SPCTRL {}
#[doc = "SPIX Controller Special Control Register."]
pub mod spctrl;
#[doc = "SPIX Controller Interrupt Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](intfl) module"]
pub type INTFL = crate::Reg<u32, _INTFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFL;
#[doc = "`read()` method returns [intfl::R](intfl::R) reader structure"]
impl crate::Readable for INTFL {}
#[doc = "`write(|w| ..)` method takes [intfl::W](intfl::W) writer structure"]
impl crate::Writable for INTFL {}
#[doc = "SPIX Controller Interrupt Status Register."]
pub mod intfl;
#[doc = "SPIX Controller Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "SPIX Controller Interrupt Enable Register."]
pub mod inten;
