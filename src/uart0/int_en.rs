#[doc = "Reader of register INT_EN"]
pub type R = crate::R<u32, super::INT_EN>;
#[doc = "Writer for register INT_EN"]
pub type W = crate::W<u32, super::INT_EN>;
#[doc = "Register INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_FRAME_ERROR`"]
pub type RX_FRAME_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FRAME_ERROR`"]
pub struct RX_FRAME_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRAME_ERROR_W<'a> {
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
#[doc = "Reader of field `RX_PARITY_ERROR`"]
pub type RX_PARITY_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_PARITY_ERROR`"]
pub struct RX_PARITY_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PARITY_ERROR_W<'a> {
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
#[doc = "Reader of field `CTS_CHANGE`"]
pub type CTS_CHANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS_CHANGE`"]
pub struct CTS_CHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHANGE_W<'a> {
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
#[doc = "Reader of field `RX_OVERRUN`"]
pub type RX_OVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_OVERRUN`"]
pub struct RX_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERRUN_W<'a> {
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
#[doc = "Reader of field `RX_FIFO_THRESH`"]
pub type RX_FIFO_THRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FIFO_THRESH`"]
pub struct RX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_THRESH_W<'a> {
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
#[doc = "Reader of field `TX_FIFO_ALMOST_EMPTY`"]
pub type TX_FIFO_ALMOST_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FIFO_ALMOST_EMPTY`"]
pub struct TX_FIFO_ALMOST_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_ALMOST_EMPTY_W<'a> {
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
#[doc = "Reader of field `TX_FIFO_THRESH`"]
pub type TX_FIFO_THRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FIFO_THRESH`"]
pub struct TX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_THRESH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BREAK`"]
pub type BREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BREAK`"]
pub struct BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RX_TIMEOUT`"]
pub type RX_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_TIMEOUT`"]
pub struct RX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LAST_BREAK`"]
pub type LAST_BREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LAST_BREAK`"]
pub struct LAST_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_frame_error(&self) -> RX_FRAME_ERROR_R {
        RX_FRAME_ERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable for RX Parity Error interrupt."]
    #[inline(always)]
    pub fn rx_parity_error(&self) -> RX_PARITY_ERROR_R {
        RX_PARITY_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable for CTS signal change interrupt."]
    #[inline(always)]
    pub fn cts_change(&self) -> CTS_CHANGE_R {
        CTS_CHANGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for RX FIFO OVerrun interrupt."]
    #[inline(always)]
    pub fn rx_overrun(&self) -> RX_OVERRUN_R {
        RX_OVERRUN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&self) -> RX_FIFO_THRESH_R {
        RX_FIFO_THRESH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_fifo_almost_empty(&self) -> TX_FIFO_ALMOST_EMPTY_R {
        TX_FIFO_ALMOST_EMPTY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&self) -> TX_FIFO_THRESH_R {
        TX_FIFO_THRESH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable for received BREAK character interrupt."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable for RX Timeout Interrupt. Trigger if there is no RX communication during n UART characters (n=UART_CN.RXTO)."]
    #[inline(always)]
    pub fn rx_timeout(&self) -> RX_TIMEOUT_R {
        RX_TIMEOUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable for Last break character interrupt."]
    #[inline(always)]
    pub fn last_break(&self) -> LAST_BREAK_R {
        LAST_BREAK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_frame_error(&mut self) -> RX_FRAME_ERROR_W {
        RX_FRAME_ERROR_W { w: self }
    }
    #[doc = "Bit 1 - Enable for RX Parity Error interrupt."]
    #[inline(always)]
    pub fn rx_parity_error(&mut self) -> RX_PARITY_ERROR_W {
        RX_PARITY_ERROR_W { w: self }
    }
    #[doc = "Bit 2 - Enable for CTS signal change interrupt."]
    #[inline(always)]
    pub fn cts_change(&mut self) -> CTS_CHANGE_W {
        CTS_CHANGE_W { w: self }
    }
    #[doc = "Bit 3 - Enable for RX FIFO OVerrun interrupt."]
    #[inline(always)]
    pub fn rx_overrun(&mut self) -> RX_OVERRUN_W {
        RX_OVERRUN_W { w: self }
    }
    #[doc = "Bit 4 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&mut self) -> RX_FIFO_THRESH_W {
        RX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bit 5 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_fifo_almost_empty(&mut self) -> TX_FIFO_ALMOST_EMPTY_W {
        TX_FIFO_ALMOST_EMPTY_W { w: self }
    }
    #[doc = "Bit 6 - Enable for interrupt when TX FIFO reaches the number of bytes configured by the TXTHD field."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&mut self) -> TX_FIFO_THRESH_W {
        TX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bit 7 - Enable for received BREAK character interrupt."]
    #[inline(always)]
    pub fn break_(&mut self) -> BREAK_W {
        BREAK_W { w: self }
    }
    #[doc = "Bit 8 - Enable for RX Timeout Interrupt. Trigger if there is no RX communication during n UART characters (n=UART_CN.RXTO)."]
    #[inline(always)]
    pub fn rx_timeout(&mut self) -> RX_TIMEOUT_W {
        RX_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 9 - Enable for Last break character interrupt."]
    #[inline(always)]
    pub fn last_break(&mut self) -> LAST_BREAK_W {
        LAST_BREAK_W { w: self }
    }
}
