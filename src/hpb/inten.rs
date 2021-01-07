#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERRINTE`"]
pub type ERRINTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRINTE`"]
pub struct ERRINTE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINTE_W<'a> {
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
    #[doc = "Bit 1 - Error Interrupt Enable."]
    #[inline(always)]
    pub fn errinte(&self) -> ERRINTE_R {
        ERRINTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Error Interrupt Enable."]
    #[inline(always)]
    pub fn errinte(&mut self) -> ERRINTE_W {
        ERRINTE_W { w: self }
    }
}
