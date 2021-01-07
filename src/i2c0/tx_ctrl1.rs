#[doc = "Reader of register TX_CTRL1"]
pub type R = crate::R<u32, super::TX_CTRL1>;
#[doc = "Writer for register TX_CTRL1"]
pub type W = crate::W<u32, super::TX_CTRL1>;
#[doc = "Register TX_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_READY`"]
pub type TX_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_READY`"]
pub struct TX_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_W<'a> {
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
#[doc = "Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_LAST_A {
    #[doc = "0: Hold SCL low on TX_FIFO empty."]
    HOLD_SCL_LOW = 0,
    #[doc = "1: End transaction on TX_FIFO empty."]
    END_TRANSACTION = 1,
}
impl From<TX_LAST_A> for bool {
    #[inline(always)]
    fn from(variant: TX_LAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_LAST`"]
pub type TX_LAST_R = crate::R<bool, TX_LAST_A>;
impl TX_LAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_LAST_A {
        match self.bits {
            false => TX_LAST_A::HOLD_SCL_LOW,
            true => TX_LAST_A::END_TRANSACTION,
        }
    }
    #[doc = "Checks if the value of the field is `HOLD_SCL_LOW`"]
    #[inline(always)]
    pub fn is_hold_scl_low(&self) -> bool {
        *self == TX_LAST_A::HOLD_SCL_LOW
    }
    #[doc = "Checks if the value of the field is `END_TRANSACTION`"]
    #[inline(always)]
    pub fn is_end_transaction(&self) -> bool {
        *self == TX_LAST_A::END_TRANSACTION
    }
}
#[doc = "Write proxy for field `TX_LAST`"]
pub struct TX_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_LAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hold SCL low on TX_FIFO empty."]
    #[inline(always)]
    pub fn hold_scl_low(self) -> &'a mut W {
        self.variant(TX_LAST_A::HOLD_SCL_LOW)
    }
    #[doc = "End transaction on TX_FIFO empty."]
    #[inline(always)]
    pub fn end_transaction(self) -> &'a mut W {
        self.variant(TX_LAST_A::END_TRANSACTION)
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
#[doc = "Reader of field `TX_FIFO`"]
pub type TX_FIFO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware)."]
    #[inline(always)]
    pub fn tx_last(&self) -> TX_LAST_R {
        TX_LAST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count. These bits reflect the number of bytes in the TX_FIFO."]
    #[inline(always)]
    pub fn tx_fifo(&self) -> TX_FIFO_R {
        TX_FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Preload Ready."]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Last. This bit is used in slave mod only. Do not use when preloading (cleared by hardware)."]
    #[inline(always)]
    pub fn tx_last(&mut self) -> TX_LAST_W {
        TX_LAST_W { w: self }
    }
}
