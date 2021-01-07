#[doc = "Reader of register INT_STAT"]
pub type R = crate::R<u16, super::INT_STAT>;
#[doc = "Writer for register INT_STAT"]
pub type W = crate::W<u16, super::INT_STAT>;
#[doc = "Register INT_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STAT {
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
#[doc = "Reader of field `BLK_GAP_EVENT`"]
pub type BLK_GAP_EVENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLK_GAP_EVENT`"]
pub struct BLK_GAP_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_GAP_EVENT_W<'a> {
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
#[doc = "Reader of field `BUFF_WR_READY`"]
pub type BUFF_WR_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFF_WR_READY`"]
pub struct BUFF_WR_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_WR_READY_W<'a> {
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
#[doc = "Reader of field `BUFF_RD_READY`"]
pub type BUFF_RD_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFF_RD_READY`"]
pub struct BUFF_RD_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_RD_READY_W<'a> {
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
#[doc = "Reader of field `CARD_INSERTION`"]
pub type CARD_INSERTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INSERTION`"]
pub struct CARD_INSERTION_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INSERTION_W<'a> {
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
#[doc = "Reader of field `CARD_INTR`"]
pub type CARD_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_INTR`"]
pub struct CARD_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INTR_W<'a> {
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
#[doc = "Reader of field `ERR_INTR`"]
pub type ERR_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR_INTR`"]
pub struct ERR_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete."]
    #[inline(always)]
    pub fn cmd_comp(&self) -> CMD_COMP_R {
        CMD_COMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete."]
    #[inline(always)]
    pub fn trans_comp(&self) -> TRANS_COMP_R {
        TRANS_COMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event."]
    #[inline(always)]
    pub fn blk_gap_event(&self) -> BLK_GAP_EVENT_R {
        BLK_GAP_EVENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready."]
    #[inline(always)]
    pub fn buff_wr_ready(&self) -> BUFF_WR_READY_R {
        BUFF_WR_READY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready."]
    #[inline(always)]
    pub fn buff_rd_ready(&self) -> BUFF_RD_READY_R {
        BUFF_RD_READY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion."]
    #[inline(always)]
    pub fn card_insertion(&self) -> CARD_INSERTION_R {
        CARD_INSERTION_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal."]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt."]
    #[inline(always)]
    pub fn card_intr(&self) -> CARD_INTR_R {
        CARD_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event."]
    #[inline(always)]
    pub fn retuning(&self) -> RETUNING_R {
        RETUNING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt."]
    #[inline(always)]
    pub fn err_intr(&self) -> ERR_INTR_R {
        ERR_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete."]
    #[inline(always)]
    pub fn cmd_comp(&mut self) -> CMD_COMP_W {
        CMD_COMP_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete."]
    #[inline(always)]
    pub fn trans_comp(&mut self) -> TRANS_COMP_W {
        TRANS_COMP_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event."]
    #[inline(always)]
    pub fn blk_gap_event(&mut self) -> BLK_GAP_EVENT_W {
        BLK_GAP_EVENT_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready."]
    #[inline(always)]
    pub fn buff_wr_ready(&mut self) -> BUFF_WR_READY_W {
        BUFF_WR_READY_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready."]
    #[inline(always)]
    pub fn buff_rd_ready(&mut self) -> BUFF_RD_READY_W {
        BUFF_RD_READY_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion."]
    #[inline(always)]
    pub fn card_insertion(&mut self) -> CARD_INSERTION_W {
        CARD_INSERTION_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal."]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W {
        CARD_REMOVAL_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt."]
    #[inline(always)]
    pub fn card_intr(&mut self) -> CARD_INTR_W {
        CARD_INTR_W { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event."]
    #[inline(always)]
    pub fn retuning(&mut self) -> RETUNING_W {
        RETUNING_W { w: self }
    }
    #[doc = "Bit 15 - Error Interrupt."]
    #[inline(always)]
    pub fn err_intr(&mut self) -> ERR_INTR_W {
        ERR_INTR_W { w: self }
    }
}
