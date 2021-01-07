#[doc = "Reader of register SEMAPHORES[%s]"]
pub type R = crate::R<u32, super::SEMAPHORES>;
#[doc = "Writer for register SEMAPHORES[%s]"]
pub type W = crate::W<u32, super::SEMAPHORES>;
#[doc = "Register SEMAPHORES[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SEMAPHORES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sema`"]
pub type SEMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sema`"]
pub struct SEMA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sema(&self) -> SEMA_R {
        SEMA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sema(&mut self) -> SEMA_W {
        SEMA_W { w: self }
    }
}
