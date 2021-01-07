#[doc = "Reader of register EPINFO"]
pub type R = crate::R<u8, super::EPINFO>;
#[doc = "Reader of field `OUTENDPOINTS`"]
pub type OUTENDPOINTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `INTENDPOINTS`"]
pub type INTENDPOINTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn outendpoints(&self) -> OUTENDPOINTS_R {
        OUTENDPOINTS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn intendpoints(&self) -> INTENDPOINTS_R {
        INTENDPOINTS_R::new((self.bits & 0x0f) as u8)
    }
}
