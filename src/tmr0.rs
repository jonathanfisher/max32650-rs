#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Count. This register stores the current timer count."]
    pub cnt: CNT,
    #[doc = "0x04 - Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
    pub cmp: CMP,
    #[doc = "0x08 - PWM. This register stores the value that is compared to the current timer count."]
    pub pwm: PWM,
    #[doc = "0x0c - Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
    pub intr: INTR,
    #[doc = "0x10 - Timer Control Register."]
    pub cn: CN,
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    pub nolcmp: NOLCMP,
}
#[doc = "Count. This register stores the current timer count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Count. This register stores the current timer count."]
pub mod cnt;
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp](cmp) module"]
pub type CMP = crate::Reg<u32, _CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP;
#[doc = "`read()` method returns [cmp::R](cmp::R) reader structure"]
impl crate::Readable for CMP {}
#[doc = "`write(|w| ..)` method takes [cmp::W](cmp::W) writer structure"]
impl crate::Writable for CMP {}
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
pub mod cmp;
#[doc = "PWM. This register stores the value that is compared to the current timer count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm](pwm) module"]
pub type PWM = crate::Reg<u32, _PWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM;
#[doc = "`read()` method returns [pwm::R](pwm::R) reader structure"]
impl crate::Readable for PWM {}
#[doc = "`write(|w| ..)` method takes [pwm::W](pwm::W) writer structure"]
impl crate::Writable for PWM {}
#[doc = "PWM. This register stores the value that is compared to the current timer count."]
pub mod pwm;
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](intr) module"]
pub type INTR = crate::Reg<u32, _INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR;
#[doc = "`read()` method returns [intr::R](intr::R) reader structure"]
impl crate::Readable for INTR {}
#[doc = "`write(|w| ..)` method takes [intr::W](intr::W) writer structure"]
impl crate::Writable for INTR {}
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
pub mod intr;
#[doc = "Timer Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cn](cn) module"]
pub type CN = crate::Reg<u32, _CN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CN;
#[doc = "`read()` method returns [cn::R](cn::R) reader structure"]
impl crate::Readable for CN {}
#[doc = "`write(|w| ..)` method takes [cn::W](cn::W) writer structure"]
impl crate::Writable for CN {}
#[doc = "Timer Control Register."]
pub mod cn;
#[doc = "Timer Non-Overlapping Compare Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nolcmp](nolcmp) module"]
pub type NOLCMP = crate::Reg<u32, _NOLCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOLCMP;
#[doc = "`read()` method returns [nolcmp::R](nolcmp::R) reader structure"]
impl crate::Readable for NOLCMP {}
#[doc = "`write(|w| ..)` method takes [nolcmp::W](nolcmp::W) writer structure"]
impl crate::Writable for NOLCMP {}
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
