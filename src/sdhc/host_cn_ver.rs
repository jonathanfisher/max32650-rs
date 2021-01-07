#[doc = "Reader of register HOST_CN_VER"]
pub type R = crate::R<u16, super::HOST_CN_VER>;
#[doc = "Writer for register HOST_CN_VER"]
pub type W = crate::W<u16, super::HOST_CN_VER>;
#[doc = "Register HOST_CN_VER `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_CN_VER {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPEC_VER`"]
pub type SPEC_VER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPEC_VER`"]
pub struct SPEC_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEC_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VEND_VER`"]
pub type VEND_VER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEND_VER`"]
pub struct VEND_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> VEND_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specification Version Number."]
    #[inline(always)]
    pub fn spec_ver(&self) -> SPEC_VER_R {
        SPEC_VER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version Number."]
    #[inline(always)]
    pub fn vend_ver(&self) -> VEND_VER_R {
        VEND_VER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specification Version Number."]
    #[inline(always)]
    pub fn spec_ver(&mut self) -> SPEC_VER_W {
        SPEC_VER_W { w: self }
    }
    #[doc = "Bits 8:15 - Vendor Version Number."]
    #[inline(always)]
    pub fn vend_ver(&mut self) -> VEND_VER_W {
        VEND_VER_W { w: self }
    }
}
