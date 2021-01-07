#[doc = "Reader of register ERRADDR"]
pub type R = crate::R<u32, super::ERRADDR>;
#[doc = "Reader of field `ERRADDR`"]
pub type ERRADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn erraddr(&self) -> ERRADDR_R {
        ERRADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
