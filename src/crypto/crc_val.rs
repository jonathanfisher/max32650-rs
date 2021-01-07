#[doc = "Reader of register CRC_VAL"]
pub type R = crate::R<u32, super::CRC_VAL>;
#[doc = "Writer for register CRC_VAL"]
pub type W = crate::W<u32, super::CRC_VAL>;
#[doc = "Register CRC_VAL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CRC_VAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC Value. This is the state for the Galois Field. This register holds the result of a CRC calculation or the current state of LFSR. This register is affected by the MSB control bit."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Value. This is the state for the Galois Field. This register holds the result of a CRC calculation or the current state of LFSR. This register is affected by the MSB control bit."]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
}
