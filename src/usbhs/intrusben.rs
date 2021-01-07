#[doc = "Reader of register INTRUSBEN"]
pub type R = crate::R<u8, super::INTRUSBEN>;
#[doc = "Writer for register INTRUSBEN"]
pub type W = crate::W<u8, super::INTRUSBEN>;
#[doc = "Register INTRUSBEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTRUSBEN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOF_INT_EN`"]
pub type SOF_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_INT_EN`"]
pub struct SOF_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RESET_INT_EN`"]
pub type RESET_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_INT_EN`"]
pub struct RESET_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESUME_INT_EN`"]
pub type RESUME_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME_INT_EN`"]
pub struct RESUME_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SUSPEND_INT_EN`"]
pub type SUSPEND_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND_INT_EN`"]
pub struct SUSPEND_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Start of Frame."]
    #[inline(always)]
    pub fn sof_int_en(&self) -> SOF_INT_EN_R {
        SOF_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus reset detected."]
    #[inline(always)]
    pub fn reset_int_en(&self) -> RESET_INT_EN_R {
        RESET_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resume detected."]
    #[inline(always)]
    pub fn resume_int_en(&self) -> RESUME_INT_EN_R {
        RESUME_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Suspend detected."]
    #[inline(always)]
    pub fn suspend_int_en(&self) -> SUSPEND_INT_EN_R {
        SUSPEND_INT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Start of Frame."]
    #[inline(always)]
    pub fn sof_int_en(&mut self) -> SOF_INT_EN_W {
        SOF_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Bus reset detected."]
    #[inline(always)]
    pub fn reset_int_en(&mut self) -> RESET_INT_EN_W {
        RESET_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Resume detected."]
    #[inline(always)]
    pub fn resume_int_en(&mut self) -> RESUME_INT_EN_W {
        RESUME_INT_EN_W { w: self }
    }
    #[doc = "Bit 0 - Suspend detected."]
    #[inline(always)]
    pub fn suspend_int_en(&mut self) -> SUSPEND_INT_EN_W {
        SUSPEND_INT_EN_W { w: self }
    }
}
