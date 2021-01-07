#[doc = "Reader of register CIPHER_INIT[%s]"]
pub type R = crate::R<u32, super::CIPHER_INIT>;
#[doc = "Writer for register CIPHER_INIT[%s]"]
pub type W = crate::W<u32, super::CIPHER_INIT>;
#[doc = "Register CIPHER_INIT[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CIPHER_INIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IVEC`"]
pub type IVEC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IVEC`"]
pub struct IVEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IVEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Initial Vector. For block cipher operations that use CBC, CFB, OFB, or CNTR modes, this register holds the initial value. This register is updated with each encryption or decryption operation. This register is affected by the endian swap bits."]
    #[inline(always)]
    pub fn ivec(&self) -> IVEC_R {
        IVEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Vector. For block cipher operations that use CBC, CFB, OFB, or CNTR modes, this register holds the initial value. This register is updated with each encryption or decryption operation. This register is affected by the endian swap bits."]
    #[inline(always)]
    pub fn ivec(&mut self) -> IVEC_W {
        IVEC_W { w: self }
    }
}
