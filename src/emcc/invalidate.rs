#[doc = "Reader of register INVALIDATE"]
pub type R = crate::R<u32, super::INVALIDATE>;
#[doc = "Writer for register INVALIDATE"]
pub type W = crate::W<u32, super::INVALIDATE>;
#[doc = "Register INVALIDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::INVALIDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IA`"]
pub type IA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IA`"]
pub struct IA_W<'a> {
    w: &'a mut W,
}
impl<'a> IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Invalidate all cache contents."]
    #[inline(always)]
    pub fn ia(&self) -> IA_R {
        IA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Invalidate all cache contents."]
    #[inline(always)]
    pub fn ia(&mut self) -> IA_W {
        IA_W { w: self }
    }
}
