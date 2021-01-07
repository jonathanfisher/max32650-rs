#[doc = "Reader of register HOST_CN_1"]
pub type R = crate::R<u8, super::HOST_CN_1>;
#[doc = "Writer for register HOST_CN_1"]
pub type W = crate::W<u8, super::HOST_CN_1>;
#[doc = "Register HOST_CN_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_CN_1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LED_CN`"]
pub type LED_CN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LED_CN`"]
pub struct LED_CN_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_CN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DATA_TRANSFER_WIDTH`"]
pub type DATA_TRANSFER_WIDTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_TRANSFER_WIDTH`"]
pub struct DATA_TRANSFER_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TRANSFER_WIDTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HS_EN`"]
pub type HS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HS_EN`"]
pub struct HS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA_SELECT`"]
pub type DMA_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_SELECT`"]
pub struct DMA_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `EXT_DATA_TRANSFER_WIDTH`"]
pub type EXT_DATA_TRANSFER_WIDTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT_DATA_TRANSFER_WIDTH`"]
pub struct EXT_DATA_TRANSFER_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_DATA_TRANSFER_WIDTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CARD_DETECT_TEST`"]
pub type CARD_DETECT_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_DETECT_TEST`"]
pub struct CARD_DETECT_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CARD_DETECT_SIGNAL`"]
pub type CARD_DETECT_SIGNAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_DETECT_SIGNAL`"]
pub struct CARD_DETECT_SIGNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_SIGNAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LED Control."]
    #[inline(always)]
    pub fn led_cn(&self) -> LED_CN_R {
        LED_CN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width."]
    #[inline(always)]
    pub fn data_transfer_width(&self) -> DATA_TRANSFER_WIDTH_R {
        DATA_TRANSFER_WIDTH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable."]
    #[inline(always)]
    pub fn hs_en(&self) -> HS_EN_R {
        HS_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select."]
    #[inline(always)]
    pub fn dma_select(&self) -> DMA_SELECT_R {
        DMA_SELECT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width."]
    #[inline(always)]
    pub fn ext_data_transfer_width(&self) -> EXT_DATA_TRANSFER_WIDTH_R {
        EXT_DATA_TRANSFER_WIDTH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level."]
    #[inline(always)]
    pub fn card_detect_test(&self) -> CARD_DETECT_TEST_R {
        CARD_DETECT_TEST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection."]
    #[inline(always)]
    pub fn card_detect_signal(&self) -> CARD_DETECT_SIGNAL_R {
        CARD_DETECT_SIGNAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control."]
    #[inline(always)]
    pub fn led_cn(&mut self) -> LED_CN_W {
        LED_CN_W { w: self }
    }
    #[doc = "Bit 1 - Data Transfer Width."]
    #[inline(always)]
    pub fn data_transfer_width(&mut self) -> DATA_TRANSFER_WIDTH_W {
        DATA_TRANSFER_WIDTH_W { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable."]
    #[inline(always)]
    pub fn hs_en(&mut self) -> HS_EN_W {
        HS_EN_W { w: self }
    }
    #[doc = "Bits 3:4 - DMA Select."]
    #[inline(always)]
    pub fn dma_select(&mut self) -> DMA_SELECT_W {
        DMA_SELECT_W { w: self }
    }
    #[doc = "Bit 5 - Extended Data Transfer Width."]
    #[inline(always)]
    pub fn ext_data_transfer_width(&mut self) -> EXT_DATA_TRANSFER_WIDTH_W {
        EXT_DATA_TRANSFER_WIDTH_W { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level."]
    #[inline(always)]
    pub fn card_detect_test(&mut self) -> CARD_DETECT_TEST_W {
        CARD_DETECT_TEST_W { w: self }
    }
    #[doc = "Bit 7 - Card Detect Signal Selection."]
    #[inline(always)]
    pub fn card_detect_signal(&mut self) -> CARD_DETECT_SIGNAL_W {
        CARD_DETECT_SIGNAL_W { w: self }
    }
}
