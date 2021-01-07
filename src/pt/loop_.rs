#[doc = "Reader of register LOOP"]
pub type R = crate::R<u32, super::LOOP>;
#[doc = "Writer for register LOOP"]
pub type W = crate::W<u32, super::LOOP>;
#[doc = "Register LOOP `reset()`'s with value 0"]
impl crate::ResetValue for super::LOOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `count`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `count`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `delay`"]
pub type DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `delay`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
}
