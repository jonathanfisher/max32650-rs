#[doc = "Reader of register EARLYDMA"]
pub type R = crate::R<u8, super::EARLYDMA>;
#[doc = "Writer for register EARLYDMA"]
pub type W = crate::W<u8, super::EARLYDMA>;
#[doc = "Register EARLYDMA `reset()`'s with value 0"]
impl crate::ResetValue for super::EARLYDMA {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDMAIN`"]
pub type EDMAIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDMAIN`"]
pub struct EDMAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDMAIN_W<'a> {
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
#[doc = "Reader of field `EDMAOUT`"]
pub type EDMAOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDMAOUT`"]
pub struct EDMAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDMAOUT_W<'a> {
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
    pub fn edmain(&self) -> EDMAIN_R {
        EDMAIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn edmaout(&self) -> EDMAOUT_R {
        EDMAOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn edmain(&mut self) -> EDMAIN_W {
        EDMAIN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn edmaout(&mut self) -> EDMAOUT_W {
        EDMAOUT_W { w: self }
    }
}
