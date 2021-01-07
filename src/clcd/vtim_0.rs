#[doc = "Reader of register VTIM_0"]
pub type R = crate::R<u32, super::VTIM_0>;
#[doc = "Writer for register VTIM_0"]
pub type W = crate::W<u32, super::VTIM_0>;
#[doc = "Register VTIM_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::VTIM_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLINES`"]
pub type VLINES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VLINES`"]
pub struct VLINES_W<'a> {
    w: &'a mut W,
}
impl<'a> VLINES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VBACKPORCH`"]
pub type VBACKPORCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBACKPORCH`"]
pub struct VBACKPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBACKPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - V Lines"]
    #[inline(always)]
    pub fn vlines(&self) -> VLINES_R {
        VLINES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - V BACK PORCH"]
    #[inline(always)]
    pub fn vbackporch(&self) -> VBACKPORCH_R {
        VBACKPORCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - V Lines"]
    #[inline(always)]
    pub fn vlines(&mut self) -> VLINES_W {
        VLINES_W { w: self }
    }
    #[doc = "Bits 16:23 - V BACK PORCH"]
    #[inline(always)]
    pub fn vbackporch(&mut self) -> VBACKPORCH_W {
        VBACKPORCH_W { w: self }
    }
}
