#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `done_ie`"]
pub type DONE_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `done_ie`"]
pub struct DONE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_IE_W<'a> {
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
#[doc = "Reader of field `ref_ready_ie`"]
pub type REF_READY_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ref_ready_ie`"]
pub struct REF_READY_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_READY_IE_W<'a> {
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
#[doc = "Reader of field `hi_limit_ie`"]
pub type HI_LIMIT_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hi_limit_ie`"]
pub struct HI_LIMIT_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> HI_LIMIT_IE_W<'a> {
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
#[doc = "Reader of field `lo_limit_ie`"]
pub type LO_LIMIT_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_limit_ie`"]
pub struct LO_LIMIT_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LIMIT_IE_W<'a> {
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
#[doc = "Reader of field `overflow_ie`"]
pub type OVERFLOW_IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `overflow_ie`"]
pub struct OVERFLOW_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_IE_W<'a> {
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
#[doc = "Reader of field `done_if`"]
pub type DONE_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `done_if`"]
pub struct DONE_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ref_ready_if`"]
pub type REF_READY_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ref_ready_if`"]
pub struct REF_READY_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_READY_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `hi_limit_if`"]
pub type HI_LIMIT_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hi_limit_if`"]
pub struct HI_LIMIT_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> HI_LIMIT_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `lo_limit_if`"]
pub type LO_LIMIT_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lo_limit_if`"]
pub struct LO_LIMIT_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LIMIT_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `overflow_if`"]
pub type OVERFLOW_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `overflow_if`"]
pub struct OVERFLOW_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_IF_W<'a> {
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
#[doc = "Reader of field `pending`"]
pub type PENDING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn done_ie(&self) -> DONE_IE_R {
        DONE_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ref_ready_ie(&self) -> REF_READY_IE_R {
        REF_READY_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn hi_limit_ie(&self) -> HI_LIMIT_IE_R {
        HI_LIMIT_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn lo_limit_ie(&self) -> LO_LIMIT_IE_R {
        LO_LIMIT_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow_ie(&self) -> OVERFLOW_IE_R {
        OVERFLOW_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn done_if(&self) -> DONE_IF_R {
        DONE_IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ref_ready_if(&self) -> REF_READY_IF_R {
        REF_READY_IF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn hi_limit_if(&self) -> HI_LIMIT_IF_R {
        HI_LIMIT_IF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn lo_limit_if(&self) -> LO_LIMIT_IF_R {
        LO_LIMIT_IF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn overflow_if(&self) -> OVERFLOW_IF_R {
        OVERFLOW_IF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC Interrupt Pending Status"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn done_ie(&mut self) -> DONE_IE_W {
        DONE_IE_W { w: self }
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ref_ready_ie(&mut self) -> REF_READY_IE_W {
        REF_READY_IE_W { w: self }
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn hi_limit_ie(&mut self) -> HI_LIMIT_IE_W {
        HI_LIMIT_IE_W { w: self }
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn lo_limit_ie(&mut self) -> LO_LIMIT_IE_W {
        LO_LIMIT_IE_W { w: self }
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow_ie(&mut self) -> OVERFLOW_IE_W {
        OVERFLOW_IE_W { w: self }
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn done_if(&mut self) -> DONE_IF_W {
        DONE_IF_W { w: self }
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ref_ready_if(&mut self) -> REF_READY_IF_W {
        REF_READY_IF_W { w: self }
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn hi_limit_if(&mut self) -> HI_LIMIT_IF_W {
        HI_LIMIT_IF_W { w: self }
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn lo_limit_if(&mut self) -> LO_LIMIT_IF_W {
        LO_LIMIT_IF_W { w: self }
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn overflow_if(&mut self) -> OVERFLOW_IF_W {
        OVERFLOW_IF_W { w: self }
    }
}
