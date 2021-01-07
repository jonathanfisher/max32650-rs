#[doc = "Reader of register CLK_HI"]
pub type R = crate::R<u32, super::CLK_HI>;
#[doc = "Writer for register CLK_HI"]
pub type W = crate::W<u32, super::CLK_HI>;
#[doc = "Register CLK_HI `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_HI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKH`"]
pub type CKH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CKH`"]
pub struct CKH_W<'a> {
    w: &'a mut W,
}
impl<'a> CKH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn ckh(&self) -> CKH_R {
        CKH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn ckh(&mut self) -> CKH_W {
        CKH_W { w: self }
    }
}
