#[doc = "Reader of register DMA"]
pub type R = crate::R<u32, super::DMA>;
#[doc = "Writer for register DMA"]
pub type W = crate::W<u32, super::DMA>;
#[doc = "Register DMA `reset()`'s with value 0x0007_0007"]
impl crate::ResetValue for super::DMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_0007
    }
}
#[doc = "Transmit FIFO Level. Set the number of free entries in the TxFIFO when a TxDMA request occurs.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_FIFO_LEVEL_A {
    #[doc = "0: `0`"]
    ENTRY1 = 0,
    #[doc = "1: `1`"]
    ENTRIES2 = 1,
    #[doc = "2: `10`"]
    ENTRIES3 = 2,
    #[doc = "3: `11`"]
    ENTRIES4 = 3,
    #[doc = "4: `100`"]
    ENTRIES5 = 4,
    #[doc = "5: `101`"]
    ENTRIES6 = 5,
    #[doc = "6: `110`"]
    ENTRIES7 = 6,
    #[doc = "7: `111`"]
    ENTRIES8 = 7,
}
impl From<TX_FIFO_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FIFO_LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX_FIFO_LEVEL`"]
pub type TX_FIFO_LEVEL_R = crate::R<u8, TX_FIFO_LEVEL_A>;
impl TX_FIFO_LEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_LEVEL_A {
        match self.bits {
            0 => TX_FIFO_LEVEL_A::ENTRY1,
            1 => TX_FIFO_LEVEL_A::ENTRIES2,
            2 => TX_FIFO_LEVEL_A::ENTRIES3,
            3 => TX_FIFO_LEVEL_A::ENTRIES4,
            4 => TX_FIFO_LEVEL_A::ENTRIES5,
            5 => TX_FIFO_LEVEL_A::ENTRIES6,
            6 => TX_FIFO_LEVEL_A::ENTRIES7,
            7 => TX_FIFO_LEVEL_A::ENTRIES8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENTRY1`"]
    #[inline(always)]
    pub fn is_entry1(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRY1
    }
    #[doc = "Checks if the value of the field is `ENTRIES2`"]
    #[inline(always)]
    pub fn is_entries2(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES2
    }
    #[doc = "Checks if the value of the field is `ENTRIES3`"]
    #[inline(always)]
    pub fn is_entries3(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES3
    }
    #[doc = "Checks if the value of the field is `ENTRIES4`"]
    #[inline(always)]
    pub fn is_entries4(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES4
    }
    #[doc = "Checks if the value of the field is `ENTRIES5`"]
    #[inline(always)]
    pub fn is_entries5(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES5
    }
    #[doc = "Checks if the value of the field is `ENTRIES6`"]
    #[inline(always)]
    pub fn is_entries6(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES6
    }
    #[doc = "Checks if the value of the field is `ENTRIES7`"]
    #[inline(always)]
    pub fn is_entries7(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES7
    }
    #[doc = "Checks if the value of the field is `ENTRIES8`"]
    #[inline(always)]
    pub fn is_entries8(&self) -> bool {
        *self == TX_FIFO_LEVEL_A::ENTRIES8
    }
}
#[doc = "Write proxy for field `TX_FIFO_LEVEL`"]
pub struct TX_FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_LEVEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn entry1(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRY1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn entries2(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn entries3(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn entries4(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn entries5(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn entries6(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn entries7(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn entries8(self) -> &'a mut W {
        self.variant(TX_FIFO_LEVEL_A::ENTRIES8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Transmit FIFO Clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_CLEAR_AW {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<TX_FIFO_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TX_FIFO_CLEAR`"]
pub struct TX_FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(TX_FIFO_CLEAR_AW::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TX_FIFO_CLEAR_AW::START)
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
#[doc = "Reader of field `TX_FIFO_CNT`"]
pub type TX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Transmit DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TX_DMA_EN`"]
pub type TX_DMA_EN_R = crate::R<bool, TX_DMA_EN_A>;
impl TX_DMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_EN_A {
        match self.bits {
            false => TX_DMA_EN_A::DISABLE,
            true => TX_DMA_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_DMA_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_DMA_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `TX_DMA_EN`"]
pub struct TX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DMA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_DMA_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Receive FIFO Level. Sets the RX FIFO DMA request threshold. This configures the number of filled RxFIFO entries before activating an RxDMA request.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_FIFO_LEVEL_A {
    #[doc = "0: `0`"]
    ENTRY1 = 0,
    #[doc = "1: `1`"]
    ENTRIES2 = 1,
    #[doc = "2: `10`"]
    ENTRIES3 = 2,
    #[doc = "3: `11`"]
    ENTRIES4 = 3,
    #[doc = "4: `100`"]
    ENTRIES5 = 4,
    #[doc = "5: `101`"]
    ENTRIES6 = 5,
    #[doc = "6: `110`"]
    ENTRIES7 = 6,
    #[doc = "7: `111`"]
    ENTRIES8 = 7,
}
impl From<RX_FIFO_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FIFO_LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_FIFO_LEVEL`"]
pub type RX_FIFO_LEVEL_R = crate::R<u8, RX_FIFO_LEVEL_A>;
impl RX_FIFO_LEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_LEVEL_A {
        match self.bits {
            0 => RX_FIFO_LEVEL_A::ENTRY1,
            1 => RX_FIFO_LEVEL_A::ENTRIES2,
            2 => RX_FIFO_LEVEL_A::ENTRIES3,
            3 => RX_FIFO_LEVEL_A::ENTRIES4,
            4 => RX_FIFO_LEVEL_A::ENTRIES5,
            5 => RX_FIFO_LEVEL_A::ENTRIES6,
            6 => RX_FIFO_LEVEL_A::ENTRIES7,
            7 => RX_FIFO_LEVEL_A::ENTRIES8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENTRY1`"]
    #[inline(always)]
    pub fn is_entry1(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRY1
    }
    #[doc = "Checks if the value of the field is `ENTRIES2`"]
    #[inline(always)]
    pub fn is_entries2(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES2
    }
    #[doc = "Checks if the value of the field is `ENTRIES3`"]
    #[inline(always)]
    pub fn is_entries3(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES3
    }
    #[doc = "Checks if the value of the field is `ENTRIES4`"]
    #[inline(always)]
    pub fn is_entries4(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES4
    }
    #[doc = "Checks if the value of the field is `ENTRIES5`"]
    #[inline(always)]
    pub fn is_entries5(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES5
    }
    #[doc = "Checks if the value of the field is `ENTRIES6`"]
    #[inline(always)]
    pub fn is_entries6(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES6
    }
    #[doc = "Checks if the value of the field is `ENTRIES7`"]
    #[inline(always)]
    pub fn is_entries7(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES7
    }
    #[doc = "Checks if the value of the field is `ENTRIES8`"]
    #[inline(always)]
    pub fn is_entries8(&self) -> bool {
        *self == RX_FIFO_LEVEL_A::ENTRIES8
    }
}
#[doc = "Write proxy for field `RX_FIFO_LEVEL`"]
pub struct RX_FIFO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_LEVEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn entry1(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRY1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn entries2(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn entries3(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn entries4(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn entries5(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn entries6(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn entries7(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn entries8(self) -> &'a mut W {
        self.variant(RX_FIFO_LEVEL_A::ENTRIES8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Receive FIFO Clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_CLEAR_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RX_FIFO_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_FIFO_CLEAR`"]
pub type RX_FIFO_CLEAR_R = crate::R<bool, RX_FIFO_CLEAR_A>;
impl RX_FIFO_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_CLEAR_A {
        match self.bits {
            false => RX_FIFO_CLEAR_A::COMPLETE,
            true => RX_FIFO_CLEAR_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == RX_FIFO_CLEAR_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RX_FIFO_CLEAR_A::START
    }
}
#[doc = "Write proxy for field `RX_FIFO_CLEAR`"]
pub struct RX_FIFO_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(RX_FIFO_CLEAR_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RX_FIFO_CLEAR_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RX_FIFO_CNT`"]
pub type RX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Receive DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DMA_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX_DMA_EN`"]
pub type RX_DMA_EN_R = crate::R<bool, RX_DMA_EN_A>;
impl RX_DMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DMA_EN_A {
        match self.bits {
            false => RX_DMA_EN_A::DISABLE,
            true => RX_DMA_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_DMA_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_DMA_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `RX_DMA_EN`"]
pub struct RX_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DMA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_DMA_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_DMA_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Level. Set the number of free entries in the TxFIFO when a TxDMA request occurs."]
    #[inline(always)]
    pub fn tx_fifo_level(&self) -> TX_FIFO_LEVEL_R {
        TX_FIFO_LEVEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Transmit FIFO Count."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Transmit DMA Enable."]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Receive FIFO Level. Sets the RX FIFO DMA request threshold. This configures the number of filled RxFIFO entries before activating an RxDMA request."]
    #[inline(always)]
    pub fn rx_fifo_level(&self) -> RX_FIFO_LEVEL_R {
        RX_FIFO_LEVEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Receive FIFO Clear."]
    #[inline(always)]
    pub fn rx_fifo_clear(&self) -> RX_FIFO_CLEAR_R {
        RX_FIFO_CLEAR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Receive FIFO Count."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Receive DMA Enable."]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit FIFO Level. Set the number of free entries in the TxFIFO when a TxDMA request occurs."]
    #[inline(always)]
    pub fn tx_fifo_level(&mut self) -> TX_FIFO_LEVEL_W {
        TX_FIFO_LEVEL_W { w: self }
    }
    #[doc = "Bit 4 - Transmit FIFO Clear."]
    #[inline(always)]
    pub fn tx_fifo_clear(&mut self) -> TX_FIFO_CLEAR_W {
        TX_FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bit 15 - Transmit DMA Enable."]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W {
        TX_DMA_EN_W { w: self }
    }
    #[doc = "Bits 16:18 - Receive FIFO Level. Sets the RX FIFO DMA request threshold. This configures the number of filled RxFIFO entries before activating an RxDMA request."]
    #[inline(always)]
    pub fn rx_fifo_level(&mut self) -> RX_FIFO_LEVEL_W {
        RX_FIFO_LEVEL_W { w: self }
    }
    #[doc = "Bit 20 - Receive FIFO Clear."]
    #[inline(always)]
    pub fn rx_fifo_clear(&mut self) -> RX_FIFO_CLEAR_W {
        RX_FIFO_CLEAR_W { w: self }
    }
    #[doc = "Bit 31 - Receive DMA Enable."]
    #[inline(always)]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W {
        RX_DMA_EN_W { w: self }
    }
}
