#[doc = "Reader of register PLL0CN"]
pub type R = crate::R<u32, super::PLL0CN>;
#[doc = "Writer for register PLL0CN"]
pub type W = crate::W<u32, super::PLL0CN>;
#[doc = "Register PLL0CN `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL0CN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFU`"]
pub type RFU_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RFU`"]
pub struct RFU_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rfu(&self) -> RFU_R {
        RFU_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Future Use"]
    #[inline(always)]
    pub fn rfu(&mut self) -> RFU_W {
        RFU_W { w: self }
    }
}
