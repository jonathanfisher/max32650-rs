#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Q30E Instruction Pointer."]
    pub ip: IP,
    #[doc = "0x04 - Q30E Stack Pointer."]
    pub sp: SP,
    #[doc = "0x08 - Q30E Data Pointer 0."]
    pub dp0: DP0,
    #[doc = "0x0c - Q30E Data Pointer 1."]
    pub dp1: DP1,
    #[doc = "0x10 - Q30E Frame Pointer Base."]
    pub bp: BP,
    #[doc = "0x14 - Q30E Frame Pointer Offset."]
    pub offs: OFFS,
    #[doc = "0x18 - Q30E Loop Counter 0."]
    pub lc0: LC0,
    #[doc = "0x1c - Q30E Loop Counter 1."]
    pub lc1: LC1,
    #[doc = "0x20 - Q30E Accumulator 0."]
    pub a0: A0,
    #[doc = "0x24 - Q30E Accumulator 1."]
    pub a1: A1,
    #[doc = "0x28 - Q30E Accumulator 2."]
    pub a2: A2,
    #[doc = "0x2c - Q30E Accumulator 3."]
    pub a3: A3,
    #[doc = "0x30 - Q30E Watchdog Control."]
    pub wdcn: WDCN,
    _reserved13: [u8; 76usize],
    #[doc = "0x80 - Interrupt Mux Control 0."]
    pub int_mux_ctrl0: INT_MUX_CTRL0,
    #[doc = "0x84 - Interrupt Mux Control 1."]
    pub int_mux_ctrl1: INT_MUX_CTRL1,
    #[doc = "0x88 - Interrupt Mux Control 2."]
    pub int_mux_ctrl2: INT_MUX_CTRL2,
    #[doc = "0x8c - Interrupt Mux Control 3."]
    pub int_mux_ctrl3: INT_MUX_CTRL3,
    #[doc = "0x90 - Configurable starting IP address for Q30E."]
    pub ip_addr: IP_ADDR,
    #[doc = "0x94 - Control Register."]
    pub ctrl: CTRL,
    _reserved19: [u8; 8usize],
    #[doc = "0xa0 - Interrupt Input From CPU Control Register."]
    pub int_in_ctrl: INT_IN_CTRL,
    #[doc = "0xa4 - Interrupt Input From CPU Flag."]
    pub int_in_flag: INT_IN_FLAG,
    #[doc = "0xa8 - Interrupt Input From CPU Enable."]
    pub int_in_ie: INT_IN_IE,
    _reserved22: [u8; 4usize],
    #[doc = "0xb0 - Interrupt Output To CPU Flag."]
    pub irq_flag: IRQ_FLAG,
    #[doc = "0xb4 - Interrupt Output To CPU Control Register."]
    pub irq_ie: IRQ_IE,
}
#[doc = "Q30E Instruction Pointer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip](ip) module"]
pub type IP = crate::Reg<u32, _IP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IP;
#[doc = "`read()` method returns [ip::R](ip::R) reader structure"]
impl crate::Readable for IP {}
#[doc = "Q30E Instruction Pointer."]
pub mod ip;
#[doc = "Q30E Stack Pointer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sp](sp) module"]
pub type SP = crate::Reg<u32, _SP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SP;
#[doc = "`read()` method returns [sp::R](sp::R) reader structure"]
impl crate::Readable for SP {}
#[doc = "Q30E Stack Pointer."]
pub mod sp;
#[doc = "Q30E Data Pointer 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dp0](dp0) module"]
pub type DP0 = crate::Reg<u32, _DP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DP0;
#[doc = "`read()` method returns [dp0::R](dp0::R) reader structure"]
impl crate::Readable for DP0 {}
#[doc = "Q30E Data Pointer 0."]
pub mod dp0;
#[doc = "Q30E Data Pointer 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dp1](dp1) module"]
pub type DP1 = crate::Reg<u32, _DP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DP1;
#[doc = "`read()` method returns [dp1::R](dp1::R) reader structure"]
impl crate::Readable for DP1 {}
#[doc = "Q30E Data Pointer 1."]
pub mod dp1;
#[doc = "Q30E Frame Pointer Base.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bp](bp) module"]
pub type BP = crate::Reg<u32, _BP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BP;
#[doc = "`read()` method returns [bp::R](bp::R) reader structure"]
impl crate::Readable for BP {}
#[doc = "Q30E Frame Pointer Base."]
pub mod bp;
#[doc = "Q30E Frame Pointer Offset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offs](offs) module"]
pub type OFFS = crate::Reg<u32, _OFFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFS;
#[doc = "`read()` method returns [offs::R](offs::R) reader structure"]
impl crate::Readable for OFFS {}
#[doc = "Q30E Frame Pointer Offset."]
pub mod offs;
#[doc = "Q30E Loop Counter 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc0](lc0) module"]
pub type LC0 = crate::Reg<u32, _LC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC0;
#[doc = "`read()` method returns [lc0::R](lc0::R) reader structure"]
impl crate::Readable for LC0 {}
#[doc = "Q30E Loop Counter 0."]
pub mod lc0;
#[doc = "Q30E Loop Counter 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc1](lc1) module"]
pub type LC1 = crate::Reg<u32, _LC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LC1;
#[doc = "`read()` method returns [lc1::R](lc1::R) reader structure"]
impl crate::Readable for LC1 {}
#[doc = "Q30E Loop Counter 1."]
pub mod lc1;
#[doc = "Q30E Accumulator 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0](a0) module"]
pub type A0 = crate::Reg<u32, _A0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A0;
#[doc = "`read()` method returns [a0::R](a0::R) reader structure"]
impl crate::Readable for A0 {}
#[doc = "Q30E Accumulator 0."]
pub mod a0;
#[doc = "Q30E Accumulator 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1](a1) module"]
pub type A1 = crate::Reg<u32, _A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A1;
#[doc = "`read()` method returns [a1::R](a1::R) reader structure"]
impl crate::Readable for A1 {}
#[doc = "Q30E Accumulator 1."]
pub mod a1;
#[doc = "Q30E Accumulator 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a2](a2) module"]
pub type A2 = crate::Reg<u32, _A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A2;
#[doc = "`read()` method returns [a2::R](a2::R) reader structure"]
impl crate::Readable for A2 {}
#[doc = "Q30E Accumulator 2."]
pub mod a2;
#[doc = "Q30E Accumulator 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a3](a3) module"]
pub type A3 = crate::Reg<u32, _A3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A3;
#[doc = "`read()` method returns [a3::R](a3::R) reader structure"]
impl crate::Readable for A3 {}
#[doc = "Q30E Accumulator 3."]
pub mod a3;
#[doc = "Q30E Watchdog Control.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdcn](wdcn) module"]
pub type WDCN = crate::Reg<u32, _WDCN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDCN;
#[doc = "`read()` method returns [wdcn::R](wdcn::R) reader structure"]
impl crate::Readable for WDCN {}
#[doc = "Q30E Watchdog Control."]
pub mod wdcn;
#[doc = "Interrupt Mux Control 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mux_ctrl0](int_mux_ctrl0) module"]
pub type INT_MUX_CTRL0 = crate::Reg<u32, _INT_MUX_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MUX_CTRL0;
#[doc = "`read()` method returns [int_mux_ctrl0::R](int_mux_ctrl0::R) reader structure"]
impl crate::Readable for INT_MUX_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [int_mux_ctrl0::W](int_mux_ctrl0::W) writer structure"]
impl crate::Writable for INT_MUX_CTRL0 {}
#[doc = "Interrupt Mux Control 0."]
pub mod int_mux_ctrl0;
#[doc = "Interrupt Mux Control 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mux_ctrl1](int_mux_ctrl1) module"]
pub type INT_MUX_CTRL1 = crate::Reg<u32, _INT_MUX_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MUX_CTRL1;
#[doc = "`read()` method returns [int_mux_ctrl1::R](int_mux_ctrl1::R) reader structure"]
impl crate::Readable for INT_MUX_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [int_mux_ctrl1::W](int_mux_ctrl1::W) writer structure"]
impl crate::Writable for INT_MUX_CTRL1 {}
#[doc = "Interrupt Mux Control 1."]
pub mod int_mux_ctrl1;
#[doc = "Interrupt Mux Control 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mux_ctrl2](int_mux_ctrl2) module"]
pub type INT_MUX_CTRL2 = crate::Reg<u32, _INT_MUX_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MUX_CTRL2;
#[doc = "`read()` method returns [int_mux_ctrl2::R](int_mux_ctrl2::R) reader structure"]
impl crate::Readable for INT_MUX_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [int_mux_ctrl2::W](int_mux_ctrl2::W) writer structure"]
impl crate::Writable for INT_MUX_CTRL2 {}
#[doc = "Interrupt Mux Control 2."]
pub mod int_mux_ctrl2;
#[doc = "Interrupt Mux Control 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mux_ctrl3](int_mux_ctrl3) module"]
pub type INT_MUX_CTRL3 = crate::Reg<u32, _INT_MUX_CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MUX_CTRL3;
#[doc = "`read()` method returns [int_mux_ctrl3::R](int_mux_ctrl3::R) reader structure"]
impl crate::Readable for INT_MUX_CTRL3 {}
#[doc = "`write(|w| ..)` method takes [int_mux_ctrl3::W](int_mux_ctrl3::W) writer structure"]
impl crate::Writable for INT_MUX_CTRL3 {}
#[doc = "Interrupt Mux Control 3."]
pub mod int_mux_ctrl3;
#[doc = "Configurable starting IP address for Q30E.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip_addr](ip_addr) module"]
pub type IP_ADDR = crate::Reg<u32, _IP_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IP_ADDR;
#[doc = "`read()` method returns [ip_addr::R](ip_addr::R) reader structure"]
impl crate::Readable for IP_ADDR {}
#[doc = "`write(|w| ..)` method takes [ip_addr::W](ip_addr::W) writer structure"]
impl crate::Writable for IP_ADDR {}
#[doc = "Configurable starting IP address for Q30E."]
pub mod ip_addr;
#[doc = "Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register."]
pub mod ctrl;
#[doc = "Interrupt Input From CPU Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_in_ctrl](int_in_ctrl) module"]
pub type INT_IN_CTRL = crate::Reg<u32, _INT_IN_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_IN_CTRL;
#[doc = "`read()` method returns [int_in_ctrl::R](int_in_ctrl::R) reader structure"]
impl crate::Readable for INT_IN_CTRL {}
#[doc = "`write(|w| ..)` method takes [int_in_ctrl::W](int_in_ctrl::W) writer structure"]
impl crate::Writable for INT_IN_CTRL {}
#[doc = "Interrupt Input From CPU Control Register."]
pub mod int_in_ctrl;
#[doc = "Interrupt Input From CPU Flag.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_in_flag](int_in_flag) module"]
pub type INT_IN_FLAG = crate::Reg<u32, _INT_IN_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_IN_FLAG;
#[doc = "`read()` method returns [int_in_flag::R](int_in_flag::R) reader structure"]
impl crate::Readable for INT_IN_FLAG {}
#[doc = "`write(|w| ..)` method takes [int_in_flag::W](int_in_flag::W) writer structure"]
impl crate::Writable for INT_IN_FLAG {}
#[doc = "Interrupt Input From CPU Flag."]
pub mod int_in_flag;
#[doc = "Interrupt Input From CPU Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_in_ie](int_in_ie) module"]
pub type INT_IN_IE = crate::Reg<u32, _INT_IN_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_IN_IE;
#[doc = "`read()` method returns [int_in_ie::R](int_in_ie::R) reader structure"]
impl crate::Readable for INT_IN_IE {}
#[doc = "`write(|w| ..)` method takes [int_in_ie::W](int_in_ie::W) writer structure"]
impl crate::Writable for INT_IN_IE {}
#[doc = "Interrupt Input From CPU Enable."]
pub mod int_in_ie;
#[doc = "Interrupt Output To CPU Flag.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_flag](irq_flag) module"]
pub type IRQ_FLAG = crate::Reg<u32, _IRQ_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_FLAG;
#[doc = "`read()` method returns [irq_flag::R](irq_flag::R) reader structure"]
impl crate::Readable for IRQ_FLAG {}
#[doc = "`write(|w| ..)` method takes [irq_flag::W](irq_flag::W) writer structure"]
impl crate::Writable for IRQ_FLAG {}
#[doc = "Interrupt Output To CPU Flag."]
pub mod irq_flag;
#[doc = "Interrupt Output To CPU Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_ie](irq_ie) module"]
pub type IRQ_IE = crate::Reg<u32, _IRQ_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_IE;
#[doc = "`read()` method returns [irq_ie::R](irq_ie::R) reader structure"]
impl crate::Readable for IRQ_IE {}
#[doc = "`write(|w| ..)` method takes [irq_ie::W](irq_ie::W) writer structure"]
impl crate::Writable for IRQ_IE {}
#[doc = "Interrupt Output To CPU Control Register."]
pub mod irq_ie;
