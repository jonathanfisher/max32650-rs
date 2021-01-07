#[doc = "Reader of register CTRL_STAT"]
pub type R = crate::R<u32, super::CTRL_STAT>;
#[doc = "Writer for register CTRL_STAT"]
pub type W = crate::W<u32, super::CTRL_STAT>;
#[doc = "Register CTRL_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `start_ow_reset`"]
pub type START_OW_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `start_ow_reset`"]
pub struct START_OW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> START_OW_RESET_W<'a> {
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
#[doc = "Reader of field `sra_mode`"]
pub type SRA_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sra_mode`"]
pub struct SRA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRA_MODE_W<'a> {
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
#[doc = "Reader of field `bit_bang_oe`"]
pub type BIT_BANG_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit_bang_oe`"]
pub struct BIT_BANG_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_BANG_OE_W<'a> {
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
#[doc = "Reader of field `ow_input`"]
pub type OW_INPUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `presence_detect`"]
pub type PRESENCE_DETECT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Start OW Reset."]
    #[inline(always)]
    pub fn start_ow_reset(&self) -> START_OW_RESET_R {
        START_OW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRA Mode."]
    #[inline(always)]
    pub fn sra_mode(&self) -> SRA_MODE_R {
        SRA_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Output Enable."]
    #[inline(always)]
    pub fn bit_bang_oe(&self) -> BIT_BANG_OE_R {
        BIT_BANG_OE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OW Input State."]
    #[inline(always)]
    pub fn ow_input(&self) -> OW_INPUT_R {
        OW_INPUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Presence Pulse Detected."]
    #[inline(always)]
    pub fn presence_detect(&self) -> PRESENCE_DETECT_R {
        PRESENCE_DETECT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start OW Reset."]
    #[inline(always)]
    pub fn start_ow_reset(&mut self) -> START_OW_RESET_W {
        START_OW_RESET_W { w: self }
    }
    #[doc = "Bit 1 - SRA Mode."]
    #[inline(always)]
    pub fn sra_mode(&mut self) -> SRA_MODE_W {
        SRA_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Bit Bang Output Enable."]
    #[inline(always)]
    pub fn bit_bang_oe(&mut self) -> BIT_BANG_OE_W {
        BIT_BANG_OE_W { w: self }
    }
}
