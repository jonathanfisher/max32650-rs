#[doc = "Reader of register FIFO_CTRL"]
pub type R = crate::R<u32, super::FIFO_CTRL>;
#[doc = "Writer for register FIFO_CTRL"]
pub type W = crate::W<u32, super::FIFO_CTRL>;
#[doc = "Register FIFO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_FIFO_AE_LVL`"]
pub type TX_FIFO_AE_LVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_FIFO_AE_LVL`"]
pub struct TX_FIFO_AE_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_AE_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TX_FIFO_CNT`"]
pub type TX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_FIFO_CNT`"]
pub struct TX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RX_FIFO_AF_LVL`"]
pub type RX_FIFO_AF_LVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FIFO_AF_LVL`"]
pub struct RX_FIFO_AF_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_AF_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RX_FIFO_CNT`"]
pub type RX_FIFO_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FIFO_CNT`"]
pub struct RX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transaction FIFO Almost Empty Level."]
    #[inline(always)]
    pub fn tx_fifo_ae_lvl(&self) -> TX_FIFO_AE_LVL_R {
        TX_FIFO_AE_LVL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Transaction FIFO Used."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Results FIFO Almost Full Level."]
    #[inline(always)]
    pub fn rx_fifo_af_lvl(&self) -> RX_FIFO_AF_LVL_R {
        RX_FIFO_AF_LVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - Result FIFO Used."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transaction FIFO Almost Empty Level."]
    #[inline(always)]
    pub fn tx_fifo_ae_lvl(&mut self) -> TX_FIFO_AE_LVL_W {
        TX_FIFO_AE_LVL_W { w: self }
    }
    #[doc = "Bits 8:12 - Transaction FIFO Used."]
    #[inline(always)]
    pub fn tx_fifo_cnt(&mut self) -> TX_FIFO_CNT_W {
        TX_FIFO_CNT_W { w: self }
    }
    #[doc = "Bits 16:20 - Results FIFO Almost Full Level."]
    #[inline(always)]
    pub fn rx_fifo_af_lvl(&mut self) -> RX_FIFO_AF_LVL_W {
        RX_FIFO_AF_LVL_W { w: self }
    }
    #[doc = "Bits 24:29 - Result FIFO Used."]
    #[inline(always)]
    pub fn rx_fifo_cnt(&mut self) -> RX_FIFO_CNT_W {
        RX_FIFO_CNT_W { w: self }
    }
}
