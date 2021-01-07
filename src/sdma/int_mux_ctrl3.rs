#[doc = "Reader of register INT_MUX_CTRL3"]
pub type R = crate::R<u32, super::INT_MUX_CTRL3>;
#[doc = "Writer for register INT_MUX_CTRL3"]
pub type W = crate::W<u32, super::INT_MUX_CTRL3>;
#[doc = "Register INT_MUX_CTRL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_MUX_CTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTSEL28`"]
pub type INTSEL28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL28`"]
pub struct INTSEL28_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INTSEL29`"]
pub type INTSEL29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL29`"]
pub struct INTSEL29_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INTSEL30`"]
pub type INTSEL30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL30`"]
pub struct INTSEL30_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INTSEL31`"]
pub type INTSEL31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL31`"]
pub struct INTSEL31_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Selection For 28th Interrupt."]
    #[inline(always)]
    pub fn intsel28(&self) -> INTSEL28_R {
        INTSEL28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 29th Interrupt."]
    #[inline(always)]
    pub fn intsel29(&self) -> INTSEL29_R {
        INTSEL29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 30th Interrupt."]
    #[inline(always)]
    pub fn intsel30(&self) -> INTSEL30_R {
        INTSEL30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 31st Interrupt."]
    #[inline(always)]
    pub fn intsel31(&self) -> INTSEL31_R {
        INTSEL31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Selection For 28th Interrupt."]
    #[inline(always)]
    pub fn intsel28(&mut self) -> INTSEL28_W {
        INTSEL28_W { w: self }
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 29th Interrupt."]
    #[inline(always)]
    pub fn intsel29(&mut self) -> INTSEL29_W {
        INTSEL29_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 30th Interrupt."]
    #[inline(always)]
    pub fn intsel30(&mut self) -> INTSEL30_W {
        INTSEL30_W { w: self }
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 31st Interrupt."]
    #[inline(always)]
    pub fn intsel31(&mut self) -> INTSEL31_W {
        INTSEL31_W { w: self }
    }
}
