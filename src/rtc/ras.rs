#[doc = "Reader of register RAS"]
pub type R = crate::R<u32, super::RAS>;
#[doc = "Writer for register RAS"]
pub type W = crate::W<u32, super::RAS>;
#[doc = "Register RAS `reset()`'s with value 0"]
impl crate::ResetValue for super::RAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAS`"]
pub type RAS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RAS`"]
pub struct RAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Time-of-day Alarm."]
    #[inline(always)]
    pub fn ras(&mut self) -> RAS_W {
        RAS_W { w: self }
    }
}
