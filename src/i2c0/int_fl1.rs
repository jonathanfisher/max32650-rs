#[doc = "Reader of register INT_FL1"]
pub type R = crate::R<u32, super::INT_FL1>;
#[doc = "Writer for register INT_FL1"]
pub type W = crate::W<u32, super::INT_FL1>;
#[doc = "Register INT_FL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_FL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OVERFLOW_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<RX_OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_OVERFLOW`"]
pub type RX_OVERFLOW_R = crate::R<bool, RX_OVERFLOW_A>;
impl RX_OVERFLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OVERFLOW_A {
        match self.bits {
            false => RX_OVERFLOW_A::INACTIVE,
            true => RX_OVERFLOW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == RX_OVERFLOW_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RX_OVERFLOW_A::PENDING
    }
}
#[doc = "Write proxy for field `RX_OVERFLOW`"]
pub struct RX_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OVERFLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_A::PENDING)
    }
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
#[doc = "Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UNDERFLOW_A {
    #[doc = "0: No Interrupt is Pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<TX_UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_UNDERFLOW`"]
pub type TX_UNDERFLOW_R = crate::R<bool, TX_UNDERFLOW_A>;
impl TX_UNDERFLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UNDERFLOW_A {
        match self.bits {
            false => TX_UNDERFLOW_A::INACTIVE,
            true => TX_UNDERFLOW_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TX_UNDERFLOW_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TX_UNDERFLOW_A::PENDING
    }
}
#[doc = "Write proxy for field `TX_UNDERFLOW`"]
pub struct TX_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UNDERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_UNDERFLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Interrupt is Pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_A::PENDING)
    }
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
impl R {
    #[doc = "Bit 0 - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Overflow Interrupt. When operating as a slave receiver, this bit is set when you reach the first data bit and the RX FIFO and shift register are both full."]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W {
        RX_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Underflow Interrupt. When operating as a slave transmitter, this bit is set when you reach the first data bit and the TX FIFO is empty and the master is still asking for more data (i.e the master hasn't sent a NACK yet)."]
    #[inline(always)]
    pub fn tx_underflow(&mut self) -> TX_UNDERFLOW_W {
        TX_UNDERFLOW_W { w: self }
    }
}
