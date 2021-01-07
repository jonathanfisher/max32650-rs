#[doc = "Reader of register INT_IN_IE"]
pub type R = crate::R<u32, super::INT_IN_IE>;
#[doc = "Writer for register INT_IN_IE"]
pub type W = crate::W<u32, super::INT_IN_IE>;
#[doc = "Register INT_IN_IE `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_IN_IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_IN_EN`"]
pub type INT_IN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_IN_EN`"]
pub struct INT_IN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_IN_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt Enable."]
    #[inline(always)]
    pub fn int_in_en(&self) -> INT_IN_EN_R {
        INT_IN_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable."]
    #[inline(always)]
    pub fn int_in_en(&mut self) -> INT_IN_EN_W {
        INT_IN_EN_W { w: self }
    }
}
