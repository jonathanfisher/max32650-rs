#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `start`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `start`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `pwr`"]
pub type PWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwr`"]
pub struct PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `refbuf_pwr`"]
pub type REFBUF_PWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `refbuf_pwr`"]
pub struct REFBUF_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> REFBUF_PWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `chgpump_pwr`"]
pub type CHGPUMP_PWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `chgpump_pwr`"]
pub struct CHGPUMP_PWR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHGPUMP_PWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ref_scale`"]
pub type REF_SCALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ref_scale`"]
pub struct REF_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_SCALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `scale`"]
pub type SCALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `scale`"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ref_sel`"]
pub type REF_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ref_sel`"]
pub struct REF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `clk_en`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_en`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `adc_xref`"]
pub type ADC_XREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `adc_xref`"]
pub struct ADC_XREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_XREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `data_align`"]
pub type DATA_ALIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `data_align`"]
pub struct DATA_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ALIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn refbuf_pwr(&self) -> REFBUF_PWR_R {
        REFBUF_PWR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC Charge Pump Power Up"]
    #[inline(always)]
    pub fn chgpump_pwr(&self) -> CHGPUMP_PWR_R {
        CHGPUMP_PWR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn ref_scale(&self) -> REF_SCALE_R {
        REF_SCALE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC Reference (VRef) Select (INTERNAL ONLY)"]
    #[inline(always)]
    pub fn ref_sel(&self) -> REF_SEL_R {
        REF_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable Use of ADC External Reference"]
    #[inline(always)]
    pub fn adc_xref(&self) -> ADC_XREF_R {
        ADC_XREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn data_align(&self) -> DATA_ALIGN_R {
        DATA_ALIGN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PWR_W {
        PWR_W { w: self }
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn refbuf_pwr(&mut self) -> REFBUF_PWR_W {
        REFBUF_PWR_W { w: self }
    }
    #[doc = "Bit 4 - ADC Charge Pump Power Up"]
    #[inline(always)]
    pub fn chgpump_pwr(&mut self) -> CHGPUMP_PWR_W {
        CHGPUMP_PWR_W { w: self }
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn ref_scale(&mut self) -> REF_SCALE_W {
        REF_SCALE_W { w: self }
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 10 - ADC Reference (VRef) Select (INTERNAL ONLY)"]
    #[inline(always)]
    pub fn ref_sel(&mut self) -> REF_SEL_W {
        REF_SEL_W { w: self }
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bits 12:15 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> CH_SEL_W {
        CH_SEL_W { w: self }
    }
    #[doc = "Bit 16 - Enable Use of ADC External Reference"]
    #[inline(always)]
    pub fn adc_xref(&mut self) -> ADC_XREF_W {
        ADC_XREF_W { w: self }
    }
    #[doc = "Bit 17 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn data_align(&mut self) -> DATA_ALIGN_W {
        DATA_ALIGN_W { w: self }
    }
}
