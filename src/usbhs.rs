#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function address register."]
    pub faddr: FADDR,
    #[doc = "0x01 - Power management register."]
    pub power: POWER,
    #[doc = "0x02 - Interrupt register for EP0 and IN EP1-15."]
    pub intrin: INTRIN,
    #[doc = "0x04 - Interrupt register for OUT EP 1-15."]
    pub introut: INTROUT,
    #[doc = "0x06 - Interrupt enable for EP 0 and IN EP 1-15."]
    pub intrinen: INTRINEN,
    #[doc = "0x08 - Interrupt enable for OUT EP 1-15."]
    pub introuten: INTROUTEN,
    #[doc = "0x0a - Interrupt register for common USB interrupts."]
    pub intrusb: INTRUSB,
    #[doc = "0x0b - Interrupt enable for common USB interrupts."]
    pub intrusben: INTRUSBEN,
    #[doc = "0x0c - Frame number."]
    pub frame: FRAME,
    #[doc = "0x0e - Index for banked registers."]
    pub index: INDEX,
    #[doc = "0x0f - USB 2.0 test mode enable register."]
    pub testmode: TESTMODE,
    #[doc = "0x10 - Maximum packet size for INx endpoint (x == INDEX)."]
    pub inmaxp: INMAXP,
    _reserved_12_csr0: [u8; 1usize],
    #[doc = "0x13 - Control status upper register for INx endpoint (x == INDEX)."]
    pub incsru: INCSRU,
    #[doc = "0x14 - Maximum packet size for OUTx endpoint (x == INDEX)."]
    pub outmaxp: OUTMAXP,
    #[doc = "0x16 - Control status lower register for OUTx endpoint (x == INDEX)."]
    pub outcsrl: OUTCSRL,
    #[doc = "0x17 - Control status upper register for OUTx endpoint (x == INDEX)."]
    pub outcsru: OUTCSRU,
    _reserved_17_count0: [u8; 2usize],
    _reserved18: [u8; 6usize],
    #[doc = "0x20 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo0: FIFO0,
    #[doc = "0x24 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo1: FIFO1,
    #[doc = "0x28 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo2: FIFO2,
    #[doc = "0x2c - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo3: FIFO3,
    #[doc = "0x30 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo4: FIFO4,
    #[doc = "0x34 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo5: FIFO5,
    #[doc = "0x38 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo6: FIFO6,
    #[doc = "0x3c - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo7: FIFO7,
    #[doc = "0x40 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo8: FIFO8,
    #[doc = "0x44 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo9: FIFO9,
    #[doc = "0x48 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo10: FIFO10,
    #[doc = "0x4c - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo11: FIFO11,
    #[doc = "0x50 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo12: FIFO12,
    #[doc = "0x54 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo13: FIFO13,
    #[doc = "0x58 - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo14: FIFO14,
    #[doc = "0x5c - Read for OUT data FIFO, write for IN data FIFO."]
    pub fifo15: FIFO15,
    _reserved34: [u8; 12usize],
    #[doc = "0x6c - HWVERS"]
    pub hwvers: HWVERS,
    _reserved35: [u8; 10usize],
    #[doc = "0x78 - Endpoint hardware information."]
    pub epinfo: EPINFO,
    #[doc = "0x79 - RAM width and DMA hardware information."]
    pub raminfo: RAMINFO,
    #[doc = "0x7a - Software reset register."]
    pub softreset: SOFTRESET,
    #[doc = "0x7b - DMA timing control register."]
    pub earlydma: EARLYDMA,
    _reserved39: [u8; 4usize],
    #[doc = "0x80 - Chirp timeout timer setting."]
    pub ctuch: CTUCH,
    #[doc = "0x82 - Sets delay between HS resume to UTM normal operating mode."]
    pub cthsrtn: CTHSRTN,
    _reserved41: [u8; 908usize],
    #[doc = "0x410 - M31_PHY_PONRST"]
    pub m31_phy_ponrst: M31_PHY_PONRST,
    #[doc = "0x414 - M31_PHY_NONCRY_RSTB"]
    pub m31_phy_noncry_rstb: M31_PHY_NONCRY_RSTB,
    #[doc = "0x418 - M31_PHY_NONCRY_EN"]
    pub m31_phy_noncry_en: M31_PHY_NONCRY_EN,
    _reserved44: [u8; 20usize],
    #[doc = "0x430 - M31_PHY_PLL_EN"]
    pub m31_phy_pll_en: M31_PHY_PLL_EN,
    _reserved45: [u8; 8usize],
    #[doc = "0x43c - M31_PHY_OSCOUTEN"]
    pub m31_phy_oscouten: M31_PHY_OSCOUTEN,
    _reserved46: [u8; 8usize],
    #[doc = "0x448 - M31_PHY_CORECLKIN"]
    pub m31_phy_coreclkin: M31_PHY_CORECLKIN,
    #[doc = "0x44c - M31_PHY_XTLSEL"]
    pub m31_phy_xtlsel: M31_PHY_XTLSEL,
    _reserved48: [u8; 12usize],
    #[doc = "0x45c - M31_PHY_OUTCLKSEL"]
    pub m31_phy_outclksel: M31_PHY_OUTCLKSEL,
    _reserved49: [u8; 56usize],
    #[doc = "0x498 - USB Added Maxim Interrupt Flag Register."]
    pub mxm_int: MXM_INT,
    #[doc = "0x49c - USB Added Maxim Interrupt Enable Register."]
    pub mxm_int_en: MXM_INT_EN,
    #[doc = "0x4a0 - USB Added Maxim Suspend Register."]
    pub mxm_suspend: MXM_SUSPEND,
    #[doc = "0x4a4 - USB Added Maxim Power Status Register"]
    pub mxm_reg_a4: MXM_REG_A4,
}
impl RegisterBlock {
    #[doc = "0x12 - Control status lower register for INx endpoint (x == INDEX)."]
    #[inline(always)]
    pub fn incsrl(&self) -> &INCSRL {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const INCSRL) }
    }
    #[doc = "0x12 - Control status lower register for INx endpoint (x == INDEX)."]
    #[inline(always)]
    pub fn incsrl_mut(&self) -> &mut INCSRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(18usize) as *mut INCSRL) }
    }
    #[doc = "0x12 - Control status register for EP 0 (when INDEX == 0)."]
    #[inline(always)]
    pub fn csr0(&self) -> &CSR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(18usize) as *const CSR0) }
    }
    #[doc = "0x12 - Control status register for EP 0 (when INDEX == 0)."]
    #[inline(always)]
    pub fn csr0_mut(&self) -> &mut CSR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(18usize) as *mut CSR0) }
    }
    #[doc = "0x18 - Number of received bytes in OUT EPx FIFO (x == INDEX)."]
    #[inline(always)]
    pub fn outcount(&self) -> &OUTCOUNT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const OUTCOUNT) }
    }
    #[doc = "0x18 - Number of received bytes in OUT EPx FIFO (x == INDEX)."]
    #[inline(always)]
    pub fn outcount_mut(&self) -> &mut OUTCOUNT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut OUTCOUNT) }
    }
    #[doc = "0x18 - Number of received bytes in EP 0 FIFO (INDEX == 0)."]
    #[inline(always)]
    pub fn count0(&self) -> &COUNT0 {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const COUNT0) }
    }
    #[doc = "0x18 - Number of received bytes in EP 0 FIFO (INDEX == 0)."]
    #[inline(always)]
    pub fn count0_mut(&self) -> &mut COUNT0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut COUNT0) }
    }
}
#[doc = "Function address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](faddr) module"]
pub type FADDR = crate::Reg<u8, _FADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FADDR;
#[doc = "`read()` method returns [faddr::R](faddr::R) reader structure"]
impl crate::Readable for FADDR {}
#[doc = "`write(|w| ..)` method takes [faddr::W](faddr::W) writer structure"]
impl crate::Writable for FADDR {}
#[doc = "Function address register."]
pub mod faddr;
#[doc = "Power management register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u8, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "Power management register."]
pub mod power;
#[doc = "Interrupt register for EP0 and IN EP1-15.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrin](intrin) module"]
pub type INTRIN = crate::Reg<u16, _INTRIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRIN;
#[doc = "`read()` method returns [intrin::R](intrin::R) reader structure"]
impl crate::Readable for INTRIN {}
#[doc = "Interrupt register for EP0 and IN EP1-15."]
pub mod intrin;
#[doc = "Interrupt register for OUT EP 1-15.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [introut](introut) module"]
pub type INTROUT = crate::Reg<u16, _INTROUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTROUT;
#[doc = "`read()` method returns [introut::R](introut::R) reader structure"]
impl crate::Readable for INTROUT {}
#[doc = "Interrupt register for OUT EP 1-15."]
pub mod introut;
#[doc = "Interrupt enable for EP 0 and IN EP 1-15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrinen](intrinen) module"]
pub type INTRINEN = crate::Reg<u16, _INTRINEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRINEN;
#[doc = "`read()` method returns [intrinen::R](intrinen::R) reader structure"]
impl crate::Readable for INTRINEN {}
#[doc = "`write(|w| ..)` method takes [intrinen::W](intrinen::W) writer structure"]
impl crate::Writable for INTRINEN {}
#[doc = "Interrupt enable for EP 0 and IN EP 1-15."]
pub mod intrinen;
#[doc = "Interrupt enable for OUT EP 1-15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [introuten](introuten) module"]
pub type INTROUTEN = crate::Reg<u16, _INTROUTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTROUTEN;
#[doc = "`read()` method returns [introuten::R](introuten::R) reader structure"]
impl crate::Readable for INTROUTEN {}
#[doc = "`write(|w| ..)` method takes [introuten::W](introuten::W) writer structure"]
impl crate::Writable for INTROUTEN {}
#[doc = "Interrupt enable for OUT EP 1-15."]
pub mod introuten;
#[doc = "Interrupt register for common USB interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrusb](intrusb) module"]
pub type INTRUSB = crate::Reg<u8, _INTRUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRUSB;
#[doc = "`read()` method returns [intrusb::R](intrusb::R) reader structure"]
impl crate::Readable for INTRUSB {}
#[doc = "Interrupt register for common USB interrupts."]
pub mod intrusb;
#[doc = "Interrupt enable for common USB interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrusben](intrusben) module"]
pub type INTRUSBEN = crate::Reg<u8, _INTRUSBEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRUSBEN;
#[doc = "`read()` method returns [intrusben::R](intrusben::R) reader structure"]
impl crate::Readable for INTRUSBEN {}
#[doc = "`write(|w| ..)` method takes [intrusben::W](intrusben::W) writer structure"]
impl crate::Writable for INTRUSBEN {}
#[doc = "Interrupt enable for common USB interrupts."]
pub mod intrusben;
#[doc = "Frame number.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame](frame) module"]
pub type FRAME = crate::Reg<u16, _FRAME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAME;
#[doc = "`read()` method returns [frame::R](frame::R) reader structure"]
impl crate::Readable for FRAME {}
#[doc = "Frame number."]
pub mod frame;
#[doc = "Index for banked registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [index](index) module"]
pub type INDEX = crate::Reg<u8, _INDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDEX;
#[doc = "`read()` method returns [index::R](index::R) reader structure"]
impl crate::Readable for INDEX {}
#[doc = "`write(|w| ..)` method takes [index::W](index::W) writer structure"]
impl crate::Writable for INDEX {}
#[doc = "Index for banked registers."]
pub mod index;
#[doc = "USB 2.0 test mode enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testmode](testmode) module"]
pub type TESTMODE = crate::Reg<u8, _TESTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESTMODE;
#[doc = "`read()` method returns [testmode::R](testmode::R) reader structure"]
impl crate::Readable for TESTMODE {}
#[doc = "`write(|w| ..)` method takes [testmode::W](testmode::W) writer structure"]
impl crate::Writable for TESTMODE {}
#[doc = "USB 2.0 test mode enable register."]
pub mod testmode;
#[doc = "Maximum packet size for INx endpoint (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inmaxp](inmaxp) module"]
pub type INMAXP = crate::Reg<u16, _INMAXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INMAXP;
#[doc = "`read()` method returns [inmaxp::R](inmaxp::R) reader structure"]
impl crate::Readable for INMAXP {}
#[doc = "`write(|w| ..)` method takes [inmaxp::W](inmaxp::W) writer structure"]
impl crate::Writable for INMAXP {}
#[doc = "Maximum packet size for INx endpoint (x == INDEX)."]
pub mod inmaxp;
#[doc = "Control status register for EP 0 (when INDEX == 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr0](csr0) module"]
pub type CSR0 = crate::Reg<u8, _CSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR0;
#[doc = "`read()` method returns [csr0::R](csr0::R) reader structure"]
impl crate::Readable for CSR0 {}
#[doc = "`write(|w| ..)` method takes [csr0::W](csr0::W) writer structure"]
impl crate::Writable for CSR0 {}
#[doc = "Control status register for EP 0 (when INDEX == 0)."]
pub mod csr0;
#[doc = "Control status lower register for INx endpoint (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [incsrl](incsrl) module"]
pub type INCSRL = crate::Reg<u8, _INCSRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INCSRL;
#[doc = "`read()` method returns [incsrl::R](incsrl::R) reader structure"]
impl crate::Readable for INCSRL {}
#[doc = "`write(|w| ..)` method takes [incsrl::W](incsrl::W) writer structure"]
impl crate::Writable for INCSRL {}
#[doc = "Control status lower register for INx endpoint (x == INDEX)."]
pub mod incsrl;
#[doc = "Control status upper register for INx endpoint (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [incsru](incsru) module"]
pub type INCSRU = crate::Reg<u8, _INCSRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INCSRU;
#[doc = "`read()` method returns [incsru::R](incsru::R) reader structure"]
impl crate::Readable for INCSRU {}
#[doc = "`write(|w| ..)` method takes [incsru::W](incsru::W) writer structure"]
impl crate::Writable for INCSRU {}
#[doc = "Control status upper register for INx endpoint (x == INDEX)."]
pub mod incsru;
#[doc = "Maximum packet size for OUTx endpoint (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outmaxp](outmaxp) module"]
pub type OUTMAXP = crate::Reg<u16, _OUTMAXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTMAXP;
#[doc = "`read()` method returns [outmaxp::R](outmaxp::R) reader structure"]
impl crate::Readable for OUTMAXP {}
#[doc = "`write(|w| ..)` method takes [outmaxp::W](outmaxp::W) writer structure"]
impl crate::Writable for OUTMAXP {}
#[doc = "Maximum packet size for OUTx endpoint (x == INDEX)."]
pub mod outmaxp;
#[doc = "Control status lower register for OUTx endpoint (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcsrl](outcsrl) module"]
pub type OUTCSRL = crate::Reg<u8, _OUTCSRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCSRL;
#[doc = "`read()` method returns [outcsrl::R](outcsrl::R) reader structure"]
impl crate::Readable for OUTCSRL {}
#[doc = "`write(|w| ..)` method takes [outcsrl::W](outcsrl::W) writer structure"]
impl crate::Writable for OUTCSRL {}
#[doc = "Control status lower register for OUTx endpoint (x == INDEX)."]
pub mod outcsrl;
#[doc = "Control status upper register for OUTx endpoint (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcsru](outcsru) module"]
pub type OUTCSRU = crate::Reg<u8, _OUTCSRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCSRU;
#[doc = "`read()` method returns [outcsru::R](outcsru::R) reader structure"]
impl crate::Readable for OUTCSRU {}
#[doc = "`write(|w| ..)` method takes [outcsru::W](outcsru::W) writer structure"]
impl crate::Writable for OUTCSRU {}
#[doc = "Control status upper register for OUTx endpoint (x == INDEX)."]
pub mod outcsru;
#[doc = "Number of received bytes in EP 0 FIFO (INDEX == 0).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count0](count0) module"]
pub type COUNT0 = crate::Reg<u16, _COUNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT0;
#[doc = "`read()` method returns [count0::R](count0::R) reader structure"]
impl crate::Readable for COUNT0 {}
#[doc = "Number of received bytes in EP 0 FIFO (INDEX == 0)."]
pub mod count0;
#[doc = "Number of received bytes in OUT EPx FIFO (x == INDEX).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcount](outcount) module"]
pub type OUTCOUNT = crate::Reg<u16, _OUTCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCOUNT;
#[doc = "`read()` method returns [outcount::R](outcount::R) reader structure"]
impl crate::Readable for OUTCOUNT {}
#[doc = "Number of received bytes in OUT EPx FIFO (x == INDEX)."]
pub mod outcount;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo0](fifo0) module"]
pub type FIFO0 = crate::Reg<u32, _FIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO0;
#[doc = "`read()` method returns [fifo0::R](fifo0::R) reader structure"]
impl crate::Readable for FIFO0 {}
#[doc = "`write(|w| ..)` method takes [fifo0::W](fifo0::W) writer structure"]
impl crate::Writable for FIFO0 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo0;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo1](fifo1) module"]
pub type FIFO1 = crate::Reg<u32, _FIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO1;
#[doc = "`read()` method returns [fifo1::R](fifo1::R) reader structure"]
impl crate::Readable for FIFO1 {}
#[doc = "`write(|w| ..)` method takes [fifo1::W](fifo1::W) writer structure"]
impl crate::Writable for FIFO1 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo1;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo2](fifo2) module"]
pub type FIFO2 = crate::Reg<u32, _FIFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO2;
#[doc = "`read()` method returns [fifo2::R](fifo2::R) reader structure"]
impl crate::Readable for FIFO2 {}
#[doc = "`write(|w| ..)` method takes [fifo2::W](fifo2::W) writer structure"]
impl crate::Writable for FIFO2 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo2;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo3](fifo3) module"]
pub type FIFO3 = crate::Reg<u32, _FIFO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO3;
#[doc = "`read()` method returns [fifo3::R](fifo3::R) reader structure"]
impl crate::Readable for FIFO3 {}
#[doc = "`write(|w| ..)` method takes [fifo3::W](fifo3::W) writer structure"]
impl crate::Writable for FIFO3 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo3;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo4](fifo4) module"]
pub type FIFO4 = crate::Reg<u32, _FIFO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO4;
#[doc = "`read()` method returns [fifo4::R](fifo4::R) reader structure"]
impl crate::Readable for FIFO4 {}
#[doc = "`write(|w| ..)` method takes [fifo4::W](fifo4::W) writer structure"]
impl crate::Writable for FIFO4 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo4;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo5](fifo5) module"]
pub type FIFO5 = crate::Reg<u32, _FIFO5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO5;
#[doc = "`read()` method returns [fifo5::R](fifo5::R) reader structure"]
impl crate::Readable for FIFO5 {}
#[doc = "`write(|w| ..)` method takes [fifo5::W](fifo5::W) writer structure"]
impl crate::Writable for FIFO5 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo5;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo6](fifo6) module"]
pub type FIFO6 = crate::Reg<u32, _FIFO6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO6;
#[doc = "`read()` method returns [fifo6::R](fifo6::R) reader structure"]
impl crate::Readable for FIFO6 {}
#[doc = "`write(|w| ..)` method takes [fifo6::W](fifo6::W) writer structure"]
impl crate::Writable for FIFO6 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo6;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo7](fifo7) module"]
pub type FIFO7 = crate::Reg<u32, _FIFO7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO7;
#[doc = "`read()` method returns [fifo7::R](fifo7::R) reader structure"]
impl crate::Readable for FIFO7 {}
#[doc = "`write(|w| ..)` method takes [fifo7::W](fifo7::W) writer structure"]
impl crate::Writable for FIFO7 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo7;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo8](fifo8) module"]
pub type FIFO8 = crate::Reg<u32, _FIFO8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO8;
#[doc = "`read()` method returns [fifo8::R](fifo8::R) reader structure"]
impl crate::Readable for FIFO8 {}
#[doc = "`write(|w| ..)` method takes [fifo8::W](fifo8::W) writer structure"]
impl crate::Writable for FIFO8 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo8;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo9](fifo9) module"]
pub type FIFO9 = crate::Reg<u32, _FIFO9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO9;
#[doc = "`read()` method returns [fifo9::R](fifo9::R) reader structure"]
impl crate::Readable for FIFO9 {}
#[doc = "`write(|w| ..)` method takes [fifo9::W](fifo9::W) writer structure"]
impl crate::Writable for FIFO9 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo9;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo10](fifo10) module"]
pub type FIFO10 = crate::Reg<u32, _FIFO10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO10;
#[doc = "`read()` method returns [fifo10::R](fifo10::R) reader structure"]
impl crate::Readable for FIFO10 {}
#[doc = "`write(|w| ..)` method takes [fifo10::W](fifo10::W) writer structure"]
impl crate::Writable for FIFO10 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo10;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo11](fifo11) module"]
pub type FIFO11 = crate::Reg<u32, _FIFO11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO11;
#[doc = "`read()` method returns [fifo11::R](fifo11::R) reader structure"]
impl crate::Readable for FIFO11 {}
#[doc = "`write(|w| ..)` method takes [fifo11::W](fifo11::W) writer structure"]
impl crate::Writable for FIFO11 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo11;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo12](fifo12) module"]
pub type FIFO12 = crate::Reg<u32, _FIFO12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO12;
#[doc = "`read()` method returns [fifo12::R](fifo12::R) reader structure"]
impl crate::Readable for FIFO12 {}
#[doc = "`write(|w| ..)` method takes [fifo12::W](fifo12::W) writer structure"]
impl crate::Writable for FIFO12 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo12;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo13](fifo13) module"]
pub type FIFO13 = crate::Reg<u32, _FIFO13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO13;
#[doc = "`read()` method returns [fifo13::R](fifo13::R) reader structure"]
impl crate::Readable for FIFO13 {}
#[doc = "`write(|w| ..)` method takes [fifo13::W](fifo13::W) writer structure"]
impl crate::Writable for FIFO13 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo13;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo14](fifo14) module"]
pub type FIFO14 = crate::Reg<u32, _FIFO14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO14;
#[doc = "`read()` method returns [fifo14::R](fifo14::R) reader structure"]
impl crate::Readable for FIFO14 {}
#[doc = "`write(|w| ..)` method takes [fifo14::W](fifo14::W) writer structure"]
impl crate::Writable for FIFO14 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo14;
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo15](fifo15) module"]
pub type FIFO15 = crate::Reg<u32, _FIFO15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO15;
#[doc = "`read()` method returns [fifo15::R](fifo15::R) reader structure"]
impl crate::Readable for FIFO15 {}
#[doc = "`write(|w| ..)` method takes [fifo15::W](fifo15::W) writer structure"]
impl crate::Writable for FIFO15 {}
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo15;
#[doc = "HWVERS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvers](hwvers) module"]
pub type HWVERS = crate::Reg<u16, _HWVERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVERS;
#[doc = "`read()` method returns [hwvers::R](hwvers::R) reader structure"]
impl crate::Readable for HWVERS {}
#[doc = "`write(|w| ..)` method takes [hwvers::W](hwvers::W) writer structure"]
impl crate::Writable for HWVERS {}
#[doc = "HWVERS"]
pub mod hwvers;
#[doc = "Endpoint hardware information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinfo](epinfo) module"]
pub type EPINFO = crate::Reg<u8, _EPINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINFO;
#[doc = "`read()` method returns [epinfo::R](epinfo::R) reader structure"]
impl crate::Readable for EPINFO {}
#[doc = "Endpoint hardware information."]
pub mod epinfo;
#[doc = "RAM width and DMA hardware information.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raminfo](raminfo) module"]
pub type RAMINFO = crate::Reg<u8, _RAMINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMINFO;
#[doc = "`read()` method returns [raminfo::R](raminfo::R) reader structure"]
impl crate::Readable for RAMINFO {}
#[doc = "RAM width and DMA hardware information."]
pub mod raminfo;
#[doc = "Software reset register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softreset](softreset) module"]
pub type SOFTRESET = crate::Reg<u8, _SOFTRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTRESET;
#[doc = "`read()` method returns [softreset::R](softreset::R) reader structure"]
impl crate::Readable for SOFTRESET {}
#[doc = "`write(|w| ..)` method takes [softreset::W](softreset::W) writer structure"]
impl crate::Writable for SOFTRESET {}
#[doc = "Software reset register."]
pub mod softreset;
#[doc = "DMA timing control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [earlydma](earlydma) module"]
pub type EARLYDMA = crate::Reg<u8, _EARLYDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARLYDMA;
#[doc = "`read()` method returns [earlydma::R](earlydma::R) reader structure"]
impl crate::Readable for EARLYDMA {}
#[doc = "`write(|w| ..)` method takes [earlydma::W](earlydma::W) writer structure"]
impl crate::Writable for EARLYDMA {}
#[doc = "DMA timing control register."]
pub mod earlydma;
#[doc = "Chirp timeout timer setting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctuch](ctuch) module"]
pub type CTUCH = crate::Reg<u16, _CTUCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTUCH;
#[doc = "`read()` method returns [ctuch::R](ctuch::R) reader structure"]
impl crate::Readable for CTUCH {}
#[doc = "`write(|w| ..)` method takes [ctuch::W](ctuch::W) writer structure"]
impl crate::Writable for CTUCH {}
#[doc = "Chirp timeout timer setting."]
pub mod ctuch;
#[doc = "Sets delay between HS resume to UTM normal operating mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cthsrtn](cthsrtn) module"]
pub type CTHSRTN = crate::Reg<u16, _CTHSRTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTHSRTN;
#[doc = "`read()` method returns [cthsrtn::R](cthsrtn::R) reader structure"]
impl crate::Readable for CTHSRTN {}
#[doc = "`write(|w| ..)` method takes [cthsrtn::W](cthsrtn::W) writer structure"]
impl crate::Writable for CTHSRTN {}
#[doc = "Sets delay between HS resume to UTM normal operating mode."]
pub mod cthsrtn;
#[doc = "M31_PHY_PONRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_ponrst](m31_phy_ponrst) module"]
pub type M31_PHY_PONRST = crate::Reg<u32, _M31_PHY_PONRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_PONRST;
#[doc = "`read()` method returns [m31_phy_ponrst::R](m31_phy_ponrst::R) reader structure"]
impl crate::Readable for M31_PHY_PONRST {}
#[doc = "`write(|w| ..)` method takes [m31_phy_ponrst::W](m31_phy_ponrst::W) writer structure"]
impl crate::Writable for M31_PHY_PONRST {}
#[doc = "M31_PHY_PONRST"]
pub mod m31_phy_ponrst;
#[doc = "M31_PHY_NONCRY_RSTB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_noncry_rstb](m31_phy_noncry_rstb) module"]
pub type M31_PHY_NONCRY_RSTB = crate::Reg<u32, _M31_PHY_NONCRY_RSTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_NONCRY_RSTB;
#[doc = "`read()` method returns [m31_phy_noncry_rstb::R](m31_phy_noncry_rstb::R) reader structure"]
impl crate::Readable for M31_PHY_NONCRY_RSTB {}
#[doc = "`write(|w| ..)` method takes [m31_phy_noncry_rstb::W](m31_phy_noncry_rstb::W) writer structure"]
impl crate::Writable for M31_PHY_NONCRY_RSTB {}
#[doc = "M31_PHY_NONCRY_RSTB"]
pub mod m31_phy_noncry_rstb;
#[doc = "M31_PHY_NONCRY_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_noncry_en](m31_phy_noncry_en) module"]
pub type M31_PHY_NONCRY_EN = crate::Reg<u32, _M31_PHY_NONCRY_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_NONCRY_EN;
#[doc = "`read()` method returns [m31_phy_noncry_en::R](m31_phy_noncry_en::R) reader structure"]
impl crate::Readable for M31_PHY_NONCRY_EN {}
#[doc = "`write(|w| ..)` method takes [m31_phy_noncry_en::W](m31_phy_noncry_en::W) writer structure"]
impl crate::Writable for M31_PHY_NONCRY_EN {}
#[doc = "M31_PHY_NONCRY_EN"]
pub mod m31_phy_noncry_en;
#[doc = "M31_PHY_PLL_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_pll_en](m31_phy_pll_en) module"]
pub type M31_PHY_PLL_EN = crate::Reg<u32, _M31_PHY_PLL_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_PLL_EN;
#[doc = "`read()` method returns [m31_phy_pll_en::R](m31_phy_pll_en::R) reader structure"]
impl crate::Readable for M31_PHY_PLL_EN {}
#[doc = "`write(|w| ..)` method takes [m31_phy_pll_en::W](m31_phy_pll_en::W) writer structure"]
impl crate::Writable for M31_PHY_PLL_EN {}
#[doc = "M31_PHY_PLL_EN"]
pub mod m31_phy_pll_en;
#[doc = "M31_PHY_OSCOUTEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_oscouten](m31_phy_oscouten) module"]
pub type M31_PHY_OSCOUTEN = crate::Reg<u32, _M31_PHY_OSCOUTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_OSCOUTEN;
#[doc = "`read()` method returns [m31_phy_oscouten::R](m31_phy_oscouten::R) reader structure"]
impl crate::Readable for M31_PHY_OSCOUTEN {}
#[doc = "`write(|w| ..)` method takes [m31_phy_oscouten::W](m31_phy_oscouten::W) writer structure"]
impl crate::Writable for M31_PHY_OSCOUTEN {}
#[doc = "M31_PHY_OSCOUTEN"]
pub mod m31_phy_oscouten;
#[doc = "M31_PHY_CORECLKIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_coreclkin](m31_phy_coreclkin) module"]
pub type M31_PHY_CORECLKIN = crate::Reg<u32, _M31_PHY_CORECLKIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_CORECLKIN;
#[doc = "`read()` method returns [m31_phy_coreclkin::R](m31_phy_coreclkin::R) reader structure"]
impl crate::Readable for M31_PHY_CORECLKIN {}
#[doc = "`write(|w| ..)` method takes [m31_phy_coreclkin::W](m31_phy_coreclkin::W) writer structure"]
impl crate::Writable for M31_PHY_CORECLKIN {}
#[doc = "M31_PHY_CORECLKIN"]
pub mod m31_phy_coreclkin;
#[doc = "M31_PHY_XTLSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_xtlsel](m31_phy_xtlsel) module"]
pub type M31_PHY_XTLSEL = crate::Reg<u32, _M31_PHY_XTLSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_XTLSEL;
#[doc = "`read()` method returns [m31_phy_xtlsel::R](m31_phy_xtlsel::R) reader structure"]
impl crate::Readable for M31_PHY_XTLSEL {}
#[doc = "`write(|w| ..)` method takes [m31_phy_xtlsel::W](m31_phy_xtlsel::W) writer structure"]
impl crate::Writable for M31_PHY_XTLSEL {}
#[doc = "M31_PHY_XTLSEL"]
pub mod m31_phy_xtlsel;
#[doc = "M31_PHY_OUTCLKSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m31_phy_outclksel](m31_phy_outclksel) module"]
pub type M31_PHY_OUTCLKSEL = crate::Reg<u32, _M31_PHY_OUTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M31_PHY_OUTCLKSEL;
#[doc = "`read()` method returns [m31_phy_outclksel::R](m31_phy_outclksel::R) reader structure"]
impl crate::Readable for M31_PHY_OUTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [m31_phy_outclksel::W](m31_phy_outclksel::W) writer structure"]
impl crate::Writable for M31_PHY_OUTCLKSEL {}
#[doc = "M31_PHY_OUTCLKSEL"]
pub mod m31_phy_outclksel;
#[doc = "USB Added Maxim Interrupt Flag Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mxm_int](mxm_int) module"]
pub type MXM_INT = crate::Reg<u32, _MXM_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MXM_INT;
#[doc = "`read()` method returns [mxm_int::R](mxm_int::R) reader structure"]
impl crate::Readable for MXM_INT {}
#[doc = "`write(|w| ..)` method takes [mxm_int::W](mxm_int::W) writer structure"]
impl crate::Writable for MXM_INT {}
#[doc = "USB Added Maxim Interrupt Flag Register."]
pub mod mxm_int;
#[doc = "USB Added Maxim Interrupt Enable Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mxm_int_en](mxm_int_en) module"]
pub type MXM_INT_EN = crate::Reg<u32, _MXM_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MXM_INT_EN;
#[doc = "`read()` method returns [mxm_int_en::R](mxm_int_en::R) reader structure"]
impl crate::Readable for MXM_INT_EN {}
#[doc = "`write(|w| ..)` method takes [mxm_int_en::W](mxm_int_en::W) writer structure"]
impl crate::Writable for MXM_INT_EN {}
#[doc = "USB Added Maxim Interrupt Enable Register."]
pub mod mxm_int_en;
#[doc = "USB Added Maxim Suspend Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mxm_suspend](mxm_suspend) module"]
pub type MXM_SUSPEND = crate::Reg<u32, _MXM_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MXM_SUSPEND;
#[doc = "`read()` method returns [mxm_suspend::R](mxm_suspend::R) reader structure"]
impl crate::Readable for MXM_SUSPEND {}
#[doc = "`write(|w| ..)` method takes [mxm_suspend::W](mxm_suspend::W) writer structure"]
impl crate::Writable for MXM_SUSPEND {}
#[doc = "USB Added Maxim Suspend Register."]
pub mod mxm_suspend;
#[doc = "USB Added Maxim Power Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mxm_reg_a4](mxm_reg_a4) module"]
pub type MXM_REG_A4 = crate::Reg<u32, _MXM_REG_A4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MXM_REG_A4;
#[doc = "`read()` method returns [mxm_reg_a4::R](mxm_reg_a4::R) reader structure"]
impl crate::Readable for MXM_REG_A4 {}
#[doc = "`write(|w| ..)` method takes [mxm_reg_a4::W](mxm_reg_a4::W) writer structure"]
impl crate::Writable for MXM_REG_A4 {}
#[doc = "USB Added Maxim Power Status Register"]
pub mod mxm_reg_a4;
