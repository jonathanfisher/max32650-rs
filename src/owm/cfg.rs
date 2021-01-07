#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `long_line_mode`"]
pub type LONG_LINE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `long_line_mode`"]
pub struct LONG_LINE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LONG_LINE_MODE_W<'a> {
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
#[doc = "Reader of field `force_pres_det`"]
pub type FORCE_PRES_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `force_pres_det`"]
pub struct FORCE_PRES_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_PRES_DET_W<'a> {
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
#[doc = "Reader of field `bit_bang_en`"]
pub type BIT_BANG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit_bang_en`"]
pub struct BIT_BANG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_BANG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ext_pullup_mode`"]
pub type EXT_PULLUP_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ext_pullup_mode`"]
pub struct EXT_PULLUP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_PULLUP_MODE_W<'a> {
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
#[doc = "Reader of field `ext_pullup_enable`"]
pub type EXT_PULLUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ext_pullup_enable`"]
pub struct EXT_PULLUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_PULLUP_ENABLE_W<'a> {
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
#[doc = "Reader of field `single_bit_mode`"]
pub type SINGLE_BIT_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `single_bit_mode`"]
pub struct SINGLE_BIT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_BIT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `overdrive`"]
pub type OVERDRIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `overdrive`"]
pub struct OVERDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERDRIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `int_pullup_enable`"]
pub type INT_PULLUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `int_pullup_enable`"]
pub struct INT_PULLUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_PULLUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Long Line Mode."]
    #[inline(always)]
    pub fn long_line_mode(&self) -> LONG_LINE_MODE_R {
        LONG_LINE_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect."]
    #[inline(always)]
    pub fn force_pres_det(&self) -> FORCE_PRES_DET_R {
        FORCE_PRES_DET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Enable."]
    #[inline(always)]
    pub fn bit_bang_en(&self) -> BIT_BANG_EN_R {
        BIT_BANG_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Provide an extra output control to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&self) -> EXT_PULLUP_MODE_R {
        EXT_PULLUP_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable External Pullup."]
    #[inline(always)]
    pub fn ext_pullup_enable(&self) -> EXT_PULLUP_ENABLE_R {
        EXT_PULLUP_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode."]
    #[inline(always)]
    pub fn single_bit_mode(&self) -> SINGLE_BIT_MODE_R {
        SINGLE_BIT_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&self) -> OVERDRIVE_R {
        OVERDRIVE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable intenral pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&self) -> INT_PULLUP_ENABLE_R {
        INT_PULLUP_ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Long Line Mode."]
    #[inline(always)]
    pub fn long_line_mode(&mut self) -> LONG_LINE_MODE_W {
        LONG_LINE_MODE_W { w: self }
    }
    #[doc = "Bit 1 - Force Line During Presence Detect."]
    #[inline(always)]
    pub fn force_pres_det(&mut self) -> FORCE_PRES_DET_W {
        FORCE_PRES_DET_W { w: self }
    }
    #[doc = "Bit 2 - Bit Bang Enable."]
    #[inline(always)]
    pub fn bit_bang_en(&mut self) -> BIT_BANG_EN_W {
        BIT_BANG_EN_W { w: self }
    }
    #[doc = "Bit 3 - Provide an extra output control to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&mut self) -> EXT_PULLUP_MODE_W {
        EXT_PULLUP_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Enable External Pullup."]
    #[inline(always)]
    pub fn ext_pullup_enable(&mut self) -> EXT_PULLUP_ENABLE_W {
        EXT_PULLUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode."]
    #[inline(always)]
    pub fn single_bit_mode(&mut self) -> SINGLE_BIT_MODE_W {
        SINGLE_BIT_MODE_W { w: self }
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&mut self) -> OVERDRIVE_W {
        OVERDRIVE_W { w: self }
    }
    #[doc = "Bit 7 - Enable intenral pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&mut self) -> INT_PULLUP_ENABLE_W {
        INT_PULLUP_ENABLE_W { w: self }
    }
}
