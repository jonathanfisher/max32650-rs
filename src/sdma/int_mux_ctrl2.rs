#[doc = "Reader of register INT_MUX_CTRL2"]
pub type R = crate::R<u32, super::INT_MUX_CTRL2>;
#[doc = "Writer for register INT_MUX_CTRL2"]
pub type W = crate::W<u32, super::INT_MUX_CTRL2>;
#[doc = "Register INT_MUX_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_MUX_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTSEL24`"]
pub type INTSEL24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL24`"]
pub struct INTSEL24_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INTSEL25`"]
pub type INTSEL25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL25`"]
pub struct INTSEL25_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INTSEL26`"]
pub type INTSEL26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL26`"]
pub struct INTSEL26_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INTSEL27`"]
pub type INTSEL27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL27`"]
pub struct INTSEL27_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Selection For 24th Interrupt."]
    #[inline(always)]
    pub fn intsel24(&self) -> INTSEL24_R {
        INTSEL24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 25th Interrupt."]
    #[inline(always)]
    pub fn intsel25(&self) -> INTSEL25_R {
        INTSEL25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 26th Interrupt."]
    #[inline(always)]
    pub fn intsel26(&self) -> INTSEL26_R {
        INTSEL26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 27th Interrupt."]
    #[inline(always)]
    pub fn intsel27(&self) -> INTSEL27_R {
        INTSEL27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Selection For 24th Interrupt."]
    #[inline(always)]
    pub fn intsel24(&mut self) -> INTSEL24_W {
        INTSEL24_W { w: self }
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 25th Interrupt."]
    #[inline(always)]
    pub fn intsel25(&mut self) -> INTSEL25_W {
        INTSEL25_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 26th Interrupt."]
    #[inline(always)]
    pub fn intsel26(&mut self) -> INTSEL26_W {
        INTSEL26_W { w: self }
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 27th Interrupt."]
    #[inline(always)]
    pub fn intsel27(&mut self) -> INTSEL27_W {
        INTSEL27_W { w: self }
    }
}
