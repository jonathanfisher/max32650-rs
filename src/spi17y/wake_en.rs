#[doc = "Reader of register WAKE_EN"]
pub type R = crate::R<u32, super::WAKE_EN>;
#[doc = "Writer for register WAKE_EN"]
pub type W = crate::W<u32, super::WAKE_EN>;
#[doc = "Register WAKE_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKE_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wake on TX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_THRESH_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
}
impl From<TX_THRESH_A> for bool {
    #[inline(always)]
    fn from(variant: TX_THRESH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_THRESH`"]
pub type TX_THRESH_R = crate::R<bool, TX_THRESH_A>;
impl TX_THRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_THRESH_A {
        match self.bits {
            false => TX_THRESH_A::DIS,
            true => TX_THRESH_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_THRESH_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_THRESH_A::EN
    }
}
#[doc = "Write proxy for field `TX_THRESH`"]
pub struct TX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_THRESH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_THRESH_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_THRESH_A::EN)
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
#[doc = "Wake on TX FIFO Empty Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EMPTY_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
}
impl From<TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_EMPTY`"]
pub type TX_EMPTY_R = crate::R<bool, TX_EMPTY_A>;
impl TX_EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EMPTY_A {
        match self.bits {
            false => TX_EMPTY_A::DIS,
            true => TX_EMPTY_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TX_EMPTY_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TX_EMPTY_A::EN
    }
}
#[doc = "Write proxy for field `TX_EMPTY`"]
pub struct TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EMPTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_EMPTY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TX_EMPTY_A::EN)
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
#[doc = "Wake on RX FIFO Threshold Crossed Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_THRESH_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
}
impl From<RX_THRESH_A> for bool {
    #[inline(always)]
    fn from(variant: RX_THRESH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_THRESH`"]
pub type RX_THRESH_R = crate::R<bool, RX_THRESH_A>;
impl RX_THRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_THRESH_A {
        match self.bits {
            false => RX_THRESH_A::DIS,
            true => RX_THRESH_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_THRESH_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_THRESH_A::EN
    }
}
#[doc = "Write proxy for field `RX_THRESH`"]
pub struct RX_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_THRESH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_THRESH_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_THRESH_A::EN)
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
#[doc = "Wake on RX FIFO Full Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FULL_A {
    #[doc = "0: Wakeup source disabled."]
    DIS = 0,
    #[doc = "1: Wakeup source enabled."]
    EN = 1,
}
impl From<RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_FULL`"]
pub type RX_FULL_R = crate::R<bool, RX_FULL_A>;
impl RX_FULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FULL_A {
        match self.bits {
            false => RX_FULL_A::DIS,
            true => RX_FULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RX_FULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RX_FULL_A::EN
    }
}
#[doc = "Write proxy for field `RX_FULL`"]
pub struct RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup source disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RX_FULL_A::DIS)
    }
    #[doc = "Wakeup source enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RX_FULL_A::EN)
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
impl R {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn tx_thresh(&self) -> TX_THRESH_R {
        TX_THRESH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn rx_thresh(&self) -> RX_THRESH_R {
        RX_THRESH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake on TX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn tx_thresh(&mut self) -> TX_THRESH_W {
        TX_THRESH_W { w: self }
    }
    #[doc = "Bit 1 - Wake on TX FIFO Empty Enable."]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - Wake on RX FIFO Threshold Crossed Enable."]
    #[inline(always)]
    pub fn rx_thresh(&mut self) -> RX_THRESH_W {
        RX_THRESH_W { w: self }
    }
    #[doc = "Bit 3 - Wake on RX FIFO Full Enable."]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RX_FULL_W {
        RX_FULL_W { w: self }
    }
}
