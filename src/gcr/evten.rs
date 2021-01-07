#[doc = "Reader of register EVTEN"]
pub type R = crate::R<u32, super::EVTEN>;
#[doc = "Writer for register EVTEN"]
pub type W = crate::W<u32, super::EVTEN>;
#[doc = "Register EVTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAEVENT`"]
pub type DMAEVENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEVENT`"]
pub struct DMAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEVENT_W<'a> {
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
#[doc = "Reader of field `RXEVENT`"]
pub type RXEVENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXEVENT`"]
pub struct RXEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEVENT_W<'a> {
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
#[doc = "Reader of field `TXEVENT`"]
pub type TXEVENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEVENT`"]
pub struct TXEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEVENT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dmaevent(&self) -> DMAEVENT_R {
        DMAEVENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO0\\[24\\]
will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rxevent(&self) -> RXEVENT_R {
        RXEVENT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO\\[25\\]."]
    #[inline(always)]
    pub fn txevent(&self) -> TXEVENT_R {
        TXEVENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA event. When this bit is set, a DMA event will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn dmaevent(&mut self) -> DMAEVENT_W {
        DMAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - Enable RXEV pin event. When this bit is set, a logic high of GPIO0\\[24\\]
will cause an RXEV event to wake the CPU from WFE sleep mode."]
    #[inline(always)]
    pub fn rxevent(&mut self) -> RXEVENT_W {
        RXEVENT_W { w: self }
    }
    #[doc = "Bit 2 - Enable TXEV pin event. When this bit is set, TXEV event from the CPU is output to GPIO\\[25\\]."]
    #[inline(always)]
    pub fn txevent(&mut self) -> TXEVENT_W {
        TXEVENT_W { w: self }
    }
}
