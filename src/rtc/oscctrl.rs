#[doc = "Reader of register OSCCTRL"]
pub type R = crate::R<u32, super::OSCCTRL>;
#[doc = "Writer for register OSCCTRL"]
pub type W = crate::W<u32, super::OSCCTRL>;
#[doc = "Register OSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLITER_EN`"]
pub type FLITER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLITER_EN`"]
pub struct FLITER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITER_EN_W<'a> {
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
#[doc = "RTC Oscillator 4X Bias Current Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBIAS_SEL_A {
    #[doc = "0: Selects 2X bias current for RTC oscillator"]
    _2X = 0,
    #[doc = "1: Selects 4X bias current for RTC oscillator"]
    _4X = 1,
}
impl From<IBIAS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IBIAS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IBIAS_SEL`"]
pub type IBIAS_SEL_R = crate::R<bool, IBIAS_SEL_A>;
impl IBIAS_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBIAS_SEL_A {
        match self.bits {
            false => IBIAS_SEL_A::_2X,
            true => IBIAS_SEL_A::_4X,
        }
    }
    #[doc = "Checks if the value of the field is `_2X`"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == IBIAS_SEL_A::_2X
    }
    #[doc = "Checks if the value of the field is `_4X`"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == IBIAS_SEL_A::_4X
    }
}
#[doc = "Write proxy for field `IBIAS_SEL`"]
pub struct IBIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBIAS_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects 2X bias current for RTC oscillator"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut W {
        self.variant(IBIAS_SEL_A::_2X)
    }
    #[doc = "Selects 4X bias current for RTC oscillator"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut W {
        self.variant(IBIAS_SEL_A::_4X)
    }
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
#[doc = "Reader of field `HYST_EN`"]
pub type HYST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYST_EN`"]
pub struct HYST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_EN_W<'a> {
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
#[doc = "Reader of field `IBIAS_EN`"]
pub type IBIAS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBIAS_EN`"]
pub struct IBIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_EN_W<'a> {
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
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `32KOUT`"]
pub type _32KOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `32KOUT`"]
pub struct _32KOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> _32KOUT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RTC Oscillator Filter Enable"]
    #[inline(always)]
    pub fn fliter_en(&self) -> FLITER_EN_R {
        FLITER_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Oscillator 4X Bias Current Select"]
    #[inline(always)]
    pub fn ibias_sel(&self) -> IBIAS_SEL_R {
        IBIAS_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Oscillator Hysteresis Buffer Enable"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Oscillator Bias Current Enable"]
    #[inline(always)]
    pub fn ibias_en(&self) -> IBIAS_EN_R {
        IBIAS_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn _32kout(&self) -> _32KOUT_R {
        _32KOUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Oscillator Filter Enable"]
    #[inline(always)]
    pub fn fliter_en(&mut self) -> FLITER_EN_W {
        FLITER_EN_W { w: self }
    }
    #[doc = "Bit 1 - RTC Oscillator 4X Bias Current Select"]
    #[inline(always)]
    pub fn ibias_sel(&mut self) -> IBIAS_SEL_W {
        IBIAS_SEL_W { w: self }
    }
    #[doc = "Bit 2 - RTC Oscillator Hysteresis Buffer Enable"]
    #[inline(always)]
    pub fn hyst_en(&mut self) -> HYST_EN_W {
        HYST_EN_W { w: self }
    }
    #[doc = "Bit 3 - RTC Oscillator Bias Current Enable"]
    #[inline(always)]
    pub fn ibias_en(&mut self) -> IBIAS_EN_W {
        IBIAS_EN_W { w: self }
    }
    #[doc = "Bit 4 - RTC Crystal Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - RTC 32kHz Square Wave Output"]
    #[inline(always)]
    pub fn _32kout(&mut self) -> _32KOUT_W {
        _32KOUT_W { w: self }
    }
}
