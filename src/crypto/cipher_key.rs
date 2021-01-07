#[doc = "Writer for register CIPHER_KEY[%s]"]
pub type W = crate::W<u32, super::CIPHER_KEY>;
#[doc = "Register CIPHER_KEY[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CIPHER_KEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Cipher Key. This register holds the key used for block cipher operations. The lower words are used for block ciphers that use shorter kye lengths. This register is affected by the endian swap input control bits."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
