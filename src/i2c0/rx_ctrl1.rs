#[doc = "Reader of register RX_CTRL1"]
pub type R = crate::R<u32, super::RX_CTRL1>;
#[doc = "Writer for register RX_CTRL1"]
pub type W = crate::W<u32, super::RX_CTRL1>;
#[doc = "Register RX_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_CNT`"]
pub type RX_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_CNT`"]
pub struct RX_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RX_FIFO`"]
pub type RX_FIFO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Receive FIFO Count. These bits reflect the number of byte in the RX_FIFO. These bits are flushed when I2CEN = 0."]
    #[inline(always)]
    pub fn rx_fifo(&self) -> RX_FIFO_R {
        RX_FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Count Bits. These bits define the number of bytes to be received in a transaction, except for the case RXCNT = 0. RXCNT = 0 means 256 bytes to be received in a transaction."]
    #[inline(always)]
    pub fn rx_cnt(&mut self) -> RX_CNT_W {
        RX_CNT_W { w: self }
    }
}
