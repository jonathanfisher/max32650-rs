#[doc = "Reader of register ACNTL"]
pub type R = crate::R<u32, super::ACNTL>;
#[doc = "Writer for register ACNTL"]
pub type W = crate::W<u32, super::ACNTL>;
#[doc = "Register ACNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADATA`"]
pub type ADATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADATA`"]
pub struct ADATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - System Info Block Access Data."]
    #[inline(always)]
    pub fn adata(&self) -> ADATA_R {
        ADATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - System Info Block Access Data."]
    #[inline(always)]
    pub fn adata(&mut self) -> ADATA_W {
        ADATA_W { w: self }
    }
}
