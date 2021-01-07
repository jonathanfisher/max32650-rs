#[doc = "Reader of register VTIM_1"]
pub type R = crate::R<u32, super::VTIM_1>;
#[doc = "Writer for register VTIM_1"]
pub type W = crate::W<u32, super::VTIM_1>;
#[doc = "Register VTIM_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::VTIM_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSYNCWIDTH`"]
pub type VSYNCWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSYNCWIDTH`"]
pub struct VSYNCWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNCWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VFRONTPORCH`"]
pub type VFRONTPORCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VFRONTPORCH`"]
pub struct VFRONTPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VFRONTPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - V Sync Width"]
    #[inline(always)]
    pub fn vsyncwidth(&self) -> VSYNCWIDTH_R {
        VSYNCWIDTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - V Front PORCH"]
    #[inline(always)]
    pub fn vfrontporch(&self) -> VFRONTPORCH_R {
        VFRONTPORCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - V Sync Width"]
    #[inline(always)]
    pub fn vsyncwidth(&mut self) -> VSYNCWIDTH_W {
        VSYNCWIDTH_W { w: self }
    }
    #[doc = "Bits 16:23 - V Front PORCH"]
    #[inline(always)]
    pub fn vfrontporch(&mut self) -> VFRONTPORCH_W {
        VFRONTPORCH_W { w: self }
    }
}
