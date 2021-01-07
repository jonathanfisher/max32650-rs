#[doc = "Reader of register SSEC"]
pub type R = crate::R<u32, super::SSEC>;
#[doc = "Writer for register SSEC"]
pub type W = crate::W<u32, super::SSEC>;
#[doc = "Register SSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::SSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTSS`"]
pub type RTSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTSS`"]
pub struct RTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RTC Sub-second Counter."]
    #[inline(always)]
    pub fn rtss(&self) -> RTSS_R {
        RTSS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC Sub-second Counter."]
    #[inline(always)]
    pub fn rtss(&mut self) -> RTSS_W {
        RTSS_W { w: self }
    }
}
