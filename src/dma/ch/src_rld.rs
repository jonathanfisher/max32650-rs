#[doc = "Reader of register SRC_RLD"]
pub type R = crate::R<u32, super::SRC_RLD>;
#[doc = "Writer for register SRC_RLD"]
pub type W = crate::W<u32, super::SRC_RLD>;
#[doc = "Register SRC_RLD `reset()`'s with value 0"]
impl crate::ResetValue for super::SRC_RLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRC_RLD`"]
pub type SRC_RLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SRC_RLD`"]
pub struct SRC_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn src_rld(&self) -> SRC_RLD_R {
        SRC_RLD_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn src_rld(&mut self) -> SRC_RLD_W {
        SRC_RLD_W { w: self }
    }
}
