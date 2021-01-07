#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Read to test and set, returns prior value. Write 0 to clear semaphore."]
    pub semaphores: [SEMAPHORES; 8],
    _reserved1: [u8; 224usize],
    #[doc = "0x100 - Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
    pub status: STATUS,
}
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semaphores](semaphores) module"]
pub type SEMAPHORES = crate::Reg<u32, _SEMAPHORES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMAPHORES;
#[doc = "`read()` method returns [semaphores::R](semaphores::R) reader structure"]
impl crate::Readable for SEMAPHORES {}
#[doc = "`write(|w| ..)` method takes [semaphores::W](semaphores::W) writer structure"]
impl crate::Writable for SEMAPHORES {}
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore."]
pub mod semaphores;
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
pub mod status;
