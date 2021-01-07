#[doc = "Writer for register ACNTL"]
pub type W = crate::W<u32, super::ACNTL>;
#[doc = "Register ACNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ACNTL`"]
pub struct ACNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACNTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn acntl(&mut self) -> ACNTL_W {
        ACNTL_W { w: self }
    }
}
