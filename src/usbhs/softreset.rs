#[doc = "Reader of register SOFTRESET"]
pub type R = crate::R<u8, super::SOFTRESET>;
#[doc = "Writer for register SOFTRESET"]
pub type W = crate::W<u8, super::SOFTRESET>;
#[doc = "Register SOFTRESET `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTRESET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTXS`"]
pub type RSTXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTXS`"]
pub struct RSTXS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTXS_W<'a> {
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
#[doc = "Reader of field `RSTS`"]
pub type RSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTS`"]
pub struct RSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTS_W<'a> {
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
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstxs(&self) -> RSTXS_R {
        RSTXS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsts(&self) -> RSTS_R {
        RSTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstxs(&mut self) -> RSTXS_W {
        RSTXS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsts(&mut self) -> RSTS_W {
        RSTS_W { w: self }
    }
}
