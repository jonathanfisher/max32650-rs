#[doc = "Reader of register WAKEUP"]
pub type R = crate::R<u8, super::WAKEUP>;
#[doc = "Writer for register WAKEUP"]
pub type W = crate::W<u8, super::WAKEUP>;
#[doc = "Register WAKEUP `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARD_INT`"]
pub type CARD_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INT`"]
pub struct CARD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INT_W<'a> {
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
#[doc = "Reader of field `CARD_INS`"]
pub type CARD_INS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INS`"]
pub struct CARD_INS_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INS_W<'a> {
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
#[doc = "Reader of field `CARD_REM`"]
pub type CARD_REM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_REM`"]
pub struct CARD_REM_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt."]
    #[inline(always)]
    pub fn card_int(&self) -> CARD_INT_R {
        CARD_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion."]
    #[inline(always)]
    pub fn card_ins(&self) -> CARD_INS_R {
        CARD_INS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal."]
    #[inline(always)]
    pub fn card_rem(&self) -> CARD_REM_R {
        CARD_REM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt."]
    #[inline(always)]
    pub fn card_int(&mut self) -> CARD_INT_W {
        CARD_INT_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion."]
    #[inline(always)]
    pub fn card_ins(&mut self) -> CARD_INS_W {
        CARD_INS_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal."]
    #[inline(always)]
    pub fn card_rem(&mut self) -> CARD_REM_W {
        CARD_REM_W { w: self }
    }
}
