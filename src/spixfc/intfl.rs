#[doc = "Reader of register INTFL"]
pub type R = crate::R<u32, super::INTFL>;
#[doc = "Writer for register INTFL"]
pub type W = crate::W<u32, super::INTFL>;
#[doc = "Register INTFL `reset()`'s with value 0"]
impl crate::ResetValue for super::INTFL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transaction Stalled Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_STALLED_A {
    #[doc = "0: Normal FIFO Transaction."]
    CLR = 0,
    #[doc = "1: Stalled FIFO Transaction."]
    SET = 1,
}
impl From<TX_STALLED_A> for bool {
    #[inline(always)]
    fn from(variant: TX_STALLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_STALLED`"]
pub type TX_STALLED_R = crate::R<bool, TX_STALLED_A>;
impl TX_STALLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_STALLED_A {
        match self.bits {
            false => TX_STALLED_A::CLR,
            true => TX_STALLED_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TX_STALLED_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TX_STALLED_A::SET
    }
}
#[doc = "Write proxy for field `TX_STALLED`"]
pub struct TX_STALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STALLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_STALLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal FIFO Transaction."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(TX_STALLED_A::CLR)
    }
    #[doc = "Stalled FIFO Transaction."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TX_STALLED_A::SET)
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
#[doc = "Results Stalled Interrupt Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_STALLED_A {
    #[doc = "0: Normal FIFO Operation."]
    CLR = 0,
    #[doc = "1: Stalled FIFO."]
    SET = 1,
}
impl From<RX_STALLED_A> for bool {
    #[inline(always)]
    fn from(variant: RX_STALLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_STALLED`"]
pub type RX_STALLED_R = crate::R<bool, RX_STALLED_A>;
impl RX_STALLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_STALLED_A {
        match self.bits {
            false => RX_STALLED_A::CLR,
            true => RX_STALLED_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == RX_STALLED_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RX_STALLED_A::SET
    }
}
#[doc = "Write proxy for field `RX_STALLED`"]
pub struct RX_STALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_STALLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_STALLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal FIFO Operation."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(RX_STALLED_A::CLR)
    }
    #[doc = "Stalled FIFO."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RX_STALLED_A::SET)
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
#[doc = "Transaction Ready Interrupt Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_READY_A {
    #[doc = "0: FIFO Transaction not ready."]
    CLR = 0,
    #[doc = "1: FIFO Transaction ready."]
    SET = 1,
}
impl From<TX_READY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_READY`"]
pub type TX_READY_R = crate::R<bool, TX_READY_A>;
impl TX_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_READY_A {
        match self.bits {
            false => TX_READY_A::CLR,
            true => TX_READY_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TX_READY_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TX_READY_A::SET
    }
}
#[doc = "Write proxy for field `TX_READY`"]
pub struct TX_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO Transaction not ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(TX_READY_A::CLR)
    }
    #[doc = "FIFO Transaction ready."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TX_READY_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Results Done Interrupt Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DONE_A {
    #[doc = "0: Results FIFO ready."]
    CLR = 0,
    #[doc = "1: Results FIFO Not ready."]
    SET = 1,
}
impl From<RX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_DONE`"]
pub type RX_DONE_R = crate::R<bool, RX_DONE_A>;
impl RX_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DONE_A {
        match self.bits {
            false => RX_DONE_A::CLR,
            true => RX_DONE_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == RX_DONE_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RX_DONE_A::SET
    }
}
#[doc = "Write proxy for field `RX_DONE`"]
pub struct RX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Results FIFO ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(RX_DONE_A::CLR)
    }
    #[doc = "Results FIFO Not ready."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RX_DONE_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Transaction FIFO Almost Empty Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_AE_A {
    #[doc = "0: Transaction FIFO not Almost Empty."]
    CLR = 0,
    #[doc = "1: Transaction FIFO Almost Empty."]
    SET = 1,
}
impl From<TX_FIFO_AE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_FIFO_AE`"]
pub type TX_FIFO_AE_R = crate::R<bool, TX_FIFO_AE_A>;
impl TX_FIFO_AE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_AE_A {
        match self.bits {
            false => TX_FIFO_AE_A::CLR,
            true => TX_FIFO_AE_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == TX_FIFO_AE_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TX_FIFO_AE_A::SET
    }
}
#[doc = "Write proxy for field `TX_FIFO_AE`"]
pub struct TX_FIFO_AE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_AE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transaction FIFO not Almost Empty."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(TX_FIFO_AE_A::CLR)
    }
    #[doc = "Transaction FIFO Almost Empty."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TX_FIFO_AE_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Results FIFO Almost Full Flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_AF_A {
    #[doc = "0: Results FIFO level below the Almost Full level."]
    CLR = 0,
    #[doc = "1: Results FIFO level at Almost Full level."]
    SET = 1,
}
impl From<RX_FIFO_AF_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_AF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_FIFO_AF`"]
pub type RX_FIFO_AF_R = crate::R<bool, RX_FIFO_AF_A>;
impl RX_FIFO_AF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_AF_A {
        match self.bits {
            false => RX_FIFO_AF_A::CLR,
            true => RX_FIFO_AF_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == RX_FIFO_AF_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RX_FIFO_AF_A::SET
    }
}
#[doc = "Write proxy for field `RX_FIFO_AF`"]
pub struct RX_FIFO_AF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_AF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_AF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Results FIFO level below the Almost Full level."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(RX_FIFO_AF_A::CLR)
    }
    #[doc = "Results FIFO level at Almost Full level."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RX_FIFO_AF_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Stalled Interrupt Flag."]
    #[inline(always)]
    pub fn tx_stalled(&self) -> TX_STALLED_R {
        TX_STALLED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Results Stalled Interrupt Flag."]
    #[inline(always)]
    pub fn rx_stalled(&self) -> RX_STALLED_R {
        RX_STALLED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transaction Ready Interrupt Status."]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Results Done Interrupt Status."]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transaction FIFO Almost Empty Flag."]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Results FIFO Almost Full Flag."]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Stalled Interrupt Flag."]
    #[inline(always)]
    pub fn tx_stalled(&mut self) -> TX_STALLED_W {
        TX_STALLED_W { w: self }
    }
    #[doc = "Bit 1 - Results Stalled Interrupt Flag."]
    #[inline(always)]
    pub fn rx_stalled(&mut self) -> RX_STALLED_W {
        RX_STALLED_W { w: self }
    }
    #[doc = "Bit 2 - Transaction Ready Interrupt Status."]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W { w: self }
    }
    #[doc = "Bit 3 - Results Done Interrupt Status."]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W {
        RX_DONE_W { w: self }
    }
    #[doc = "Bit 4 - Transaction FIFO Almost Empty Flag."]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W { w: self }
    }
    #[doc = "Bit 5 - Results FIFO Almost Full Flag."]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W { w: self }
    }
}
