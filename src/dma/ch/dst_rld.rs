#[doc = "Reader of register DST_RLD"]
pub type R = crate::R<u32, super::DST_RLD>;
#[doc = "Writer for register DST_RLD"]
pub type W = crate::W<u32, super::DST_RLD>;
#[doc = "Register DST_RLD `reset()`'s with value 0"]
impl crate::ResetValue for super::DST_RLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DST_RLD`"]
pub type DST_RLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DST_RLD`"]
pub struct DST_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn dst_rld(&self) -> DST_RLD_R {
        DST_RLD_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn dst_rld(&mut self) -> DST_RLD_W {
        DST_RLD_W { w: self }
    }
}
