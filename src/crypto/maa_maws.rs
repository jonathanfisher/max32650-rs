#[doc = "Reader of register MAA_MAWS"]
pub type R = crate::R<u32, super::MAA_MAWS>;
#[doc = "Writer for register MAA_MAWS"]
pub type W = crate::W<u32, super::MAA_MAWS>;
#[doc = "Register MAA_MAWS `reset()`'s with value 0"]
impl crate::ResetValue for super::MAA_MAWS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAWS`"]
pub type MAWS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAWS`"]
pub struct MAWS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - MAA Word Size."]
    #[inline(always)]
    pub fn maws(&self) -> MAWS_R {
        MAWS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MAA Word Size."]
    #[inline(always)]
    pub fn maws(&mut self) -> MAWS_W {
        MAWS_W { w: self }
    }
}
