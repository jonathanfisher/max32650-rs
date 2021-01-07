#[doc = "Reader of register INT_MUX_CTRL1"]
pub type R = crate::R<u32, super::INT_MUX_CTRL1>;
#[doc = "Writer for register INT_MUX_CTRL1"]
pub type W = crate::W<u32, super::INT_MUX_CTRL1>;
#[doc = "Register INT_MUX_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_MUX_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTSEL20`"]
pub type INTSEL20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL20`"]
pub struct INTSEL20_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INTSEL21`"]
pub type INTSEL21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL21`"]
pub struct INTSEL21_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INTSEL22`"]
pub type INTSEL22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL22`"]
pub struct INTSEL22_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INTSEL23`"]
pub type INTSEL23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTSEL23`"]
pub struct INTSEL23_W<'a> {
    w: &'a mut W,
}
impl<'a> INTSEL23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt Selection For 20th Interrupt."]
    #[inline(always)]
    pub fn intsel20(&self) -> INTSEL20_R {
        INTSEL20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 21st Interrupt."]
    #[inline(always)]
    pub fn intsel21(&self) -> INTSEL21_R {
        INTSEL21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 22nd Interrupt."]
    #[inline(always)]
    pub fn intsel22(&self) -> INTSEL22_R {
        INTSEL22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 23rd Interrupt."]
    #[inline(always)]
    pub fn intsel23(&self) -> INTSEL23_R {
        INTSEL23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Selection For 20th Interrupt."]
    #[inline(always)]
    pub fn intsel20(&mut self) -> INTSEL20_W {
        INTSEL20_W { w: self }
    }
    #[doc = "Bits 8:15 - Interrupt Selection For 21st Interrupt."]
    #[inline(always)]
    pub fn intsel21(&mut self) -> INTSEL21_W {
        INTSEL21_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Selection For 22nd Interrupt."]
    #[inline(always)]
    pub fn intsel22(&mut self) -> INTSEL22_W {
        INTSEL22_W { w: self }
    }
    #[doc = "Bits 24:31 - Interrupt Selection For 23rd Interrupt."]
    #[inline(always)]
    pub fn intsel23(&mut self) -> INTSEL23_W {
        INTSEL23_W { w: self }
    }
}
