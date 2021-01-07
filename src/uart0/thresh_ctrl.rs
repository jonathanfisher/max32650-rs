#[doc = "Reader of register THRESH_CTRL"]
pub type R = crate::R<u32, super::THRESH_CTRL>;
#[doc = "Writer for register THRESH_CTRL"]
pub type W = crate::W<u32, super::THRESH_CTRL>;
#[doc = "Register THRESH_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::THRESH_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_FIFO_THRESH`"]
pub type RX_FIFO_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FIFO_THRESH`"]
pub struct RX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TX_FIFO_THRESH`"]
pub type TX_FIFO_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_FIFO_THRESH`"]
pub struct TX_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTS_FIFO_THRESH`"]
pub type RTS_FIFO_THRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTS_FIFO_THRESH`"]
pub struct RTS_FIFO_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_FIFO_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - RX FIFO Threshold Level.When the RX FIFO reaches this many bytes or higher, UARTn_INFTL.rx_fifo_level is set."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&self) -> RX_FIFO_THRESH_R {
        RX_FIFO_THRESH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - TX FIFO Threshold Level. When the TX FIFO reaches this many bytes or higher, UARTn_INTFL.tx_fifo_level is set."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&self) -> TX_FIFO_THRESH_R {
        TX_FIFO_THRESH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - RTS threshold control. When the RX FIFO reaches this many bytes or higher, the RTS output signal is deasserted, informing the transmitting UART to stop sending data to this UART."]
    #[inline(always)]
    pub fn rts_fifo_thresh(&self) -> RTS_FIFO_THRESH_R {
        RTS_FIFO_THRESH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX FIFO Threshold Level.When the RX FIFO reaches this many bytes or higher, UARTn_INFTL.rx_fifo_level is set."]
    #[inline(always)]
    pub fn rx_fifo_thresh(&mut self) -> RX_FIFO_THRESH_W {
        RX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bits 8:13 - TX FIFO Threshold Level. When the TX FIFO reaches this many bytes or higher, UARTn_INTFL.tx_fifo_level is set."]
    #[inline(always)]
    pub fn tx_fifo_thresh(&mut self) -> TX_FIFO_THRESH_W {
        TX_FIFO_THRESH_W { w: self }
    }
    #[doc = "Bits 16:21 - RTS threshold control. When the RX FIFO reaches this many bytes or higher, the RTS output signal is deasserted, informing the transmitting UART to stop sending data to this UART."]
    #[inline(always)]
    pub fn rts_fifo_thresh(&mut self) -> RTS_FIFO_THRESH_W {
        RTS_FIFO_THRESH_W { w: self }
    }
}
