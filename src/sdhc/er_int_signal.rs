#[doc = "Reader of register ER_INT_SIGNAL"]
pub type R = crate::R<u16, super::ER_INT_SIGNAL>;
#[doc = "Writer for register ER_INT_SIGNAL"]
pub type W = crate::W<u16, super::ER_INT_SIGNAL>;
#[doc = "Register ER_INT_SIGNAL `reset()`'s with value 0"]
impl crate::ResetValue for super::ER_INT_SIGNAL {
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
#[doc = "Reader of field `CURR_LIM`"]
pub type CURR_LIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURR_LIM`"]
pub struct CURR_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CURR_LIM_W<'a> {
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
#[doc = "Reader of field `AUTO_CMD`"]
pub type AUTO_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_CMD`"]
pub struct AUTO_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD_W<'a> {
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
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TAR_RESP`"]
pub type TAR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAR_RESP`"]
pub struct TAR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_RESP_W<'a> {
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
    #[doc = "Bit 0 - Command Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CMD_TO_R {
        CMD_TO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CMD_CRC_R {
        CMD_CRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CMD_END_BIT_R {
        CMD_END_BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_idx(&self) -> CMD_IDX_R {
        CMD_IDX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn data_to(&self) -> DATA_TO_R {
        DATA_TO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable."]
    #[inline(always)]
    pub fn data_crc(&self) -> DATA_CRC_R {
        DATA_CRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DATA_END_BIT_R {
        DATA_END_BIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable."]
    #[inline(always)]
    pub fn curr_lim(&self) -> CURR_LIM_R {
        CURR_LIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AUTO_CMD_R {
        AUTO_CMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable."]
    #[inline(always)]
    pub fn adma(&self) -> ADMA_R {
        ADMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tuning Error Signal Enable."]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable."]
    #[inline(always)]
    pub fn tar_resp(&self) -> TAR_RESP_R {
        TAR_RESP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_to(&mut self) -> CMD_TO_W {
        CMD_TO_W { w: self }
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_crc(&mut self) -> CMD_CRC_W {
        CMD_CRC_W { w: self }
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_end_bit(&mut self) -> CMD_END_BIT_W {
        CMD_END_BIT_W { w: self }
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_idx(&mut self) -> CMD_IDX_W {
        CMD_IDX_W { w: self }
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn data_to(&mut self) -> DATA_TO_W {
        DATA_TO_W { w: self }
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable."]
    #[inline(always)]
    pub fn data_crc(&mut self) -> DATA_CRC_W {
        DATA_CRC_W { w: self }
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn data_end_bit(&mut self) -> DATA_END_BIT_W {
        DATA_END_BIT_W { w: self }
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable."]
    #[inline(always)]
    pub fn curr_lim(&mut self) -> CURR_LIM_W {
        CURR_LIM_W { w: self }
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable."]
    #[inline(always)]
    pub fn auto_cmd(&mut self) -> AUTO_CMD_W {
        AUTO_CMD_W { w: self }
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable."]
    #[inline(always)]
    pub fn adma(&mut self) -> ADMA_W {
        ADMA_W { w: self }
    }
    #[doc = "Bit 10 - Tuning Error Signal Enable."]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable."]
    #[inline(always)]
    pub fn tar_resp(&mut self) -> TAR_RESP_W {
        TAR_RESP_W { w: self }
    }
}
