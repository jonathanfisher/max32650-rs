#[doc = "Reader of register WAKE_EN_CLR"]
pub type R = crate::R<u32, super::WAKE_EN_CLR>;
#[doc = "Writer for register WAKE_EN_CLR"]
pub type W = crate::W<u32, super::WAKE_EN_CLR>;
#[doc = "Register WAKE_EN_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKE_EN_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALL`"]
pub type ALL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ALL`"]
pub struct ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&mut self) -> ALL_W {
        ALL_W { w: self }
    }
}
