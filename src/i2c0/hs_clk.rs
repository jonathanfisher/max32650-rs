#[doc = "Reader of register HS_CLK"]
pub type R = crate::R<u32, super::HS_CLK>;
#[doc = "Writer for register HS_CLK"]
pub type W = crate::W<u32, super::HS_CLK>;
#[doc = "Register HS_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::HS_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HS_CLK_LO`"]
pub type HS_CLK_LO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HS_CLK_LO`"]
pub struct HS_CLK_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_CLK_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `HS_CLK_HI`"]
pub type HS_CLK_HI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HS_CLK_HI`"]
pub struct HS_CLK_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_CLK_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_lo(&self) -> HS_CLK_LO_R {
        HS_CLK_LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_hi(&self) -> HS_CLK_HI_R {
        HS_CLK_HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_lo(&mut self) -> HS_CLK_LO_W {
        HS_CLK_LO_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Address."]
    #[inline(always)]
    pub fn hs_clk_hi(&mut self) -> HS_CLK_HI_W {
        HS_CLK_HI_W { w: self }
    }
}
