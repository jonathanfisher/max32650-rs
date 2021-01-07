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
#[doc = "Transaction Stalled Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_STALLED_A {
    #[doc = "0: Disable Transaction Stalled Interrupt."]
    EN = 0,
    #[doc = "1: Enable Transaction Stalled Interrupt."]
    DIS = 1,
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
            false => TX_STALLED_A::EN,
            true => TX_STALLED_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_STALLED_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_STALLED_A::DIS
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
    #[doc = "Disable Transaction Stalled Interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_STALLED_A::EN)
    }
    #[doc = "Enable Transaction Stalled Interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_STALLED_A::DIS)
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
#[doc = "Results Stalled Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_STALLED_A {
    #[doc = "0: Disable Results Stalled Interrupt."]
    EN = 0,
    #[doc = "1: Enable Results Stalled Interrupt."]
    DIS = 1,
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
            false => RX_STALLED_A::EN,
            true => RX_STALLED_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_STALLED_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_STALLED_A::DIS
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
    #[doc = "Disable Results Stalled Interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_STALLED_A::EN)
    }
    #[doc = "Enable Results Stalled Interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_STALLED_A::DIS)
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
#[doc = "Transaction Ready Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_READY_A {
    #[doc = "0: Disable FIFO Transaction Ready Interrupt."]
    EN = 0,
    #[doc = "1: Enable FIFO Transaction Ready Interrupt."]
    DIS = 1,
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
            false => TX_READY_A::EN,
            true => TX_READY_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_READY_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_READY_A::DIS
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
    #[doc = "Disable FIFO Transaction Ready Interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_READY_A::EN)
    }
    #[doc = "Enable FIFO Transaction Ready Interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_READY_A::DIS)
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
#[doc = "Results Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DONE_A {
    #[doc = "0: Disable Results Done Interrupt."]
    EN = 0,
    #[doc = "1: Enable Results Done Interrupt."]
    DIS = 1,
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
            false => RX_DONE_A::EN,
            true => RX_DONE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_DONE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_DONE_A::DIS
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
    #[doc = "Disable Results Done Interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_DONE_A::EN)
    }
    #[doc = "Enable Results Done Interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_DONE_A::DIS)
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
#[doc = "Transaction FIFO Almost Empty Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_AE_A {
    #[doc = "0: Disable Transaction FIFO Almost Empty Interrupt."]
    EN = 0,
    #[doc = "1: Enable Transaction FIFO Almost Empty Interrupt."]
    DIS = 1,
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
            false => TX_FIFO_AE_A::EN,
            true => TX_FIFO_AE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_FIFO_AE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_FIFO_AE_A::DIS
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
    #[doc = "Disable Transaction FIFO Almost Empty Interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_FIFO_AE_A::EN)
    }
    #[doc = "Enable Transaction FIFO Almost Empty Interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_FIFO_AE_A::DIS)
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
#[doc = "Results FIFO Almost Full Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_AF_A {
    #[doc = "0: Disable Results FIFO Almost Full Interrupt."]
    EN = 0,
    #[doc = "1: Enable Results FIFO Almost Full Interrupt."]
    DIS = 1,
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
            false => RX_FIFO_AF_A::EN,
            true => RX_FIFO_AF_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_FIFO_AF_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_FIFO_AF_A::DIS
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
    #[doc = "Disable Results FIFO Almost Full Interrupt."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_FIFO_AF_A::EN)
    }
    #[doc = "Enable Results FIFO Almost Full Interrupt."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_FIFO_AF_A::DIS)
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
    #[doc = "Bit 0 - Transaction Stalled Interrupt Enable."]
    #[inline(always)]
    pub fn tx_stalled(&self) -> TX_STALLED_R {
        TX_STALLED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Results Stalled Interrupt Enable."]
    #[inline(always)]
    pub fn rx_stalled(&self) -> RX_STALLED_R {
        RX_STALLED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transaction Ready Interrupt Enable."]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Results Done Interrupt Enable."]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transaction FIFO Almost Empty Interrupt Enable."]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Results FIFO Almost Full Interrupt Enable."]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Stalled Interrupt Enable."]
    #[inline(always)]
    pub fn tx_stalled(&mut self) -> TX_STALLED_W {
        TX_STALLED_W { w: self }
    }
    #[doc = "Bit 1 - Results Stalled Interrupt Enable."]
    #[inline(always)]
    pub fn rx_stalled(&mut self) -> RX_STALLED_W {
        RX_STALLED_W { w: self }
    }
    #[doc = "Bit 2 - Transaction Ready Interrupt Enable."]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W { w: self }
    }
    #[doc = "Bit 3 - Results Done Interrupt Enable."]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W {
        RX_DONE_W { w: self }
    }
    #[doc = "Bit 4 - Transaction FIFO Almost Empty Interrupt Enable."]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W { w: self }
    }
    #[doc = "Bit 5 - Results FIFO Almost Full Interrupt Enable."]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W { w: self }
    }
}
