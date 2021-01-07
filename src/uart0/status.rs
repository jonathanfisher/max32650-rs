#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `TX_BUSY`"]
pub type TX_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_BUSY`"]
pub type RX_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BREAK`"]
pub type BREAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_EMPTY`"]
pub type RX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FULL`"]
pub type RX_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_EMPTY`"]
pub type TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TX_FULL`"]
pub type TX_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_FIFO_CNT`"]
pub type RX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX_FIFO_CNT`"]
pub type TX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RX_TO`"]
pub type RX_TO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read-only flag indicating the UART transmit status."]
    #[inline(always)]
    pub fn tx_busy(&self) -> TX_BUSY_R {
        TX_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read-only flag indicating the UARTreceiver status."]
    #[inline(always)]
    pub fn rx_busy(&self) -> RX_BUSY_R {
        RX_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 9th Received bit state. This bit identifies the state of the 9th bit of received data. Only available for UART_CTRL.SIZE\\[1:0\\]=3."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Received BREAK status. BREAKS is cleared when UART_STAT register is read. Received data input is held in spacing (logic 0) state for longer than a full word transmission time (that is, the total time of Start bit + data bits + Parity + Stop bits)."]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read-only flag indicating the RX FIFO state."]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read-only flag indicating the RX FIFO state."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read-only flag indicating the TX FIFO state."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read-only flag indicating the TX FIFO state."]
    #[inline(always)]
    pub fn tx_full(&self) -> TX_FULL_R {
        TX_FULL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Indicates the number of bytes currently in the RX FIFO."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Indicates the number of bytes currently in the TX FIFO."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - RX Timeout status."]
    #[inline(always)]
    pub fn rx_to(&self) -> RX_TO_R {
        RX_TO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
