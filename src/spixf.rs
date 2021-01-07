#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPIX Configuration Register."]
    pub cfg: CFG,
    #[doc = "0x04 - SPIX Fetch Control Register."]
    pub fetch_ctrl: FETCH_CTRL,
    #[doc = "0x08 - SPIX Mode Control Register."]
    pub mode_ctrl: MODE_CTRL,
    #[doc = "0x0c - SPIX Mode Data Register."]
    pub mode_data: MODE_DATA,
    #[doc = "0x10 - SPIX Feedback Control Register."]
    pub sclk_fb_ctrl: SCLK_FB_CTRL,
    _reserved5: [u8; 8usize],
    #[doc = "0x1c - SPIX IO Control Register."]
    pub io_ctrl: IO_CTRL,
    #[doc = "0x20 - SPIX Memory Security Control Register."]
    pub memseccn: MEMSECCN,
}
#[doc = "SPIX Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "SPIX Configuration Register."]
pub mod cfg;
#[doc = "SPIX Fetch Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetch_ctrl](fetch_ctrl) module"]
pub type FETCH_CTRL = crate::Reg<u32, _FETCH_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FETCH_CTRL;
#[doc = "`read()` method returns [fetch_ctrl::R](fetch_ctrl::R) reader structure"]
impl crate::Readable for FETCH_CTRL {}
#[doc = "`write(|w| ..)` method takes [fetch_ctrl::W](fetch_ctrl::W) writer structure"]
impl crate::Writable for FETCH_CTRL {}
#[doc = "SPIX Fetch Control Register."]
pub mod fetch_ctrl;
#[doc = "SPIX Mode Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_ctrl](mode_ctrl) module"]
pub type MODE_CTRL = crate::Reg<u32, _MODE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE_CTRL;
#[doc = "`read()` method returns [mode_ctrl::R](mode_ctrl::R) reader structure"]
impl crate::Readable for MODE_CTRL {}
#[doc = "`write(|w| ..)` method takes [mode_ctrl::W](mode_ctrl::W) writer structure"]
impl crate::Writable for MODE_CTRL {}
#[doc = "SPIX Mode Control Register."]
pub mod mode_ctrl;
#[doc = "SPIX Mode Data Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_data](mode_data) module"]
pub type MODE_DATA = crate::Reg<u32, _MODE_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE_DATA;
#[doc = "`read()` method returns [mode_data::R](mode_data::R) reader structure"]
impl crate::Readable for MODE_DATA {}
#[doc = "`write(|w| ..)` method takes [mode_data::W](mode_data::W) writer structure"]
impl crate::Writable for MODE_DATA {}
#[doc = "SPIX Mode Data Register."]
pub mod mode_data;
#[doc = "SPIX Feedback Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclk_fb_ctrl](sclk_fb_ctrl) module"]
pub type SCLK_FB_CTRL = crate::Reg<u32, _SCLK_FB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCLK_FB_CTRL;
#[doc = "`read()` method returns [sclk_fb_ctrl::R](sclk_fb_ctrl::R) reader structure"]
impl crate::Readable for SCLK_FB_CTRL {}
#[doc = "`write(|w| ..)` method takes [sclk_fb_ctrl::W](sclk_fb_ctrl::W) writer structure"]
impl crate::Writable for SCLK_FB_CTRL {}
#[doc = "SPIX Feedback Control Register."]
pub mod sclk_fb_ctrl;
#[doc = "SPIX IO Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_ctrl](io_ctrl) module"]
pub type IO_CTRL = crate::Reg<u32, _IO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO_CTRL;
#[doc = "`read()` method returns [io_ctrl::R](io_ctrl::R) reader structure"]
impl crate::Readable for IO_CTRL {}
#[doc = "`write(|w| ..)` method takes [io_ctrl::W](io_ctrl::W) writer structure"]
impl crate::Writable for IO_CTRL {}
#[doc = "SPIX IO Control Register."]
pub mod io_ctrl;
#[doc = "SPIX Memory Security Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memseccn](memseccn) module"]
pub type MEMSECCN = crate::Reg<u32, _MEMSECCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMSECCN;
#[doc = "`read()` method returns [memseccn::R](memseccn::R) reader structure"]
impl crate::Readable for MEMSECCN {}
#[doc = "`write(|w| ..)` method takes [memseccn::W](memseccn::W) writer structure"]
impl crate::Writable for MEMSECCN {}
#[doc = "SPIX Memory Security Control Register."]
pub mod memseccn;
