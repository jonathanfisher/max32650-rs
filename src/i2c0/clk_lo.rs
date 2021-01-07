#[doc = "Reader of register CLK_LO"]
pub type R = crate::R<u32, super::CLK_LO>;
#[doc = "Writer for register CLK_LO"]
pub type W = crate::W<u32, super::CLK_LO>;
#[doc = "Register CLK_LO `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_LO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_LO`"]
pub type CLK_LO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLK_LO`"]
pub struct CLK_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn clk_lo(&self) -> CLK_LO_R {
        CLK_LO_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn clk_lo(&mut self) -> CLK_LO_W {
        CLK_LO_W { w: self }
    }
}
