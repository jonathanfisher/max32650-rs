#[doc = "Reader of register FIFO_LEN"]
pub type R = crate::R<u32, super::FIFO_LEN>;
#[doc = "Reader of field `RX_LEN`"]
pub type RX_LEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX_LEN`"]
pub type TX_LEN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO Length."]
    #[inline(always)]
    pub fn rx_len(&self) -> RX_LEN_R {
        RX_LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit FIFO Length."]
    #[inline(always)]
    pub fn tx_len(&self) -> TX_LEN_R {
        TX_LEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
