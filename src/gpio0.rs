#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
    pub en: EN,
    #[doc = "0x04 - GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
    pub en_set: EN_SET,
    #[doc = "0x08 - GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
    pub en_clr: EN_CLR,
    #[doc = "0x0c - GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
    pub out_en: OUT_EN,
    #[doc = "0x10 - GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
    pub out_en_set: OUT_EN_SET,
    #[doc = "0x14 - GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
    pub out_en_clr: OUT_EN_CLR,
    #[doc = "0x18 - GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
    pub out: OUT,
    #[doc = "0x1c - GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
    pub out_set: OUT_SET,
    #[doc = "0x20 - GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
    pub out_clr: OUT_CLR,
    #[doc = "0x24 - GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
    pub in_: IN,
    #[doc = "0x28 - GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
    pub int_mod: INT_MOD,
    #[doc = "0x2c - GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
    pub int_pol: INT_POL,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
    pub int_en: INT_EN,
    #[doc = "0x38 - GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
    pub int_en_set: INT_EN_SET,
    #[doc = "0x3c - GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
    pub int_en_clr: INT_EN_CLR,
    #[doc = "0x40 - GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
    pub int_stat: INT_STAT,
    _reserved16: [u8; 4usize],
    #[doc = "0x48 - GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
    pub int_clr: INT_CLR,
    #[doc = "0x4c - GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
    pub wake_en: WAKE_EN,
    #[doc = "0x50 - GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
    pub wake_en_set: WAKE_EN_SET,
    #[doc = "0x54 - GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
    pub wake_en_clr: WAKE_EN_CLR,
    _reserved20: [u8; 4usize],
    #[doc = "0x5c - GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
    pub int_dual_edge: INT_DUAL_EDGE,
    #[doc = "0x60 - GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    pub pad_cfg1: PAD_CFG1,
    #[doc = "0x64 - GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    pub pad_cfg2: PAD_CFG2,
    #[doc = "0x68 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    pub en1: EN1,
    #[doc = "0x6c - GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
    pub en1_set: EN1_SET,
    #[doc = "0x70 - GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
    pub en1_clr: EN1_CLR,
    #[doc = "0x74 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    pub en2: EN2,
    #[doc = "0x78 - GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
    pub en2_set: EN2_SET,
    #[doc = "0x7c - GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
    pub en2_clr: EN2_CLR,
    _reserved29: [u8; 40usize],
    #[doc = "0xa8 - Input Hysteresis Enable Register"]
    pub is: IS,
    #[doc = "0xac - Slew Rate Select Register."]
    pub sr: SR,
    #[doc = "0xb0 - GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    pub ds: DS,
    #[doc = "0xb4 - GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    pub ds1: DS1,
    #[doc = "0xb8 - GPIO Pull Select Mode."]
    pub ps: PS,
    _reserved34: [u8; 4usize],
    #[doc = "0xc0 - GPIO Voltage Select."]
    pub vssel: VSSEL,
}
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
pub mod en;
#[doc = "GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_set](en_set) module"]
pub type EN_SET = crate::Reg<u32, _EN_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_SET;
#[doc = "`read()` method returns [en_set::R](en_set::R) reader structure"]
impl crate::Readable for EN_SET {}
#[doc = "`write(|w| ..)` method takes [en_set::W](en_set::W) writer structure"]
impl crate::Writable for EN_SET {}
#[doc = "GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
pub mod en_set;
#[doc = "GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_clr](en_clr) module"]
pub type EN_CLR = crate::Reg<u32, _EN_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN_CLR;
#[doc = "`read()` method returns [en_clr::R](en_clr::R) reader structure"]
impl crate::Readable for EN_CLR {}
#[doc = "`write(|w| ..)` method takes [en_clr::W](en_clr::W) writer structure"]
impl crate::Writable for EN_CLR {}
#[doc = "GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
pub mod en_clr;
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_en](out_en) module"]
pub type OUT_EN = crate::Reg<u32, _OUT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EN;
#[doc = "`read()` method returns [out_en::R](out_en::R) reader structure"]
impl crate::Readable for OUT_EN {}
#[doc = "`write(|w| ..)` method takes [out_en::W](out_en::W) writer structure"]
impl crate::Writable for OUT_EN {}
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
pub mod out_en;
#[doc = "GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_en_set](out_en_set) module"]
pub type OUT_EN_SET = crate::Reg<u32, _OUT_EN_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EN_SET;
#[doc = "`read()` method returns [out_en_set::R](out_en_set::R) reader structure"]
impl crate::Readable for OUT_EN_SET {}
#[doc = "`write(|w| ..)` method takes [out_en_set::W](out_en_set::W) writer structure"]
impl crate::Writable for OUT_EN_SET {}
#[doc = "GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
pub mod out_en_set;
#[doc = "GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_en_clr](out_en_clr) module"]
pub type OUT_EN_CLR = crate::Reg<u32, _OUT_EN_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_EN_CLR;
#[doc = "`read()` method returns [out_en_clr::R](out_en_clr::R) reader structure"]
impl crate::Readable for OUT_EN_CLR {}
#[doc = "`write(|w| ..)` method takes [out_en_clr::W](out_en_clr::W) writer structure"]
impl crate::Writable for OUT_EN_CLR {}
#[doc = "GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
pub mod out_en_clr;
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
pub mod out;
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_set](out_set) module"]
pub type OUT_SET = crate::Reg<u32, _OUT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_SET;
#[doc = "`write(|w| ..)` method takes [out_set::W](out_set::W) writer structure"]
impl crate::Writable for OUT_SET {}
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
pub mod out_set;
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_clr](out_clr) module"]
pub type OUT_CLR = crate::Reg<u32, _OUT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT_CLR;
#[doc = "`write(|w| ..)` method takes [out_clr::W](out_clr::W) writer structure"]
impl crate::Writable for OUT_CLR {}
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
pub mod out_clr;
#[doc = "GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
pub mod in_;
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mod](int_mod) module"]
pub type INT_MOD = crate::Reg<u32, _INT_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MOD;
#[doc = "`read()` method returns [int_mod::R](int_mod::R) reader structure"]
impl crate::Readable for INT_MOD {}
#[doc = "`write(|w| ..)` method takes [int_mod::W](int_mod::W) writer structure"]
impl crate::Writable for INT_MOD {}
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
pub mod int_mod;
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_pol](int_pol) module"]
pub type INT_POL = crate::Reg<u32, _INT_POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_POL;
#[doc = "`read()` method returns [int_pol::R](int_pol::R) reader structure"]
impl crate::Readable for INT_POL {}
#[doc = "`write(|w| ..)` method takes [int_pol::W](int_pol::W) writer structure"]
impl crate::Writable for INT_POL {}
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
pub mod int_pol;
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u32, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
pub mod int_en;
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en_set](int_en_set) module"]
pub type INT_EN_SET = crate::Reg<u32, _INT_EN_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN_SET;
#[doc = "`read()` method returns [int_en_set::R](int_en_set::R) reader structure"]
impl crate::Readable for INT_EN_SET {}
#[doc = "`write(|w| ..)` method takes [int_en_set::W](int_en_set::W) writer structure"]
impl crate::Writable for INT_EN_SET {}
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
pub mod int_en_set;
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en_clr](int_en_clr) module"]
pub type INT_EN_CLR = crate::Reg<u32, _INT_EN_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN_CLR;
#[doc = "`read()` method returns [int_en_clr::R](int_en_clr::R) reader structure"]
impl crate::Readable for INT_EN_CLR {}
#[doc = "`write(|w| ..)` method takes [int_en_clr::W](int_en_clr::W) writer structure"]
impl crate::Writable for INT_EN_CLR {}
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
pub mod int_en_clr;
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat](int_stat) module"]
pub type INT_STAT = crate::Reg<u32, _INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STAT;
#[doc = "`read()` method returns [int_stat::R](int_stat::R) reader structure"]
impl crate::Readable for INT_STAT {}
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
pub mod int_stat;
#[doc = "GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](int_clr) module"]
pub type INT_CLR = crate::Reg<u32, _INT_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLR;
#[doc = "`read()` method returns [int_clr::R](int_clr::R) reader structure"]
impl crate::Readable for INT_CLR {}
#[doc = "`write(|w| ..)` method takes [int_clr::W](int_clr::W) writer structure"]
impl crate::Writable for INT_CLR {}
#[doc = "GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
pub mod int_clr;
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en](wake_en) module"]
pub type WAKE_EN = crate::Reg<u32, _WAKE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_EN;
#[doc = "`read()` method returns [wake_en::R](wake_en::R) reader structure"]
impl crate::Readable for WAKE_EN {}
#[doc = "`write(|w| ..)` method takes [wake_en::W](wake_en::W) writer structure"]
impl crate::Writable for WAKE_EN {}
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
pub mod wake_en;
#[doc = "GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en_set](wake_en_set) module"]
pub type WAKE_EN_SET = crate::Reg<u32, _WAKE_EN_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_EN_SET;
#[doc = "`read()` method returns [wake_en_set::R](wake_en_set::R) reader structure"]
impl crate::Readable for WAKE_EN_SET {}
#[doc = "`write(|w| ..)` method takes [wake_en_set::W](wake_en_set::W) writer structure"]
impl crate::Writable for WAKE_EN_SET {}
#[doc = "GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
pub mod wake_en_set;
#[doc = "GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_en_clr](wake_en_clr) module"]
pub type WAKE_EN_CLR = crate::Reg<u32, _WAKE_EN_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKE_EN_CLR;
#[doc = "`read()` method returns [wake_en_clr::R](wake_en_clr::R) reader structure"]
impl crate::Readable for WAKE_EN_CLR {}
#[doc = "`write(|w| ..)` method takes [wake_en_clr::W](wake_en_clr::W) writer structure"]
impl crate::Writable for WAKE_EN_CLR {}
#[doc = "GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
pub mod wake_en_clr;
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_dual_edge](int_dual_edge) module"]
pub type INT_DUAL_EDGE = crate::Reg<u32, _INT_DUAL_EDGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_DUAL_EDGE;
#[doc = "`read()` method returns [int_dual_edge::R](int_dual_edge::R) reader structure"]
impl crate::Readable for INT_DUAL_EDGE {}
#[doc = "`write(|w| ..)` method takes [int_dual_edge::W](int_dual_edge::W) writer structure"]
impl crate::Writable for INT_DUAL_EDGE {}
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
pub mod int_dual_edge;
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg1](pad_cfg1) module"]
pub type PAD_CFG1 = crate::Reg<u32, _PAD_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAD_CFG1;
#[doc = "`read()` method returns [pad_cfg1::R](pad_cfg1::R) reader structure"]
impl crate::Readable for PAD_CFG1 {}
#[doc = "`write(|w| ..)` method takes [pad_cfg1::W](pad_cfg1::W) writer structure"]
impl crate::Writable for PAD_CFG1 {}
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod pad_cfg1;
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg2](pad_cfg2) module"]
pub type PAD_CFG2 = crate::Reg<u32, _PAD_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAD_CFG2;
#[doc = "`read()` method returns [pad_cfg2::R](pad_cfg2::R) reader structure"]
impl crate::Readable for PAD_CFG2 {}
#[doc = "`write(|w| ..)` method takes [pad_cfg2::W](pad_cfg2::W) writer structure"]
impl crate::Writable for PAD_CFG2 {}
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod pad_cfg2;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en1](en1) module"]
pub type EN1 = crate::Reg<u32, _EN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN1;
#[doc = "`read()` method returns [en1::R](en1::R) reader structure"]
impl crate::Readable for EN1 {}
#[doc = "`write(|w| ..)` method takes [en1::W](en1::W) writer structure"]
impl crate::Writable for EN1 {}
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en1;
#[doc = "GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en1_set](en1_set) module"]
pub type EN1_SET = crate::Reg<u32, _EN1_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN1_SET;
#[doc = "`read()` method returns [en1_set::R](en1_set::R) reader structure"]
impl crate::Readable for EN1_SET {}
#[doc = "`write(|w| ..)` method takes [en1_set::W](en1_set::W) writer structure"]
impl crate::Writable for EN1_SET {}
#[doc = "GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
pub mod en1_set;
#[doc = "GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en1_clr](en1_clr) module"]
pub type EN1_CLR = crate::Reg<u32, _EN1_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN1_CLR;
#[doc = "`read()` method returns [en1_clr::R](en1_clr::R) reader structure"]
impl crate::Readable for EN1_CLR {}
#[doc = "`write(|w| ..)` method takes [en1_clr::W](en1_clr::W) writer structure"]
impl crate::Writable for EN1_CLR {}
#[doc = "GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
pub mod en1_clr;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en2](en2) module"]
pub type EN2 = crate::Reg<u32, _EN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN2;
#[doc = "`read()` method returns [en2::R](en2::R) reader structure"]
impl crate::Readable for EN2 {}
#[doc = "`write(|w| ..)` method takes [en2::W](en2::W) writer structure"]
impl crate::Writable for EN2 {}
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en2;
#[doc = "GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en2_set](en2_set) module"]
pub type EN2_SET = crate::Reg<u32, _EN2_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN2_SET;
#[doc = "`read()` method returns [en2_set::R](en2_set::R) reader structure"]
impl crate::Readable for EN2_SET {}
#[doc = "`write(|w| ..)` method takes [en2_set::W](en2_set::W) writer structure"]
impl crate::Writable for EN2_SET {}
#[doc = "GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
pub mod en2_set;
#[doc = "GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en2_clr](en2_clr) module"]
pub type EN2_CLR = crate::Reg<u32, _EN2_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN2_CLR;
#[doc = "`read()` method returns [en2_clr::R](en2_clr::R) reader structure"]
impl crate::Readable for EN2_CLR {}
#[doc = "`write(|w| ..)` method takes [en2_clr::W](en2_clr::W) writer structure"]
impl crate::Writable for EN2_CLR {}
#[doc = "GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
pub mod en2_clr;
#[doc = "Input Hysteresis Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](is) module"]
pub type IS = crate::Reg<u32, _IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS;
#[doc = "`read()` method returns [is::R](is::R) reader structure"]
impl crate::Readable for IS {}
#[doc = "`write(|w| ..)` method takes [is::W](is::W) writer structure"]
impl crate::Writable for IS {}
#[doc = "Input Hysteresis Enable Register"]
pub mod is;
#[doc = "Slew Rate Select Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "Slew Rate Select Register."]
pub mod sr;
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds](ds) module"]
pub type DS = crate::Reg<u32, _DS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DS;
#[doc = "`read()` method returns [ds::R](ds::R) reader structure"]
impl crate::Readable for DS {}
#[doc = "`write(|w| ..)` method takes [ds::W](ds::W) writer structure"]
impl crate::Writable for DS {}
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds;
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds1](ds1) module"]
pub type DS1 = crate::Reg<u32, _DS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DS1;
#[doc = "`read()` method returns [ds1::R](ds1::R) reader structure"]
impl crate::Readable for DS1 {}
#[doc = "`write(|w| ..)` method takes [ds1::W](ds1::W) writer structure"]
impl crate::Writable for DS1 {}
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds1;
#[doc = "GPIO Pull Select Mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps](ps) module"]
pub type PS = crate::Reg<u32, _PS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PS;
#[doc = "`read()` method returns [ps::R](ps::R) reader structure"]
impl crate::Readable for PS {}
#[doc = "`write(|w| ..)` method takes [ps::W](ps::W) writer structure"]
impl crate::Writable for PS {}
#[doc = "GPIO Pull Select Mode."]
pub mod ps;
#[doc = "GPIO Voltage Select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vssel](vssel) module"]
pub type VSSEL = crate::Reg<u32, _VSSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VSSEL;
#[doc = "`read()` method returns [vssel::R](vssel::R) reader structure"]
impl crate::Readable for VSSEL {}
#[doc = "`write(|w| ..)` method takes [vssel::W](vssel::W) writer structure"]
impl crate::Writable for VSSEL {}
#[doc = "GPIO Voltage Select."]
pub mod vssel;
