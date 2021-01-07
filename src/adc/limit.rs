#[doc = "Reader of register LIMIT[%s]"]
pub type R = crate::R<u32, super::LIMIT>;
#[doc = "Writer for register LIMIT[%s]"]
pub type W = crate::W<u32, super::LIMIT>;
#[doc = "Register LIMIT[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::LIMIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ch_lo_limit`"]
pub type CH_LO_LIMIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ch_lo_limit`"]
pub struct CH_LO_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_LO_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `ch_hi_limit`"]
pub type CH_HI_LIMIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ch_hi_limit`"]
pub struct CH_HI_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_HI_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | (((value as u32) & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `ch_sel`"]
pub type CH_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ch_sel`"]
pub struct CH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ch_lo_limit_en`"]
pub type CH_LO_LIMIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch_lo_limit_en`"]
pub struct CH_LO_LIMIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_LO_LIMIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ch_hi_limit_en`"]
pub type CH_HI_LIMIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch_hi_limit_en`"]
pub struct CH_HI_LIMIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_HI_LIMIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&self) -> CH_LO_LIMIT_R {
        CH_LO_LIMIT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&self) -> CH_HI_LIMIT_R {
        CH_HI_LIMIT_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&self) -> CH_LO_LIMIT_EN_R {
        CH_LO_LIMIT_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&self) -> CH_HI_LIMIT_EN_R {
        CH_HI_LIMIT_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&mut self) -> CH_LO_LIMIT_W {
        CH_LO_LIMIT_W { w: self }
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&mut self) -> CH_HI_LIMIT_W {
        CH_HI_LIMIT_W { w: self }
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> CH_SEL_W {
        CH_SEL_W { w: self }
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&mut self) -> CH_LO_LIMIT_EN_W {
        CH_LO_LIMIT_EN_W { w: self }
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&mut self) -> CH_HI_LIMIT_EN_W {
        CH_HI_LIMIT_EN_W { w: self }
    }
}
