#[doc = "Reader of register INT_EN"]
pub type R = crate::R<u32, super::INT_EN>;
#[doc = "Writer for register INT_EN"]
pub type W = crate::W<u32, super::INT_EN>;
#[doc = "Register INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UFLO`"]
pub type UFLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UFLO`"]
pub struct UFLO_W<'a> {
    w: &'a mut W,
}
impl<'a> UFLO_W<'a> {
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
#[doc = "Reader of field `ADRRDY`"]
pub type ADRRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRRDY`"]
pub struct ADRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRRDY_W<'a> {
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
#[doc = "Reader of field `VCI`"]
pub type VCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCI`"]
pub struct VCI_W<'a> {
    w: &'a mut W,
}
impl<'a> VCI_W<'a> {
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
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERR`"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
    #[doc = "Bit 0 - Under FLow Interupt Enable"]
    #[inline(always)]
    pub fn uflo(&self) -> UFLO_R {
        UFLO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address Ready Interupt Enable"]
    #[inline(always)]
    pub fn adrrdy(&self) -> ADRRDY_R {
        ADRRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VCI Interupt Enable"]
    #[inline(always)]
    pub fn vci(&self) -> VCI_R {
        VCI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BERR Interupt Enable"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Under FLow Interupt Enable"]
    #[inline(always)]
    pub fn uflo(&mut self) -> UFLO_W {
        UFLO_W { w: self }
    }
    #[doc = "Bit 1 - Address Ready Interupt Enable"]
    #[inline(always)]
    pub fn adrrdy(&mut self) -> ADRRDY_W {
        ADRRDY_W { w: self }
    }
    #[doc = "Bit 2 - VCI Interupt Enable"]
    #[inline(always)]
    pub fn vci(&mut self) -> VCI_W {
        VCI_W { w: self }
    }
    #[doc = "Bit 3 - BERR Interupt Enable"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
}
