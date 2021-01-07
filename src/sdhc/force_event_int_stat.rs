#[doc = "Reader of register FORCE_EVENT_INT_STAT"]
pub type R = crate::R<u16, super::FORCE_EVENT_INT_STAT>;
#[doc = "Writer for register FORCE_EVENT_INT_STAT"]
pub type W = crate::W<u16, super::FORCE_EVENT_INT_STAT>;
#[doc = "Register FORCE_EVENT_INT_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCE_EVENT_INT_STAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_TO`"]
pub type CMD_TO_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD_CRC`"]
pub type CMD_CRC_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD_END_BIT`"]
pub type CMD_END_BIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD_INDEX`"]
pub type CMD_INDEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA_TO`"]
pub type DATA_TO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA_CRC`"]
pub type DATA_CRC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATA_END_BIT`"]
pub type DATA_END_BIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CURR_LIMIT`"]
pub type CURR_LIMIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUTO_CMD`"]
pub type AUTO_CMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADMA`"]
pub type ADMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMA`"]
pub struct ADMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `VENDOR`"]
pub struct VENDOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force Event for Command Timeout Error."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CMD_TO_R {
        CMD_TO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CMD_CRC_R {
        CMD_CRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CMD_END_BIT_R {
        CMD_END_BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error."]
    #[inline(always)]
    pub fn data_to(&self) -> DATA_TO_R {
        DATA_TO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error."]
    #[inline(always)]
    pub fn data_crc(&self) -> DATA_CRC_R {
        DATA_CRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DATA_END_BIT_R {
        DATA_END_BIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error."]
    #[inline(always)]
    pub fn curr_limit(&self) -> CURR_LIMIT_R {
        CURR_LIMIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AUTO_CMD_R {
        AUTO_CMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error."]
    #[inline(always)]
    pub fn adma(&self) -> ADMA_R {
        ADMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Force Event for ADMA Error."]
    #[inline(always)]
    pub fn adma(&mut self) -> ADMA_W {
        ADMA_W { w: self }
    }
    #[doc = "Bits 12:14 - Force Event for Vendor Specific Error Status."]
    #[inline(always)]
    pub fn vendor(&mut self) -> VENDOR_W {
        VENDOR_W { w: self }
    }
}
