#[doc = "Reader of register REG3"]
pub type R = crate::R<u32, super::REG3>;
#[doc = "Writer for register REG3"]
pub type W = crate::W<u32, super::REG3>;
#[doc = "Register REG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::REG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DONECNT`"]
pub type DONECNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DONECNT`"]
pub struct DONECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DONECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    pub fn donecnt(&self) -> DONECNT_R {
        DONECNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    pub fn donecnt(&mut self) -> DONECNT_W {
        DONECNT_W { w: self }
    }
}
