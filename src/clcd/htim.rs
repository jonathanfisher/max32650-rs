#[doc = "Reader of register HTIM"]
pub type R = crate::R<u32, super::HTIM>;
#[doc = "Writer for register HTIM"]
pub type W = crate::W<u32, super::HTIM>;
#[doc = "Register HTIM `reset()`'s with value 0"]
impl crate::ResetValue for super::HTIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSYNCWIDTH`"]
pub type HSYNCWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSYNCWIDTH`"]
pub struct HSYNCWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNCWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `HFRONTPORCH`"]
pub type HFRONTPORCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFRONTPORCH`"]
pub struct HFRONTPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRONTPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSIZE`"]
pub type HSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSIZE`"]
pub struct HSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HBACKPORCH`"]
pub type HBACKPORCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HBACKPORCH`"]
pub struct HBACKPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> HBACKPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Horizontal Sync Width in CLCD Clocks from 1 to 256 HSync Width = HSYNCWIDTH+1 Clocks"]
    #[inline(always)]
    pub fn hsyncwidth(&self) -> HSYNCWIDTH_R {
        HSYNCWIDTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch size in lines from 1 to 256"]
    #[inline(always)]
    pub fn hfrontporch(&self) -> HFRONTPORCH_R {
        HFRONTPORCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal Front Porch Size in Pixels = (HSIZE + 1)*16"]
    #[inline(always)]
    pub fn hsize(&self) -> HSIZE_R {
        HSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal Back Porch size in CLCD Clocks from 1 to 256 -> HBP=(HBACKPORCH+1)"]
    #[inline(always)]
    pub fn hbackporch(&self) -> HBACKPORCH_R {
        HBACKPORCH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal Sync Width in CLCD Clocks from 1 to 256 HSync Width = HSYNCWIDTH+1 Clocks"]
    #[inline(always)]
    pub fn hsyncwidth(&mut self) -> HSYNCWIDTH_W {
        HSYNCWIDTH_W { w: self }
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch size in lines from 1 to 256"]
    #[inline(always)]
    pub fn hfrontporch(&mut self) -> HFRONTPORCH_W {
        HFRONTPORCH_W { w: self }
    }
    #[doc = "Bits 16:23 - Horizontal Front Porch Size in Pixels = (HSIZE + 1)*16"]
    #[inline(always)]
    pub fn hsize(&mut self) -> HSIZE_W {
        HSIZE_W { w: self }
    }
    #[doc = "Bits 24:31 - Horizontal Back Porch size in CLCD Clocks from 1 to 256 -> HBP=(HBACKPORCH+1)"]
    #[inline(always)]
    pub fn hbackporch(&mut self) -> HBACKPORCH_W {
        HBACKPORCH_W { w: self }
    }
}
