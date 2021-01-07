#[doc = "Reader of register INTFL"]
pub type R = crate::R<u32, super::INTFL>;
#[doc = "Reader of field `ERRINTE`"]
pub type ERRINTE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Error Interrupt Status."]
    #[inline(always)]
    pub fn errinte(&self) -> ERRINTE_R {
        ERRINTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
