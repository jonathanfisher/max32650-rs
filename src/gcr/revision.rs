#[doc = "Reader of register REVISION"]
pub type R = crate::R<u32, super::REVISION>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Manufacturer Chip Revision."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xffff) as u16)
    }
}
