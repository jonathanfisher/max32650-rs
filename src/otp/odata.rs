#[doc = "Writer for register ODATA"]
pub type W = crate::W<u32, super::ODATA>;
#[doc = "Register ODATA `reset()`'s with value 0"]
impl crate::ResetValue for super::ODATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OTPDATA`"]
pub struct OTPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP write data."]
    #[inline(always)]
    pub fn otpdata(&mut self) -> OTPDATA_W {
        OTPDATA_W { w: self }
    }
}
