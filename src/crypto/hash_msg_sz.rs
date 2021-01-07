#[doc = "Reader of register HASH_MSG_SZ[%s]"]
pub type R = crate::R<u32, super::HASH_MSG_SZ>;
#[doc = "Writer for register HASH_MSG_SZ[%s]"]
pub type W = crate::W<u32, super::HASH_MSG_SZ>;
#[doc = "Register HASH_MSG_SZ[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_MSG_SZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSGSZ`"]
pub type MSGSZ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MSGSZ`"]
pub struct MSGSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Size. This register holds the lowest 32-bit of message size in bytes."]
    #[inline(always)]
    pub fn msgsz(&self) -> MSGSZ_R {
        MSGSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Size. This register holds the lowest 32-bit of message size in bytes."]
    #[inline(always)]
    pub fn msgsz(&mut self) -> MSGSZ_W {
        MSGSZ_W { w: self }
    }
}
