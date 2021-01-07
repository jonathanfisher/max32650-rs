#[doc = "Reader of register CRC_POLY"]
pub type R = crate::R<u32, super::CRC_POLY>;
#[doc = "Writer for register CRC_POLY"]
pub type W = crate::W<u32, super::CRC_POLY>;
#[doc = "Register CRC_POLY `reset()`'s with value 0xedb8_8320"]
impl crate::ResetValue for super::CRC_POLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xedb8_8320
    }
}
#[doc = "Reader of field `POLY`"]
pub type POLY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `POLY`"]
pub struct POLY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Polynomial. The polynomial to be used for Galois Field calculations (CRC or LFSR) should be written to this register. This register is affected by the MSB control bit."]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Polynomial. The polynomial to be used for Galois Field calculations (CRC or LFSR) should be written to this register. This register is affected by the MSB control bit."]
    #[inline(always)]
    pub fn poly(&mut self) -> POLY_W {
        POLY_W { w: self }
    }
}
