#[doc = "Reader of register ADMA_ER"]
pub type R = crate::R<u8, super::ADMA_ER>;
#[doc = "Writer for register ADMA_ER"]
pub type W = crate::W<u8, super::ADMA_ER>;
#[doc = "Register ADMA_ER `reset()`'s with value 0"]
impl crate::ResetValue for super::ADMA_ER {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `LEN_MISMATCH`"]
pub type LEN_MISMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEN_MISMATCH`"]
pub struct LEN_MISMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_MISMATCH_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - ADMA Error State."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error."]
    #[inline(always)]
    pub fn len_mismatch(&self) -> LEN_MISMATCH_R {
        LEN_MISMATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMA Error State."]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error."]
    #[inline(always)]
    pub fn len_mismatch(&mut self) -> LEN_MISMATCH_W {
        LEN_MISMATCH_W { w: self }
    }
}
