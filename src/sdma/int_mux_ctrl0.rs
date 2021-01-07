#[doc = "Reader of register INT_MUX_CTRL0"]
pub type R = crate::R<u32, super::INT_MUX_CTRL0>;
#[doc = "Writer for register INT_MUX_CTRL0"]
pub type W = crate::W<u32, super::INT_MUX_CTRL0>;
#[doc = "Register INT_MUX_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_MUX_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTSEL16`"]
pub type INTSEL16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL16`"]
pub struct INTSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INTSEL17`"]
pub type INTSEL17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL17`"]
pub struct INTSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INTSEL18`"]
pub type INTSEL18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL18`"]
pub struct INTSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INTSEL19`"]
pub type INTSEL19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL19`"]
pub struct INTSEL19_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Selection For 16th Interrupt."]
    #[inline(always)]
    pub fn intsel16(&self) -> INTSEL16_R {
        INTSEL16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 17th Interrupt."]
    #[inline(always)]
    pub fn intsel17(&self) -> INTSEL17_R {
        INTSEL17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 18th Interrupt."]
    #[inline(always)]
    pub fn intsel18(&self) -> INTSEL18_R {
        INTSEL18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 19th Interrupt."]
    #[inline(always)]
    pub fn intsel19(&self) -> INTSEL19_R {
        INTSEL19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Selection For 16th Interrupt."]
    #[inline(always)]
    pub fn intsel16(&mut self) -> INTSEL16_W {
        INTSEL16_W { w: self }
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 17th Interrupt."]
    #[inline(always)]
    pub fn intsel17(&mut self) -> INTSEL17_W {
        INTSEL17_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 18th Interrupt."]
    #[inline(always)]
    pub fn intsel18(&mut self) -> INTSEL18_W {
        INTSEL18_W { w: self }
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 19th Interrupt."]
    #[inline(always)]
    pub fn intsel19(&mut self) -> INTSEL19_W {
        INTSEL19_W { w: self }
    }
}
