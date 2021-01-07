#[doc = "Reader of register OUT"]
pub type R = crate::R<u32, super::OUT>;
#[doc = "Writer for register OUT"]
pub type W = crate::W<u32, super::OUT>;
#[doc = "Register OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_OUT_A {
    #[doc = "0: Drive Logic 0 (low) on GPIO output."]
    LOW = 0,
    #[doc = "1: Drive logic 1 (high) on GPIO output."]
    HIGH = 1,
}
impl From<GPIO_OUT_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_OUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_OUT`"]
pub type GPIO_OUT_R = crate::R<u32, GPIO_OUT_A>;
impl GPIO_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_OUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_OUT_A::LOW),
            1 => Val(GPIO_OUT_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == GPIO_OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == GPIO_OUT_A::HIGH
    }
}
#[doc = "Write proxy for field `GPIO_OUT`"]
pub struct GPIO_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_OUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Drive Logic 0 (low) on GPIO output."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(GPIO_OUT_A::LOW)
    }
    #[doc = "Drive logic 1 (high) on GPIO output."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(GPIO_OUT_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out(&self) -> GPIO_OUT_R {
        GPIO_OUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out(&mut self) -> GPIO_OUT_W {
        GPIO_OUT_W { w: self }
    }
}
