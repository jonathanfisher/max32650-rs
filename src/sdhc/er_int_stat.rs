#[doc = "Reader of register ER_INT_STAT"]
pub type R = crate::R<u16, super::ER_INT_STAT>;
#[doc = "Writer for register ER_INT_STAT"]
pub type W = crate::W<u16, super::ER_INT_STAT>;
#[doc = "Register ER_INT_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::ER_INT_STAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_TO`"]
pub type CMD_TO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_TO`"]
pub struct CMD_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_TO_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CMD_CRC`"]
pub type CMD_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_CRC`"]
pub struct CMD_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CMD_END_BIT`"]
pub type CMD_END_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_END_BIT`"]
pub struct CMD_END_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_END_BIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CMD_IDX`"]
pub type CMD_IDX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_IDX`"]
pub struct CMD_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_IDX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DATA_TO`"]
pub type DATA_TO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_TO`"]
pub struct DATA_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DATA_CRC`"]
pub type DATA_CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_CRC`"]
pub struct DATA_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DATA_END_BIT`"]
pub type DATA_END_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_END_BIT`"]
pub struct DATA_END_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_END_BIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CURRENT_LIMIT`"]
pub type CURRENT_LIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURRENT_LIMIT`"]
pub struct CURRENT_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_LIMIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `AUTO_CMD_12`"]
pub type AUTO_CMD_12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_CMD_12`"]
pub struct AUTO_CMD_12_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
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
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Timeout Error."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CMD_TO_R {
        CMD_TO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CMD_CRC_R {
        CMD_CRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CMD_END_BIT_R {
        CMD_END_BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error."]
    #[inline(always)]
    pub fn cmd_idx(&self) -> CMD_IDX_R {
        CMD_IDX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error."]
    #[inline(always)]
    pub fn data_to(&self) -> DATA_TO_R {
        DATA_TO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error."]
    #[inline(always)]
    pub fn data_crc(&self) -> DATA_CRC_R {
        DATA_CRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DATA_END_BIT_R {
        DATA_END_BIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error."]
    #[inline(always)]
    pub fn current_limit(&self) -> CURRENT_LIMIT_R {
        CURRENT_LIMIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd_12(&self) -> AUTO_CMD_12_R {
        AUTO_CMD_12_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADMA Error."]
    #[inline(always)]
    pub fn adma(&self) -> ADMA_R {
        ADMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DMA Error."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error."]
    #[inline(always)]
    pub fn cmd_to(&mut self) -> CMD_TO_W {
        CMD_TO_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&mut self) -> CMD_CRC_W {
        CMD_CRC_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_end_bit(&mut self) -> CMD_END_BIT_W {
        CMD_END_BIT_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error."]
    #[inline(always)]
    pub fn cmd_idx(&mut self) -> CMD_IDX_W {
        CMD_IDX_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error."]
    #[inline(always)]
    pub fn data_to(&mut self) -> DATA_TO_W {
        DATA_TO_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error."]
    #[inline(always)]
    pub fn data_crc(&mut self) -> DATA_CRC_W {
        DATA_CRC_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error."]
    #[inline(always)]
    pub fn data_end_bit(&mut self) -> DATA_END_BIT_W {
        DATA_END_BIT_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error."]
    #[inline(always)]
    pub fn current_limit(&mut self) -> CURRENT_LIMIT_W {
        CURRENT_LIMIT_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd_12(&mut self) -> AUTO_CMD_12_W {
        AUTO_CMD_12_W { w: self }
    }
    #[doc = "Bit 9 - ADMA Error."]
    #[inline(always)]
    pub fn adma(&mut self) -> ADMA_W {
        ADMA_W { w: self }
    }
    #[doc = "Bit 12 - DMA Error."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
}
