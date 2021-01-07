#[doc = "Reader of register DMA"]
pub type R = crate::R<u32, super::DMA>;
#[doc = "Writer for register DMA"]
pub type W = crate::W<u32, super::DMA>;
#[doc = "Register DMA `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TX DMA channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMA_EN_A {
    #[doc = "0: DMA is disabled"]
    DIS = 0,
    #[doc = "1: DMA is enabled"]
    EN = 1,
}
impl From<TDMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TDMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDMA_EN`"]
pub type TDMA_EN_R = crate::R<bool, TDMA_EN_A>;
impl TDMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMA_EN_A {
        match self.bits {
            false => TDMA_EN_A::DIS,
            true => TDMA_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TDMA_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TDMA_EN_A::EN
    }
}
#[doc = "Write proxy for field `TDMA_EN`"]
pub struct TDMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TDMA_EN_A::DIS)
    }
    #[doc = "DMA is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TDMA_EN_A::EN)
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
#[doc = "RX DMA channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMA_EN_A {
    #[doc = "0: DMA is disabled"]
    DIS = 0,
    #[doc = "1: DMA is enabled"]
    EN = 1,
}
impl From<RXDMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDMA_EN`"]
pub type RXDMA_EN_R = crate::R<bool, RXDMA_EN_A>;
impl RXDMA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMA_EN_A {
        match self.bits {
            false => RXDMA_EN_A::DIS,
            true => RXDMA_EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RXDMA_EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RXDMA_EN_A::EN
    }
}
#[doc = "Write proxy for field `RXDMA_EN`"]
pub struct RXDMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RXDMA_EN_A::DIS)
    }
    #[doc = "DMA is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RXDMA_EN_A::EN)
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
#[doc = "Reader of field `TXDMA_LEVEL`"]
pub type TXDMA_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDMA_LEVEL`"]
pub struct TXDMA_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RXDMA_LEVEL`"]
pub type RXDMA_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXDMA_LEVEL`"]
pub struct RXDMA_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX DMA channel enable."]
    #[inline(always)]
    pub fn tdma_en(&self) -> TDMA_EN_R {
        TDMA_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX DMA channel enable."]
    #[inline(always)]
    pub fn rxdma_en(&self) -> RXDMA_EN_R {
        RXDMA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - TX threshold for DMA transmission."]
    #[inline(always)]
    pub fn txdma_level(&self) -> TXDMA_LEVEL_R {
        TXDMA_LEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - RX threshold for DMA transmission."]
    #[inline(always)]
    pub fn rxdma_level(&self) -> RXDMA_LEVEL_R {
        RXDMA_LEVEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX DMA channel enable."]
    #[inline(always)]
    pub fn tdma_en(&mut self) -> TDMA_EN_W {
        TDMA_EN_W { w: self }
    }
    #[doc = "Bit 1 - RX DMA channel enable."]
    #[inline(always)]
    pub fn rxdma_en(&mut self) -> RXDMA_EN_W {
        RXDMA_EN_W { w: self }
    }
    #[doc = "Bits 8:13 - TX threshold for DMA transmission."]
    #[inline(always)]
    pub fn txdma_level(&mut self) -> TXDMA_LEVEL_W {
        TXDMA_LEVEL_W { w: self }
    }
    #[doc = "Bits 16:21 - RX threshold for DMA transmission."]
    #[inline(always)]
    pub fn rxdma_level(&mut self) -> RXDMA_LEVEL_W {
        RXDMA_LEVEL_W { w: self }
    }
}
