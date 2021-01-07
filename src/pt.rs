#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pulse Train Configuration"]
    pub rate_length: RATE_LENGTH,
    #[doc = "0x04 - Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
    pub train: TRAIN,
    #[doc = "0x08 - Pulse Train Loop Count"]
    pub loop_: LOOP,
    #[doc = "0x0c - Pulse Train Auto-Restart Configuration."]
    pub restart: RESTART,
}
#[doc = "Pulse Train Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rate_length](rate_length) module"]
pub type RATE_LENGTH = crate::Reg<u32, _RATE_LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATE_LENGTH;
#[doc = "`read()` method returns [rate_length::R](rate_length::R) reader structure"]
impl crate::Readable for RATE_LENGTH {}
#[doc = "`write(|w| ..)` method takes [rate_length::W](rate_length::W) writer structure"]
impl crate::Writable for RATE_LENGTH {}
#[doc = "Pulse Train Configuration"]
pub mod rate_length;
#[doc = "Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [train](train) module"]
pub type TRAIN = crate::Reg<u32, _TRAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRAIN;
#[doc = "`read()` method returns [train::R](train::R) reader structure"]
impl crate::Readable for TRAIN {}
#[doc = "`write(|w| ..)` method takes [train::W](train::W) writer structure"]
impl crate::Writable for TRAIN {}
#[doc = "Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
pub mod train;
#[doc = "Pulse Train Loop Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop_](loop_) module"]
pub type LOOP = crate::Reg<u32, _LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOOP;
#[doc = "`read()` method returns [loop_::R](loop_::R) reader structure"]
impl crate::Readable for LOOP {}
#[doc = "`write(|w| ..)` method takes [loop_::W](loop_::W) writer structure"]
impl crate::Writable for LOOP {}
#[doc = "Pulse Train Loop Count"]
pub mod loop_;
#[doc = "Pulse Train Auto-Restart Configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [restart](restart) module"]
pub type RESTART = crate::Reg<u32, _RESTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESTART;
#[doc = "`read()` method returns [restart::R](restart::R) reader structure"]
impl crate::Readable for RESTART {}
#[doc = "`write(|w| ..)` method takes [restart::W](restart::W) writer structure"]
impl crate::Writable for RESTART {}
#[doc = "Pulse Train Auto-Restart Configuration."]
pub mod restart;
