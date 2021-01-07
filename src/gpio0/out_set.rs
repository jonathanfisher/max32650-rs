#[doc = "Writer for register OUT_SET"]
pub type W = crate::W<u32, super::OUT_SET>;
#[doc = "Register OUT_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_OUT_SET_AW {
    #[doc = "0: No Effect."]
    NO = 0,
    #[doc = "1: Set GPIO_OUT bit in this position to '1'"]
    SET = 1,
}
impl From<GPIO_OUT_SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `GPIO_OUT_SET`"]
pub struct GPIO_OUT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_OUT_SET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(GPIO_OUT_SET_AW::NO)
    }
    #[doc = "Set GPIO_OUT bit in this position to '1'"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(GPIO_OUT_SET_AW::SET)
    }
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
    pub fn gpio_out_set(&mut self) -> GPIO_OUT_SET_W {
        GPIO_OUT_SET_W { w: self }
    }
}
