#[doc = "Reader of register SW_RESET"]
pub type R = crate::R<u8, super::SW_RESET>;
#[doc = "Writer for register SW_RESET"]
pub type W = crate::W<u8, super::SW_RESET>;
#[doc = "Register SW_RESET `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_RESET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESET_ALL`"]
pub type RESET_ALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_ALL`"]
pub struct RESET_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_ALL_W<'a> {
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
#[doc = "Reader of field `RESET_CMD`"]
pub type RESET_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_CMD`"]
pub struct RESET_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CMD_W<'a> {
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
#[doc = "Reader of field `RESET_DAT`"]
pub type RESET_DAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_DAT`"]
pub struct RESET_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_DAT_W<'a> {
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
    #[doc = "Bit 0 - Software Reset For All."]
    #[inline(always)]
    pub fn reset_all(&self) -> RESET_ALL_R {
        RESET_ALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line."]
    #[inline(always)]
    pub fn reset_cmd(&self) -> RESET_CMD_R {
        RESET_CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line."]
    #[inline(always)]
    pub fn reset_dat(&self) -> RESET_DAT_R {
        RESET_DAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All."]
    #[inline(always)]
    pub fn reset_all(&mut self) -> RESET_ALL_W {
        RESET_ALL_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset For CMD Line."]
    #[inline(always)]
    pub fn reset_cmd(&mut self) -> RESET_CMD_W {
        RESET_CMD_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset For DAT Line."]
    #[inline(always)]
    pub fn reset_dat(&mut self) -> RESET_DAT_W {
        RESET_DAT_W { w: self }
    }
}
