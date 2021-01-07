#[doc = "Reader of register CLK_DIV_1US"]
pub type R = crate::R<u32, super::CLK_DIV_1US>;
#[doc = "Writer for register CLK_DIV_1US"]
pub type W = crate::W<u32, super::CLK_DIV_1US>;
#[doc = "Register CLK_DIV_1US `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_DIV_1US {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `divisor`"]
pub type DIVISOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `divisor`"]
pub struct DIVISOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVISOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Divisor for 1Mhz."]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divisor for 1Mhz."]
    #[inline(always)]
    pub fn divisor(&mut self) -> DIVISOR_W {
        DIVISOR_W { w: self }
    }
}
