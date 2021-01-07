#[doc = "Writer for register OUT_CLR"]
pub type W = crate::W<u32, super::OUT_CLR>;
#[doc = "Register OUT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `GPIO_OUT_CLR`"]
pub struct GPIO_OUT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out_clr(&mut self) -> GPIO_OUT_CLR_W {
        GPIO_OUT_CLR_W { w: self }
    }
}
