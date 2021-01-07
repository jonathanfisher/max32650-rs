#[doc = "Reader of register MXM_REG_A4"]
pub type R = crate::R<u32, super::MXM_REG_A4>;
#[doc = "Writer for register MXM_REG_A4"]
pub type W = crate::W<u32, super::MXM_REG_A4>;
#[doc = "Register MXM_REG_A4 `reset()`'s with value 0"]
impl crate::ResetValue for super::MXM_REG_A4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VRST_VDDB_N_A`"]
pub type VRST_VDDB_N_A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VRST_VDDB_N_A`"]
pub struct VRST_VDDB_N_A_W<'a> {
    w: &'a mut W,
}
impl<'a> VRST_VDDB_N_A_W<'a> {
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
#[doc = "Reader of field `DMA_INT`"]
pub type DMA_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_INT`"]
pub struct DMA_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - VRST_VDDB_N_A"]
    #[inline(always)]
    pub fn vrst_vddb_n_a(&self) -> VRST_VDDB_N_A_R {
        VRST_VDDB_N_A_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA_INT"]
    #[inline(always)]
    pub fn dma_int(&self) -> DMA_INT_R {
        DMA_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VRST_VDDB_N_A"]
    #[inline(always)]
    pub fn vrst_vddb_n_a(&mut self) -> VRST_VDDB_N_A_W {
        VRST_VDDB_N_A_W { w: self }
    }
    #[doc = "Bit 1 - DMA_INT"]
    #[inline(always)]
    pub fn dma_int(&mut self) -> DMA_INT_W {
        DMA_INT_W { w: self }
    }
}
