#[doc = "Reader of register INT_EN"]
pub type R = crate::R<u16, super::INT_EN>;
#[doc = "Writer for register INT_EN"]
pub type W = crate::W<u16, super::INT_EN>;
#[doc = "Register INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMD_COMP`"]
pub type CMD_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_COMP`"]
pub struct CMD_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TRANS_COMP`"]
pub type TRANS_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_COMP`"]
pub struct TRANS_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `BLK_GAP`"]
pub type BLK_GAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLK_GAP`"]
pub struct BLK_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_GAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BUFFER_WR`"]
pub type BUFFER_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFER_WR`"]
pub struct BUFFER_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BUFFER_RD`"]
pub type BUFFER_RD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFER_RD`"]
pub struct BUFFER_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFER_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CARD_INSERT`"]
pub type CARD_INSERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INSERT`"]
pub struct CARD_INSERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INSERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CARD_REMOVAL`"]
pub type CARD_REMOVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_REMOVAL`"]
pub struct CARD_REMOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_REMOVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CARD_INT`"]
pub type CARD_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INT`"]
pub struct CARD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RETUNING`"]
pub type RETUNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETUNING`"]
pub struct RETUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> RETUNING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable."]
    #[inline(always)]
    pub fn cmd_comp(&self) -> CMD_COMP_R {
        CMD_COMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable."]
    #[inline(always)]
    pub fn trans_comp(&self) -> TRANS_COMP_R {
        TRANS_COMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable."]
    #[inline(always)]
    pub fn blk_gap(&self) -> BLK_GAP_R {
        BLK_GAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_wr(&self) -> BUFFER_WR_R {
        BUFFER_WR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_rd(&self) -> BUFFER_RD_R {
        BUFFER_RD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable."]
    #[inline(always)]
    pub fn card_insert(&self) -> CARD_INSERT_R {
        CARD_INSERT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable."]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable."]
    #[inline(always)]
    pub fn card_int(&self) -> CARD_INT_R {
        CARD_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable."]
    #[inline(always)]
    pub fn retuning(&self) -> RETUNING_R {
        RETUNING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable."]
    #[inline(always)]
    pub fn cmd_comp(&mut self) -> CMD_COMP_W {
        CMD_COMP_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable."]
    #[inline(always)]
    pub fn trans_comp(&mut self) -> TRANS_COMP_W {
        TRANS_COMP_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable."]
    #[inline(always)]
    pub fn blk_gap(&mut self) -> BLK_GAP_W {
        BLK_GAP_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_wr(&mut self) -> BUFFER_WR_W {
        BUFFER_WR_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_rd(&mut self) -> BUFFER_RD_W {
        BUFFER_RD_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable."]
    #[inline(always)]
    pub fn card_insert(&mut self) -> CARD_INSERT_W {
        CARD_INSERT_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable."]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W {
        CARD_REMOVAL_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable."]
    #[inline(always)]
    pub fn card_int(&mut self) -> CARD_INT_W {
        CARD_INT_W { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable."]
    #[inline(always)]
    pub fn retuning(&mut self) -> RETUNING_W {
        RETUNING_W { w: self }
    }
}
