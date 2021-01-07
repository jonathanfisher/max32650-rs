#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ow_reset_done`"]
pub type OW_RESET_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ow_reset_done`"]
pub struct OW_RESET_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> OW_RESET_DONE_W<'a> {
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
#[doc = "Reader of field `tx_data_empty`"]
pub type TX_DATA_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_data_empty`"]
pub struct TX_DATA_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_EMPTY_W<'a> {
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
#[doc = "Reader of field `rx_data_ready`"]
pub type RX_DATA_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_data_ready`"]
pub struct RX_DATA_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_READY_W<'a> {
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
#[doc = "Reader of field `line_short`"]
pub type LINE_SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `line_short`"]
pub struct LINE_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_SHORT_W<'a> {
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
#[doc = "Reader of field `line_low`"]
pub type LINE_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `line_low`"]
pub struct LINE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_LOW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - OW Reset Sequence Completed."]
    #[inline(always)]
    pub fn ow_reset_done(&self) -> OW_RESET_DONE_R {
        OW_RESET_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Enable."]
    #[inline(always)]
    pub fn tx_data_empty(&self) -> TX_DATA_EMPTY_R {
        TX_DATA_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rx_data_ready(&self) -> RX_DATA_READY_R {
        RX_DATA_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Enable."]
    #[inline(always)]
    pub fn line_short(&self) -> LINE_SHORT_R {
        LINE_SHORT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Enable."]
    #[inline(always)]
    pub fn line_low(&self) -> LINE_LOW_R {
        LINE_LOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OW Reset Sequence Completed."]
    #[inline(always)]
    pub fn ow_reset_done(&mut self) -> OW_RESET_DONE_W {
        OW_RESET_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Enable."]
    #[inline(always)]
    pub fn tx_data_empty(&mut self) -> TX_DATA_EMPTY_W {
        TX_DATA_EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rx_data_ready(&mut self) -> RX_DATA_READY_W {
        RX_DATA_READY_W { w: self }
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Enable."]
    #[inline(always)]
    pub fn line_short(&mut self) -> LINE_SHORT_W {
        LINE_SHORT_W { w: self }
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Enable."]
    #[inline(always)]
    pub fn line_low(&mut self) -> LINE_LOW_W {
        LINE_LOW_W { w: self }
    }
}
