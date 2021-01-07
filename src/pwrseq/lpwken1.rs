#[doc = "Reader of register LPWKEN1"]
pub type R = crate::R<u32, super::LPWKEN1>;
#[doc = "Writer for register LPWKEN1"]
pub type W = crate::W<u32, super::LPWKEN1>;
#[doc = "Register LPWKEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LPWKEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKEEN`"]
pub type WAKEEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WAKEEN`"]
pub struct WAKEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin(s) on transition(s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&self) -> WAKEEN_R {
        WAKEEN_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin(s) on transition(s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WAKEEN_W {
        WAKEEN_W { w: self }
    }
}
