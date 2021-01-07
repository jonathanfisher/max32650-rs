#[doc = "Reader of register CLK_CN"]
pub type R = crate::R<u16, super::CLK_CN>;
#[doc = "Writer for register CLK_CN"]
pub type W = crate::W<u16, super::CLK_CN>;
#[doc = "Register CLK_CN `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_CN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERNAL_CLK_EN`"]
pub type INTERNAL_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERNAL_CLK_EN`"]
pub struct INTERNAL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL_CLK_STABLE`"]
pub type INTERNAL_CLK_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SD_CLK_EN`"]
pub type SD_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SD_CLK_EN`"]
pub struct SD_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLK_GEN_SEL`"]
pub type CLK_GEN_SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `UPPER_SDCLK_FREQ_SEL`"]
pub type UPPER_SDCLK_FREQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPPER_SDCLK_FREQ_SEL`"]
pub struct UPPER_SDCLK_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_SDCLK_FREQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SDCLK_FREQ_SEL`"]
pub type SDCLK_FREQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDCLK_FREQ_SEL`"]
pub struct SDCLK_FREQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLK_FREQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Internal Clock Enable."]
    #[inline(always)]
    pub fn internal_clk_en(&self) -> INTERNAL_CLK_EN_R {
        INTERNAL_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable."]
    #[inline(always)]
    pub fn internal_clk_stable(&self) -> INTERNAL_CLK_STABLE_R {
        INTERNAL_CLK_STABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable."]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SD_CLK_EN_R {
        SD_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select."]
    #[inline(always)]
    pub fn clk_gen_sel(&self) -> CLK_GEN_SEL_R {
        CLK_GEN_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select."]
    #[inline(always)]
    pub fn upper_sdclk_freq_sel(&self) -> UPPER_SDCLK_FREQ_SEL_R {
        UPPER_SDCLK_FREQ_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select."]
    #[inline(always)]
    pub fn sdclk_freq_sel(&self) -> SDCLK_FREQ_SEL_R {
        SDCLK_FREQ_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable."]
    #[inline(always)]
    pub fn internal_clk_en(&mut self) -> INTERNAL_CLK_EN_W {
        INTERNAL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - SD Clock Enable."]
    #[inline(always)]
    pub fn sd_clk_en(&mut self) -> SD_CLK_EN_W {
        SD_CLK_EN_W { w: self }
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select."]
    #[inline(always)]
    pub fn upper_sdclk_freq_sel(&mut self) -> UPPER_SDCLK_FREQ_SEL_W {
        UPPER_SDCLK_FREQ_SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select."]
    #[inline(always)]
    pub fn sdclk_freq_sel(&mut self) -> SDCLK_FREQ_SEL_W {
        SDCLK_FREQ_SEL_W { w: self }
    }
}
