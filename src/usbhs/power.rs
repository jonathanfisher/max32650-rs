#[doc = "Reader of register POWER"]
pub type R = crate::R<u8, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u8, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN_SUSPENDM`"]
pub type EN_SUSPENDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_SUSPENDM`"]
pub struct EN_SUSPENDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_SUSPENDM_W<'a> {
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
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Reader of field `HS_MODE`"]
pub type HS_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HS_ENABLE`"]
pub type HS_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HS_ENABLE`"]
pub struct HS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SOFTCONN`"]
pub type SOFTCONN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTCONN`"]
pub struct SOFTCONN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTCONN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ISO_UPDATE`"]
pub type ISO_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO_UPDATE`"]
pub struct ISO_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable SUSPENDM signal."]
    #[inline(always)]
    pub fn en_suspendm(&self) -> EN_SUSPENDM_R {
        EN_SUSPENDM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Suspend mode detected."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generate resume signaling."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus reset detected."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - High-speed mode detected."]
    #[inline(always)]
    pub fn hs_mode(&self) -> HS_MODE_R {
        HS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High-speed mode enable."]
    #[inline(always)]
    pub fn hs_enable(&self) -> HS_ENABLE_R {
        HS_ENABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Softconn."]
    #[inline(always)]
    pub fn softconn(&self) -> SOFTCONN_R {
        SOFTCONN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait for SOF during Isochronous xfers."]
    #[inline(always)]
    pub fn iso_update(&self) -> ISO_UPDATE_R {
        ISO_UPDATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SUSPENDM signal."]
    #[inline(always)]
    pub fn en_suspendm(&mut self) -> EN_SUSPENDM_W {
        EN_SUSPENDM_W { w: self }
    }
    #[doc = "Bit 2 - Generate resume signaling."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 5 - High-speed mode enable."]
    #[inline(always)]
    pub fn hs_enable(&mut self) -> HS_ENABLE_W {
        HS_ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Softconn."]
    #[inline(always)]
    pub fn softconn(&mut self) -> SOFTCONN_W {
        SOFTCONN_W { w: self }
    }
    #[doc = "Bit 7 - Wait for SOF during Isochronous xfers."]
    #[inline(always)]
    pub fn iso_update(&mut self) -> ISO_UPDATE_W {
        ISO_UPDATE_W { w: self }
    }
}
